Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

# [unreleased]

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
