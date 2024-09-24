//! This is a sample time driver implementation for the VA416xx family of devices, supporting
//! one alarm and requiring/reserving 2 TIM peripherals. You could adapt this implementation to
//! support more alarms.
use core::{
    cell::Cell,
    mem, ptr,
    sync::atomic::{AtomicU32, AtomicU8, Ordering},
};
use critical_section::CriticalSection;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;

use embassy_time_driver::{time_driver_impl, AlarmHandle, Driver, TICK_HZ};
use once_cell::sync::OnceCell;
use va416xx_hal::{
    clock::Clocks,
    enable_interrupt,
    irq_router::enable_and_init_irq_router,
    pac::{self, interrupt},
    timer::{assert_tim_reset_for_two_cycles, enable_tim_clk, ValidTim},
};

pub type TimekeeperClk = pac::Tim15;
pub type AlarmClk0 = pac::Tim14;
pub type AlarmClk1 = pac::Tim13;
pub type AlarmClk2 = pac::Tim12;

/// Initialization method for embassy
///
/// # Safety
/// This has to be called once at initialization time to initiate the time driver for
/// embassy.
pub unsafe fn init(
    syscfg: &mut pac::Sysconfig,
    irq_router: &pac::IrqRouter,
    timekeeper: TimekeeperClk,
    alarm: AlarmClk0,
    clocks: &Clocks,
) {
    enable_and_init_irq_router(syscfg, irq_router);
    DRIVER.init(syscfg, timekeeper, alarm, clocks)
}

const fn alarm_tim(idx: usize) -> &'static pac::tim0::RegisterBlock {
    // Safety: This is a static memory-mapped peripheral.
    match idx {
        0 => unsafe { &*AlarmClk0::ptr() },
        1 => unsafe { &*AlarmClk1::ptr() },
        2 => unsafe { &*AlarmClk2::ptr() },
        _ => {
            panic!("invalid alarm timer index")
        }
    }
}

const fn timekeeping_tim() -> &'static pac::tim0::RegisterBlock {
    // Safety: This is a memory-mapped peripheral.
    unsafe { &*TimekeeperClk::ptr() }
}

struct AlarmState {
    timestamp: Cell<u64>,

    // This is really a Option<(fn(*mut ()), *mut ())>
    // but fn pointers aren't allowed in const yet
    callback: Cell<*const ()>,
    ctx: Cell<*mut ()>,
}

impl AlarmState {
    const fn new() -> Self {
        Self {
            timestamp: Cell::new(u64::MAX),
            callback: Cell::new(ptr::null()),
            ctx: Cell::new(ptr::null_mut()),
        }
    }
}

unsafe impl Send for AlarmState {}

const ALARM_COUNT: usize = 1;

static SCALE: OnceCell<u64> = OnceCell::new();

pub struct TimerDriverEmbassy {
    periods: AtomicU32,
    alarm_count: AtomicU8,
    /// Timestamp at which to fire alarm. u64::MAX if no alarm is scheduled.
    alarms: Mutex<CriticalSectionRawMutex, [AlarmState; ALARM_COUNT]>,
}

impl TimerDriverEmbassy {
    fn init(
        &self,
        syscfg: &mut pac::Sysconfig,
        timekeeper: TimekeeperClk,
        alarm_tim: AlarmClk0,
        clocks: &Clocks,
    ) {
        enable_tim_clk(syscfg, TimekeeperClk::TIM_ID);
        assert_tim_reset_for_two_cycles(syscfg, TimekeeperClk::TIM_ID);

        // Initiate scale value here. This is required to convert timer ticks back to a timestamp.
        SCALE
            .set((TimekeeperClk::clock(clocks).raw() / TICK_HZ as u32) as u64)
            .unwrap();
        timekeeper
            .rst_value()
            .write(|w| unsafe { w.bits(u32::MAX) });
        // Decrementing counter.
        timekeeper
            .cnt_value()
            .write(|w| unsafe { w.bits(u32::MAX) });
        // Switch on. Timekeeping should always be done.
        unsafe {
            enable_interrupt(TimekeeperClk::IRQ);
        }
        timekeeper.ctrl().modify(|_, w| w.irq_enb().set_bit());
        timekeeper.enable().write(|w| unsafe { w.bits(1) });

        enable_tim_clk(syscfg, AlarmClk0::TIM_ID);
        assert_tim_reset_for_two_cycles(syscfg, AlarmClk0::TIM_ID);

        // Explicitely disable alarm timer until needed.
        alarm_tim.ctrl().modify(|_, w| {
            w.irq_enb().clear_bit();
            w.enable().clear_bit()
        });
        // Enable general interrupts. The IRQ enable of the peripheral remains cleared.
        unsafe {
            enable_interrupt(AlarmClk0::IRQ);
        }
    }

    // Should be called inside the IRQ of the timekeeper timer.
    fn on_interrupt_timekeeping(&self) {
        self.next_period();
    }

    // Should be called inside the IRQ of the alarm timer.
    fn on_interrupt_alarm(&self, idx: usize) {
        critical_section::with(|cs| {
            if self.alarms.borrow(cs)[idx].timestamp.get() <= self.now() {
                self.trigger_alarm(idx, cs)
            }
        })
    }

