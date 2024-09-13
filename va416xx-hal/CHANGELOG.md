Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

# [unreleased]

# [v0.2.0]

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

## Added

- Added basic DMA driver
- Added basic EDAC module
- Added bootloader and flashloader example application
- Added NVM module which exposes a simple API to write to the NVM memory used for the boot process

# [v0.1.0] 2024-07-01

- Initial release with basic HAL drivers
