#!/usr/bin/env python3
from spacepackets.ecss.defs import PusService
import toml
import logging
import argparse
import time
from tmtccmd.com.serial_base import SerialCfg
from tmtccmd.com.serial_cobs import SerialCobsComIF
from tmtccmd.com.ser_utils import prompt_com_port
from spacepackets.ecss.tc import PusTc
from pathlib import Path


BAUD_RATE = 115200
BOOTLOADER_START_ADDR = 0x0
BOOTLOADER_END_ADDR = 0x4000
BOOTLOADER_CRC_ADDR = 0x3FFE
APP_A_START_ADDR = 0x4000
APP_A_END_ADDR = 0x22000
# The actual size of the image which is relevant for CRC calculation.
APP_A_SIZE_ADDR = 0x21FF8
APP_A_CRC_ADDR = 0x21FFC
APP_B_START_ADDR = 0x22000
APP_B_END_ADDR = 0x40000
# The actual size of the image which is relevant for CRC calculation.
APP_B_SIZE_ADDR = 0x3FFF8
APP_B_CRC_ADDR = 0x3FFFC
APP_IMG_SZ = 0x1E000

_LOGGER = logging.getLogger(__name__)


def main():
    print("Python VA416XX Image Loader Application")
    logging.basicConfig(
        format="[%(asctime)s] [%(levelname)s] %(message)s", level=logging.DEBUG
    )
    parser = argparse.ArgumentParser(
        prog="image-loader", description="Python VA416XX Image Loader Application"
    )
    parser.add_argument("-p", "--ping", action="store_true", help="Send ping command")
    parser.add_argument(
        "-f",
        "--flash",
        choices=["bl", "a", "b"],
        help="Flash target (Bootloader or slot A or B)",
    )
    parser.add_argument(
        "path", nargs="?", default=None, help="Path to the App to flash"
    )
    args = parser.parse_args()
    serial_port = None
    if Path("loader.toml").exists():
        with open("loader.toml", "r") as toml_file:
            parsed_toml = toml.loads(toml_file.read())
            if "serial_port" in parsed_toml:
                serial_port = parsed_toml["serial_port"]
    if serial_port is None:
        serial_port = prompt_com_port()
    serial_cfg = SerialCfg(
        com_if_id="ser_cobs",
        serial_port=serial_port,
        baud_rate=BAUD_RATE,
        serial_timeout=0.1,
    )
    com_if = SerialCobsComIF(serial_cfg)
    com_if.open()
    if args.flash and not args.path:
        _LOGGER.error("App Path needs to be specified for the flash process")
    ping_tc = PusTc(apid=0x00, service=PusService.S17_TEST, subservice=1)
    if args.ping:
        _LOGGER.info("Sending ping command")
        com_if.send(ping_tc.pack())
    if args.flash:
        while True:
            data_available = com_if.data_available(0.4)
            if data_available:
                reply = com_if.receive()
                # TODO: Parse replies
                print("Received replies: {}", reply)
                break
    com_if.close()


if __name__ == "__main__":
    main()
