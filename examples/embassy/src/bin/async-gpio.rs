//! This example demonstrates the usage of async GPIO operations on VA416xx.
//!
//! You need to tie the PA0 to the PA1 pin for this example to work. You can optionally also tie
//! more pin combinations together and test other ports by setting the appropriate
//! [CHECK_XXX_TO_XXX] constants to true.
#![no_std]
#![no_main]
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use embassy_sync::channel::{Receiver, Sender};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::{Duration, Instant, Timer};
use embedded_hal_async::digital::Wait;
use va416xx_hal::clock::ClkgenExt;
use va416xx_hal::gpio::{
    on_interrupt_for_async_gpio_for_port, InputDynPinAsync, InputPinAsync, PinsB, PinsC, PinsD,
    PinsE, PinsF, PinsG, Port,
};
use va416xx_hal::time::Hertz;
use va416xx_hal::{
    gpio::{DynPin, PinsA},
    pac::{self, interrupt},
};

const CHECK_PA0_TO_PA1: bool = true;
const CHECK_PB0_TO_PB1: bool = true;
const CHECK_PC14_TO_PC15: bool = true;
const CHECK_PD2_TO_PD3: bool = true;
const CHECK_PE0_TO_PE1: bool = true;
const CHECK_PF0_TO_PF1: bool = true;

#[derive(Clone, Copy)]
pub struct GpioCmd {
    cmd_type: GpioCmdType,
    after_delay: u32,
}

impl GpioCmd {
    pub fn new(cmd_type: GpioCmdType, after_delay: u32) -> Self {
        Self {
            cmd_type,
            after_delay,
        }
    }
}

#[derive(Clone, Copy)]
pub enum GpioCmdType {
    SetHigh,
    SetLow,
    RisingEdge,
    FallingEdge,
    CloseTask,
}

