#!/usr/bin/env python3
from spacepackets.ecss import RequestId
from spacepackets.ecss.defs import PusService
from spacepackets.ecss.tm import PusTm
import toml
import struct
import logging
import argparse
import threading
import time
import enum
from tmtccmd.com.serial_base import SerialCfg
from tmtccmd.com.serial_cobs import SerialCobsComIF
from tmtccmd.com.ser_utils import prompt_com_port
from crcmod.predefined import PredefinedCrc
from spacepackets.ecss.tc import PusTc
from spacepackets.ecss.pus_verificator import PusVerificator, StatusField
from spacepackets.ecss.pus_1_verification import Service1Tm, UnpackParams
from spacepackets.seqcount import SeqCountProvider
from pathlib import Path
import dataclasses
from elftools.elf.elffile import ELFFile


BAUD_RATE = 115200
BOOTLOADER_START_ADDR = 0x0
BOOTLOADER_END_ADDR = 0x4000
BOOTLOADER_CRC_ADDR = 0x3FFC
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
ACTION_SERVICE = 8

RAW_MEMORY_WRITE_SUBSERVICE = 2
BOOT_NVM_MEMORY_ID = 1


class ActionId(enum.IntEnum):
    CORRUPT_APP_A = 128
    CORRUPT_APP_B = 129


_LOGGER = logging.getLogger(__name__)


@dataclasses.dataclass
class LoadableSegment:
    name: str
    offset: int
    size: int
    data: bytes


SEQ_PROVIDER = SeqCountProvider(bit_width=14)


