Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

# [unreleased]

# [v0.4.0]

## Changed

- GPIO API: Interrupt, pulse and filter and `set_datamask` and `clear_datamask` APIs are now
  methods which mutable modify the pin instead of consuming and returning it.
- Simplified PWM module implementation.
- All error types now implement `core::error::Error` by using the `thiserror::Error` derive.
- `InvalidPinTypeError` now wraps the pin mode.
- I2C `TimingCfg` constructor now returns explicit error instead of generic Error.
  Removed the timing configuration error type from the generic I2C error enumeration.
- `PinsA` and `PinsB` constructor do not expect an optional `pac::Ioconfig` argument anymore.
- `IrqCfg` renamed to `InterruptConfig`, kept alias for old name.
- All library provided interrupt handlers now start with common prefix `on_interrupt_*`
- `RxWithIrq` renamed to `RxWithInterrupt`
- `Rx::into_rx_with_irq` does not expect any arguments any more.
- `filter_type` renamed to `configure_filter_type`.
- `level_irq` renamed to `configure_level_interrupt`.
- `edge_irq` renamed to `configure_edge_interrupt`.
- UART interrupt management is now handled by the main constructor instead of later stages to
  statically ensure one interrupt vector for the UART peripheral. `Uart::new` expects an
  optional `InterruptConfig` argument.
- `enable_interrupt` and `disable_interrupt` renamed to `enable_nvic_interrupt` and
  `disable_nvic_interrupt` to distinguish them from peripheral interrupts more clearly.
- `port_mux` renamed to `port_function_select`
- Renamed `IrqUartErrors` to `UartErrors`.

## Added

- Add `downgrade` method for `Pin` and `upgrade` method for `DynPin` as explicit conversion
  methods.
- Asynchronous GPIO support.
- Asynchronous UART TX support.
- Asynchronous UART RX support.
- Add new `get_tim_raw` unsafe method to retrieve TIM peripheral blocks.
- `Uart::with_with_interrupt` and `Uart::new_without_interrupt`

# [v0.3.0] 2024-30-09

## Changed

- Improve and fix SPI abstractions. Add new low level interface. The primary SPI constructor now
  only expects a configuration structure and the transfer configuration needs to be applied in a
  separate step.
- Added an additional way to read the UART RX with IRQs. The module documentation provides
  more information.
- Made the UART with IRQ API more flexible for future additions.
- Improved UART API result and error handling, added low level API to read from and write
  to the FIFO directly

## Fixed

- Fixes for SPI peripheral: Flush implementation was incorrect and should now flush properly.
- Fixes for SPI example
- Fixes for RTIC example

# [v0.2.0] 2024-09-18

- Documentation improvements
- Improved UART typing support: Validity of passed pins is now checked properly

## Changed

- Added `va41620`, `va41630`, `va41628` and `va41629` device features. A device now has to be
  selected for HAL compilation to work properly
- Adaptions for the UART IRQ feature which are now only implemented for the RX part of the UART.

## Fixed

- Small fixes and improvements for ADC drivers
- Fixes for the SPI implementation where the clock divider values were not calculated
  correctly
- Fixes for UART IRQ handler implementation
- Add new IRQ router initialization method `irq_router::enable_and_init_irq_router`. This method
  also sets the initial values of some registers to 0 where the datasheet and the actual reset
  value are inconsistent, which can lead to weird bugs like IRQs not being triggered properly.

## Added

- Added basic DMA driver
- Added basic EDAC module
- Added bootloader and flashloader example application
- Added NVM module which exposes a simple API to write to the NVM memory used for the boot process

# [v0.1.0] 2024-07-01

- Initial release with basic HAL drivers