// Declare a bounded channel of 3 u32s.
static CHANNEL_PA0_TO_PA1: Channel<ThreadModeRawMutex, GpioCmd, 3> = Channel::new();
static CHANNEL_PB0_TO_PB1: Channel<ThreadModeRawMutex, GpioCmd, 3> = Channel::new();
static CHANNEL_PC14_TO_PC15: Channel<ThreadModeRawMutex, GpioCmd, 3> = Channel::new();
static CHANNEL_PD2_TO_PD3: Channel<ThreadModeRawMutex, GpioCmd, 3> = Channel::new();
static CHANNEL_PE0_TO_PE1: Channel<ThreadModeRawMutex, GpioCmd, 3> = Channel::new();
static CHANNEL_PF0_TO_PF1: Channel<ThreadModeRawMutex, GpioCmd, 3> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::println!("-- VA416xx Async GPIO Demo --");

    let mut dp = pac::Peripherals::take().unwrap();

    // Initialize the systick interrupt & obtain the token to prove that we did
    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
        .freeze(&mut dp.sysconfig)
        .unwrap();
    // Safety: Only called once here.
    unsafe {
        va416xx_embassy::init(
            &mut dp.sysconfig,
            &dp.irq_router,
            dp.tim15,
            dp.tim14,
            &clocks,
        )
    };

    let porta = PinsA::new(&mut dp.sysconfig, dp.porta);
    let portb = PinsB::new(&mut dp.sysconfig, dp.portb);
    let portc = PinsC::new(&mut dp.sysconfig, dp.portc);
    let portd = PinsD::new(&mut dp.sysconfig, dp.portd);
    let porte = PinsE::new(&mut dp.sysconfig, dp.porte);
    let portf = PinsF::new(&mut dp.sysconfig, dp.portf);

    let portg = PinsG::new(&mut dp.sysconfig, dp.portg);
    let mut led = portg.pg5.into_readable_push_pull_output();

    if CHECK_PA0_TO_PA1 {
        let out_pin = porta.pa0.into_readable_push_pull_output();
        let in_pin = porta.pa1.into_floating_input();
        let out_pin = out_pin.downgrade();
        let in_pin = InputPinAsync::new(in_pin).unwrap();

        spawner
            .spawn(output_task(
                "PA0 to PA1",
                out_pin,
                CHANNEL_PA0_TO_PA1.receiver(),
            ))
            .unwrap();
        check_pin_to_pin_async_ops("PA0 to PA1", CHANNEL_PA0_TO_PA1.sender(), in_pin).await;
        defmt::info!("Example PA0 to PA1 done");
    }

    if CHECK_PB0_TO_PB1 {
        let out_pin = portb.pb0.into_readable_push_pull_output();
        let in_pin = portb.pb1.into_floating_input();
        let out_pin = out_pin.downgrade();
        let in_pin = InputDynPinAsync::new(in_pin.downgrade()).unwrap();

        spawner
            .spawn(output_task(
                "PB0 to PB1",
                out_pin,
                CHANNEL_PB0_TO_PB1.receiver(),
            ))
            .unwrap();
        check_pin_to_pin_async_ops("PB0 to PB1", CHANNEL_PB0_TO_PB1.sender(), in_pin).await;
        defmt::info!("Example PB0 to PB1 done");
    }

    if CHECK_PC14_TO_PC15 {
        let out_pin = portc.pc14.into_readable_push_pull_output();
        let in_pin = portc.pc15.into_floating_input();
        let out_pin = out_pin.downgrade();
        let in_pin = InputDynPinAsync::new(in_pin.downgrade()).unwrap();
        spawner
            .spawn(output_task(
                "PC14 to PC15",
                out_pin,
                CHANNEL_PC14_TO_PC15.receiver(),
            ))
            .unwrap();
        check_pin_to_pin_async_ops("PC14 to PC15", CHANNEL_PC14_TO_PC15.sender(), in_pin).await;
        defmt::info!("Example PC14 to PC15 done");
    }

    if CHECK_PD2_TO_PD3 {
        let out_pin = portd.pd2.into_readable_push_pull_output();
        let in_pin = portd.pd3.into_floating_input();
        let out_pin = out_pin.downgrade();
        let in_pin = InputDynPinAsync::new(in_pin.downgrade()).unwrap();
        spawner
            .spawn(output_task(
                "PD2 to PD3",
                out_pin,
                CHANNEL_PD2_TO_PD3.receiver(),
            ))
            .unwrap();
        check_pin_to_pin_async_ops("PD2 to PD3", CHANNEL_PD2_TO_PD3.sender(), in_pin).await;
        defmt::info!("Example PD2 to PD3 done");
    }

    if CHECK_PE0_TO_PE1 {
        let out_pin = porte.pe0.into_readable_push_pull_output();
        let in_pin = porte.pe1.into_floating_input();
        let out_pin = out_pin.downgrade();
        let in_pin = InputDynPinAsync::new(in_pin.downgrade()).unwrap();
        spawner
            .spawn(output_task(
                "PE0 to PE1",
                out_pin,
                CHANNEL_PE0_TO_PE1.receiver(),
            ))
            .unwrap();
        check_pin_to_pin_async_ops("PE0 to PE1", CHANNEL_PE0_TO_PE1.sender(), in_pin).await;
        defmt::info!("Example PE0 to PE1 done");
    }

    if CHECK_PF0_TO_PF1 {
        let out_pin = portf.pf0.into_readable_push_pull_output();
        let in_pin = portf.pf1.into_floating_input();
        let out_pin = out_pin.downgrade();
        let in_pin = InputDynPinAsync::new(in_pin.downgrade()).unwrap();
        spawner
            .spawn(output_task(
                "PF0 to PF1",
                out_pin,
                CHANNEL_PF0_TO_PF1.receiver(),
            ))
            .unwrap();
        check_pin_to_pin_async_ops("PF0 to PF1", CHANNEL_PF0_TO_PF1.sender(), in_pin).await;
        defmt::info!("Example PF0 to PF1 done");
    }

    defmt::info!("Example done, toggling LED0");
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await;
    }
}