def main() -> int:
    print("Python VA416XX Image Loader Application")
    logging.basicConfig(
        format="[%(asctime)s] [%(levelname)s] %(message)s", level=logging.DEBUG
    )
    parser = argparse.ArgumentParser(
        prog="image-loader", description="Python VA416XX Image Loader Application"
    )
    parser.add_argument("-p", "--ping", action="store_true", help="Send ping command")
    parser.add_argument("-c", "--corrupt", action="store_true", help="Corrupt a target")
    parser.add_argument(
        "-t",
        "--target",
        choices=["bl", "a", "b"],
        help="Target (Bootloader or slot A or B)",
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
    verificator = PusVerificator()
    com_if = SerialCobsComIF(serial_cfg)
    com_if.open()
    file_path = None
    if args.target:
        if not args.corrupt:
            if not args.path:
                _LOGGER.error("App Path needs to be specified for the flash process")
                return -1
            file_path = Path(args.path)
            if not file_path.exists():
                _LOGGER.error("File does not exist")
                return -1
    if args.ping:
        _LOGGER.info("Sending ping command")
        ping_tc = PusTc(
            apid=0x00,
            service=PusService.S17_TEST,
            subservice=1,
            seq_count=SEQ_PROVIDER.get_and_increment(),
        )
        com_if.send(ping_tc.pack())
    if args.corrupt:
        if not args.target:
            _LOGGER.error("target for corruption command required")
            return -1
        if args.target == "bl":
            _LOGGER.error("can not corrupt bootloader")
        if args.target == "a":
            packet = PusTc(
                apid=0,
                service=ACTION_SERVICE,
                subservice=ActionId.CORRUPT_APP_A,
            )
            com_if.send(packet.pack())
        if args.target == "b":
            packet = PusTc(
                apid=0,
                service=ACTION_SERVICE,
                subservice=ActionId.CORRUPT_APP_B,
            )
            com_if.send(packet.pack())
    else:
        assert file_path is not None
        loadable_segments = []
        _LOGGER.info("Parsing ELF file for loadable sections")
        total_size = 0
        with open(file_path, "rb") as app_file:
            elf_file = ELFFile(app_file)

            for (idx, segment) in enumerate(elf_file.iter_segments("PT_LOAD")):
                if segment.header.p_filesz == 0:
                    continue
                # Basic validity checks of the base addresses.
                if idx == 0:
                    if (
                        args.target == "bl"
                        and segment.header.p_paddr != BOOTLOADER_START_ADDR
                    ):
                        raise ValueError(
                            f"detected possibly invalid start address {segment.header.p_paddr:#08x} for "
                            f"bootloader, expected {BOOTLOADER_START_ADDR}"
                        )
                    if (
                        args.target == "a"
                        and segment.header.p_paddr != APP_A_START_ADDR
                    ):
                        raise ValueError(
                            f"detected possibly invalid start address {segment.header.p_paddr:#08x} for "
                            f"App A, expected {APP_A_START_ADDR}"
                        )
                    if (
                        args.target == "b"
                        and segment.header.p_paddr != APP_B_START_ADDR
                    ):
                        raise ValueError(
                            f"detected possibly invalid start address {segment.header.p_paddr:#08x} for "
                            f"App B, expected {APP_B_START_ADDR}"
                        )
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
                total_size += segment.header.p_filesz
            context_str = None
            if args.target == "bl":
                context_str = "Bootloader"
            elif args.target == "a":
                context_str = "App Slot A"
            elif args.target == "b":
                context_str = "App Slot B"
            _LOGGER.info(
                f"Flashing {context_str} with image {file_path} (size {total_size})"
            )
            for idx, segment in enumerate(loadable_segments):
                _LOGGER.info(
                    f"Loadable section {idx} {segment.name} with offset {segment.offset:#08x} and size {segment.size}"
                )
            for segment in loadable_segments:
                segment_end = segment.offset + segment.size
                current_addr = segment.offset
                pos_in_segment = 0
                while pos_in_segment < segment.size:
                    next_chunk_size = min(segment_end - current_addr, CHUNK_SIZE)
                    data = segment.data[
                        pos_in_segment : pos_in_segment + next_chunk_size
                    ]
                    next_packet = pack_memory_write_command(current_addr, data)
                    _LOGGER.info(
                        f"Sending memory write command for address {current_addr:#08x} and data with "
                        f"length {len(data)}"
                    )
                    verificator.add_tc(next_packet)
                    com_if.send(next_packet.pack())
                    current_addr += next_chunk_size
                    pos_in_segment += next_chunk_size
                    while True:
                        data_available = com_if.data_available(0.1)
                        done = False
                        if not data_available:
                            continue
                        replies = com_if.receive()
                        for reply in replies:
                            tm = PusTm.unpack(reply, 0)
                            if tm.service != 1:
                                continue
                            service_1_tm = Service1Tm.from_tm(tm, UnpackParams(0))
                            check_result = verificator.add_tm(service_1_tm)
                            # We could send after we have received the step reply, but that can
                            # somehow lead to overrun errors. I think it's okay to do it like
                            # this as long as the flash loader only uses polling..
                            if (
                                check_result is not None
                                and check_result.status.completed == StatusField.SUCCESS
                            ):
                                done = True
                                # Still keep a small delay
                                time.sleep(0.01)
                        verificator.remove_completed_entries()
                        if done:
                            break
            if args.target == "bl":
                _LOGGER.info("Blanking the bootloader checksum")
                # Blank the checksum. For the bootloader, the bootloader will calculate the
                # checksum itself on the initial run.
                checksum_write_packet = pack_memory_write_command(
                    BOOTLOADER_CRC_ADDR, bytes([0x00, 0x00, 0x00, 0x00])
                )
                com_if.send(checksum_write_packet.pack())
            else:
                crc_addr = None
                size_addr = None
                if args.target == "a":
                    crc_addr = APP_A_CRC_ADDR
                    size_addr = APP_A_SIZE_ADDR
                elif args.target == "b":
                    crc_addr = APP_B_CRC_ADDR
                    size_addr = APP_B_SIZE_ADDR
                assert crc_addr is not None
                assert size_addr is not None
                _LOGGER.info(
                    f"Writing app size {total_size} at address {size_addr:#08x}"
                )
                size_write_packet = pack_memory_write_command(
                    size_addr, struct.pack("!I", total_size)
                )
                com_if.send(size_write_packet.pack())
                time.sleep(0.2)
                crc_calc = PredefinedCrc("crc-32")
                for segment in loadable_segments:
                    crc_calc.update(segment.data)
                checksum = crc_calc.digest()
                _LOGGER.info(
                    f"Writing checksum 0x[{checksum.hex(sep=',')}] at address {crc_addr:#08x}"
                )
                checksum_write_packet = pack_memory_write_command(crc_addr, checksum)
                com_if.send(checksum_write_packet.pack())
    com_if.close()
    return 0


def pack_memory_write_command(addr: int, data: bytes) -> PusTc:
    app_data = bytearray()
    app_data.append(BOOT_NVM_MEMORY_ID)
    # N parameter is always 1 here.
    app_data.append(1)
    app_data.extend(struct.pack("!I", addr))
    app_data.extend(struct.pack("!I", len(data)))
    app_data.extend(data)
    return PusTc(
        apid=0,
        service=MEMORY_SERVICE,
        subservice=RAW_MEMORY_WRITE_SUBSERVICE,
        seq_count=SEQ_PROVIDER.get_and_increment(),
        app_data=app_data,
    )


if __name__ == "__main__":
    main()
