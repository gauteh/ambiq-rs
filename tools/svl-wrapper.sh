#! /usr/bin/env bash

set -ep

TARGET=${1}
BIN="${TARGET}.bin"

BOOTLOADER="$(dirname ${0})/svl/svl.py"

echo "size:"
arm-none-eabi-size "${TARGET}"

echo "objcopy.."
arm-none-eabi-objcopy -S -O binary "${TARGET}" "${BIN}"

echo "flashing /dev/ttyUSB0.."
python3 "${BOOTLOADER}" -f "${BIN}" /dev/ttyUSB0 -v