async fn check_pin_to_pin_async_ops(
    ctx: &'static str,
    sender: Sender<'static, ThreadModeRawMutex, GpioCmd, 3>,
    mut async_input: impl Wait,
) {
    defmt::info!(
        "{}: sending SetHigh command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender.send(GpioCmd::new(GpioCmdType::SetHigh, 20)).await;
    async_input.wait_for_high().await.unwrap();
    defmt::info!(
        "{}: Input pin is high now ({} ms)",
        ctx,
        Instant::now().as_millis()
    );

    defmt::info!(
        "{}: sending SetLow command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender.send(GpioCmd::new(GpioCmdType::SetLow, 20)).await;
    async_input.wait_for_low().await.unwrap();
    defmt::info!(
        "{}: Input pin is low now ({} ms)",
        ctx,
        Instant::now().as_millis()
    );

    defmt::info!(
        "{}: sending RisingEdge command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender.send(GpioCmd::new(GpioCmdType::RisingEdge, 20)).await;
    async_input.wait_for_rising_edge().await.unwrap();
    defmt::info!(
        "{}: input pin had rising edge ({} ms)",
        ctx,
        Instant::now().as_millis()
    );

    defmt::info!(
        "{}: sending Falling command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender
        .send(GpioCmd::new(GpioCmdType::FallingEdge, 20))
        .await;
    async_input.wait_for_falling_edge().await.unwrap();
    defmt::info!(
        "{}: input pin had a falling edge ({} ms)",
        ctx,
        Instant::now().as_millis()
    );

    defmt::info!(
        "{}: sending Falling command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender
        .send(GpioCmd::new(GpioCmdType::FallingEdge, 20))
        .await;
    async_input.wait_for_any_edge().await.unwrap();
    defmt::info!(
        "{}: input pin had a falling (any) edge ({} ms)",
        ctx,
        Instant::now().as_millis()
    );

    defmt::info!(
        "{}: sending Falling command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender.send(GpioCmd::new(GpioCmdType::RisingEdge, 20)).await;
    async_input.wait_for_any_edge().await.unwrap();
    defmt::info!(
        "{}: input pin had a rising (any) edge ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender.send(GpioCmd::new(GpioCmdType::CloseTask, 0)).await;
}

#[embassy_executor::task(pool_size = 8)]
async fn output_task(
    ctx: &'static str,
    mut out: DynPin,
    receiver: Receiver<'static, ThreadModeRawMutex, GpioCmd, 3>,
) {
    loop {
        let next_cmd = receiver.receive().await;
        Timer::after(Duration::from_millis(next_cmd.after_delay.into())).await;
        match next_cmd.cmd_type {
            GpioCmdType::SetHigh => {
                defmt::info!("{}: Set output high", ctx);
                out.set_high().unwrap();
            }
            GpioCmdType::SetLow => {
                defmt::info!("{}: Set output low", ctx);
                out.set_low().unwrap();
            }
            GpioCmdType::RisingEdge => {
                defmt::info!("{}: Rising edge", ctx);
                if !out.is_low().unwrap() {
                    out.set_low().unwrap();
                }
                out.set_high().unwrap();
            }
            GpioCmdType::FallingEdge => {
                defmt::info!("{}: Falling edge", ctx);
                if !out.is_high().unwrap() {
                    out.set_high().unwrap();
                }
                out.set_low().unwrap();
            }
            GpioCmdType::CloseTask => {
                defmt::info!("{}: Closing task", ctx);
                break;
            }
        }
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn PORTA1() {
    on_interrupt_for_async_gpio_for_port(Port::A).unwrap();
}

#[interrupt]
#[allow(non_snake_case)]
fn PORTB1() {
    on_interrupt_for_async_gpio_for_port(Port::B).unwrap();
}

#[interrupt]
#[allow(non_snake_case)]
fn PORTC15() {
    on_interrupt_for_async_gpio_for_port(Port::C).unwrap();
}

#[interrupt]
#[allow(non_snake_case)]
fn PORTD3() {
    on_interrupt_for_async_gpio_for_port(Port::D).unwrap();
}

#[interrupt]
#[allow(non_snake_case)]
fn PORTE1() {
    on_interrupt_for_async_gpio_for_port(Port::E).unwrap();
}

#[interrupt]
#[allow(non_snake_case)]
fn PORTF1() {
    on_interrupt_for_async_gpio_for_port(Port::F).unwrap();
}
