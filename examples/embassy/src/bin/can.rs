#![no_std]
#![no_main]

// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use va416xx_hal::can::asynch::on_interrupt_can;
use va416xx_hal::can::{Can, CanFrame, CanFrameNormal, CanId, CanRx, CanTx, ClockConfig};
use va416xx_hal::clock::ClockConfigurator;
use va416xx_hal::pac::{self, interrupt};
use va416xx_hal::time::Hertz;
use va416xx_hal::{can, prelude::*};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::println!("-- VA416xx CAN Demo --");

    let dp = pac::Peripherals::take().unwrap();

    // Initialize the systick interrupt & obtain the token to prove that we did
    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
        .freeze()
        .unwrap();
    // Safety: Only called once here.
    va416xx_embassy::init(dp.tim15, dp.tim14, &clocks);
    let clk_config = ClockConfig::from_bitrate_and_segments(&clocks, 250.kHz(), 16, 4, 4).unwrap();
    let mut can = Can::new(dp.can0, clk_config);
    can.set_loopback(true);
    can.set_bufflock(true);
    can.set_base_mask_for_all_match();
    can.enable();
    let mut channels = can.take_channels().unwrap();
    // Transmit channel.
    let mut tx = CanTx::new(channels.take(0).unwrap(), None);
    // Base channel which has dedicated mask.
    let mut rx = CanRx::new(channels.take(14).unwrap());
    let send_frame = CanFrame::Normal(CanFrameNormal::new(
        can::Id::Standard(can::StandardId::new(0x1).unwrap()),
        &[1, 2, 3, 4],
    ));
    defmt::info!("sending CAN frame");
    tx.transmit_frame(send_frame).unwrap();
    let _frame = nb::block!(rx.receive(true)).expect("invalid CAN rx state");
    defmt::info!("received CAN frame with data");
}

#[interrupt]
#[allow(non_snake_case)]
fn CAN0() {
    on_interrupt_can(CanId::Can0, false).unwrap();
}
