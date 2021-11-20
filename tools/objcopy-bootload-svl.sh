#! /usr/bin/env bash

TARGET=${1}
BIN="${TARGET}.bin"

BOOTLOADER="$(dirname ${0})/svl/svl.py"

echo "size:"
arm-none-eabi-size "${TARGET}"

echo "objcopy.."
arm-none-eabi-objcopy -S -O binary "${TARGET}" "${BIN}"

echo "flashing /dev/ttyUSB0.."
python3 "${BOOTLOADER}" -f "${BIN}" /dev/ttyUSB0 -v

echo "running probe-run.."
probe-run --chip AMA3B1KK-KBR "${TARGET}" -v