    fn next_period(&self) {
        let period = self.periods.fetch_add(1, Ordering::AcqRel) + 1;
        let t = (period as u64) << 32;
        critical_section::with(|cs| {
            for i in 0..ALARM_COUNT {
                let alarm = &self.alarms.borrow(cs)[i];
                let at = alarm.timestamp.get();
                let alarm_tim = alarm_tim(0);
                if at < t {
                    self.trigger_alarm(i, cs);
                } else {
                    let remaining_ticks = (at - t) * *SCALE.get().unwrap();
                    if remaining_ticks <= u32::MAX as u64 {
                        alarm_tim.enable().write(|w| unsafe { w.bits(0) });
                        alarm_tim
                            .cnt_value()
                            .write(|w| unsafe { w.bits(remaining_ticks as u32) });
                        alarm_tim.ctrl().modify(|_, w| w.irq_enb().set_bit());
                        alarm_tim.enable().write(|w| unsafe { w.bits(1) })
                    }
                }
            }
        })
    }

    fn get_alarm<'a>(&'a self, cs: CriticalSection<'a>, alarm: AlarmHandle) -> &'a AlarmState {
        // safety: we're allowed to assume the AlarmState is created by us, and
        // we never create one that's out of bounds.
        unsafe { self.alarms.borrow(cs).get_unchecked(alarm.id() as usize) }
    }

    fn trigger_alarm(&self, n: usize, cs: CriticalSection) {
        alarm_tim(n).ctrl().modify(|_, w| {
            w.irq_enb().clear_bit();
            w.enable().clear_bit()
        });

        let alarm = &self.alarms.borrow(cs)[n];
        // Setting the maximum value disables the alarm.
        alarm.timestamp.set(u64::MAX);

        // Call after clearing alarm, so the callback can set another alarm.

        // safety:
        // - we can ignore the possiblity of `f` being unset (null) because of the safety contract of `allocate_alarm`.
        // - other than that we only store valid function pointers into alarm.callback
        let f: fn(*mut ()) = unsafe { mem::transmute(alarm.callback.get()) };
        f(alarm.ctx.get());
    }
}

impl Driver for TimerDriverEmbassy {
    fn now(&self) -> u64 {
        if SCALE.get().is_none() {
            return 0;
        }
        let mut period1: u32;
        let mut period2: u32;
        let mut counter_val: u32;

        loop {
            // Acquire ensures that we get the latest value of `periods` and
            // no instructions can be reordered before the load.
            period1 = self.periods.load(Ordering::Acquire);

            counter_val = u32::MAX - timekeeping_tim().cnt_value().read().bits();

            // Double read to protect against race conditions when the counter is overflowing.
            period2 = self.periods.load(Ordering::Relaxed);
            if period1 == period2 {
                let now = (((period1 as u64) << 32) | counter_val as u64) / *SCALE.get().unwrap();
                return now;
            }
        }
    }

    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        let id = self
            .alarm_count
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |x| {
                if x < ALARM_COUNT as u8 {
                    Some(x + 1)
                } else {
                    None
                }
            });

        match id {
            Ok(id) => Some(AlarmHandle::new(id)),
            Err(_) => None,
        }
    }

    fn set_alarm_callback(
        &self,
        alarm: embassy_time_driver::AlarmHandle,
        callback: fn(*mut ()),
        ctx: *mut (),
    ) {
        critical_section::with(|cs| {
            let alarm = self.get_alarm(cs, alarm);

            alarm.callback.set(callback as *const ());
            alarm.ctx.set(ctx);
        })
    }

    fn set_alarm(&self, alarm: embassy_time_driver::AlarmHandle, timestamp: u64) -> bool {
        if SCALE.get().is_none() {
            return false;
        }
        critical_section::with(|cs| {
            let n = alarm.id();
            let alarm_tim = alarm_tim(n.into());
            alarm_tim.ctrl().modify(|_, w| {
                w.irq_enb().clear_bit();
                w.enable().clear_bit()
            });

            let alarm = self.get_alarm(cs, alarm);
            alarm.timestamp.set(timestamp);

            let t = self.now();
            if timestamp <= t {
                alarm.timestamp.set(u64::MAX);
                return false;
            }

            // If it hasn't triggered yet, setup the relevant reset value, regardless of whether
            // the interrupts are enabled or not. When they are enabled at a later point, the
            // right value is already set.

            // If the timestamp is in the next few ticks, add a bit of buffer to be sure the alarm
            // is not missed.
            //
            // This means that an alarm can be delayed for up to 2 ticks (from t+1 to t+3), but this is allowed
            // by the Alarm trait contract. What's not allowed is triggering alarms *before* their scheduled time,
            // and we don't do that here.
            let safe_timestamp = timestamp.max(t + 3);
            let timer_ticks = (safe_timestamp - t) * *SCALE.get().unwrap();
            alarm_tim.rst_value().write(|w| unsafe { w.bits(u32::MAX) });
            if timer_ticks <= u32::MAX as u64 {
                alarm_tim
                    .cnt_value()
                    .write(|w| unsafe { w.bits(timer_ticks as u32) });
                alarm_tim.ctrl().modify(|_, w| w.irq_enb().set_bit());
                alarm_tim.enable().write(|w| unsafe { w.bits(1) });
            }
            // If it's too far in the future, don't enable timer yet.
            // It will be enabled later by `next_period`.

            true
        })
    }
}

time_driver_impl!(
    static DRIVER: TimerDriverEmbassy = TimerDriverEmbassy {
        periods: AtomicU32::new(0),
        alarm_count: AtomicU8::new(0),
        alarms: Mutex::const_new(CriticalSectionRawMutex::new(), [AlarmState::new(); ALARM_COUNT])
});

#[interrupt]
#[allow(non_snake_case)]
fn TIM15() {
    DRIVER.on_interrupt_timekeeping()
}

#[interrupt]
#[allow(non_snake_case)]
fn TIM14() {
    DRIVER.on_interrupt_alarm(0)
}
