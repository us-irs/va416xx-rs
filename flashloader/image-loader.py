#!/usr/bin/env python3
from dataclasses import dataclass
from spacepackets.ecss.defs import PusService
import toml
import struct
import logging
import argparse
from typing import List
import time
from tmtccmd.com.serial_base import SerialCfg
from tmtccmd.com.serial_cobs import SerialCobsComIF
from tmtccmd.com.ser_utils import prompt_com_port
from spacepackets.ecss.tc import PusTc
from pathlib import Path
import dataclasses
from elftools.elf.elffile import ELFFile


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

CHUNK_SIZE = 896

MEMORY_SERVICE = 6
RAW_MEMORY_WRITE_SUBSERVICE = 2
BOOT_NVM_MEMORY_ID = 1

_LOGGER = logging.getLogger(__name__)


@dataclasses.dataclass
class LoadableSegment:
    name: str
    offset: int
    size: int
    data: bytes


def main() -> int:
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
    file_path = None
    if args.flash:
        if not args.path:
            _LOGGER.error("App Path needs to be specified for the flash process")
            return -1
        file_path = Path(args.path)
        if not file_path.exists():
            _LOGGER.error("File does not exist")
            return -1
    ping_tc = PusTc(apid=0x00, service=PusService.S17_TEST, subservice=1)
    if args.ping:
        _LOGGER.info("Sending ping command")
        com_if.send(ping_tc.pack())
    if args.flash:
        assert file_path is not None
        loadable_segments = []
        _LOGGER.info("Parsing ELF file for loadable sections")
        with open(file_path, "rb") as app_file:
            elf_file = ELFFile(app_file)

            for segment in elf_file.iter_segments("PT_LOAD"):
                if segment.header.p_filesz == 0:
                    continue
                name = None
                for section in elf_file.iter_sections():
                    if (
                        section.header.sh_offset == segment.header.p_offset
                        and section.header.sh_size > 0
                    ):
                        name = section.name
                if name is None:
                    _LOGGER.warning("no fitting section found for segment")
                    continue
                # print(f"Segment Addr: {segment.header.p_paddr}")
                # print(f"Segment Offset: {segment.header.p_offset}")
                # print(f"Segment Filesize: {segment.header.p_filesz}")
                loadable_segments.append(
                    LoadableSegment(
                        name=name,
                        offset=segment.header.p_paddr,
                        size=segment.header.p_filesz,
                        data=segment.data(),
                    )
                )
            for idx, segment in enumerate(loadable_segments):
                _LOGGER.info(
                    f"Loadable section {idx} {segment.name} with offset {segment.offset} and size {segment.size}"
                )
            for segment in loadable_segments:
                current_addr = segment.offset
                while current_addr < segment.offset + segment.size:
                    next_chunk_size = segment.offset + segment.size - current_addr
                    if next_chunk_size > CHUNK_SIZE:
                        next_chunk_size = CHUNK_SIZE
                    app_data = bytearray()
                    app_data.append(BOOT_NVM_MEMORY_ID)
                    # N parameter is always 1 here.
                    app_data.append(1)
                    app_data.extend(struct.pack("!I", current_addr))
                    app_data.extend(struct.pack("!I", next_chunk_size))
                    app_data.extend(
                        segment.data[current_addr : current_addr + next_chunk_size]
                    )
                    current_addr += next_chunk_size
                    next_packet = PusTc(
                        apid=0,
                        service=MEMORY_SERVICE,
                        subservice=RAW_MEMORY_WRITE_SUBSERVICE,
                        app_data=app_data,
                    )
                    com_if.send(next_packet.pack())
                    time.sleep(0.5)
        while True:
            data_available = com_if.data_available(0.4)
            if data_available:
                reply = com_if.receive()
                # TODO: Parse replies
                print("Received replies: {}", reply)
                break
    com_if.close()
    return 0


if __name__ == "__main__":
    main()
