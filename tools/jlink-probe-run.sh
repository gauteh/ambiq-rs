#! /usr/bin/env bash

set -ep

TARGET=${1}
BIN="${TARGET}.bin"
JLINK="${TARGET}.jlink"

BOOTLOADER="$(dirname ${0})/svl/svl.py"

echo "size:"
arm-none-eabi-size "${TARGET}"

echo "objcopy.."
arm-none-eabi-objcopy -S -O binary "${TARGET}" "${BIN}"

echo "writing JLink script to ${JLINK}.."
cat > ${JLINK} <<EOF
r
sleep 10
wreg MSP, 0x10000100
loadbin ${BIN}, 0x10000
w4 0x40000004 0x1B
qc
EOF

echo "flashing using JLinkExe.."
JLinkExe -device AMA3B1KK-KBR -autoconnect 1 -if swd -speed 4000 -NoGui -ExitOnError -CommandFile ${JLINK}

echo "running probe-run.."
probe-run --chip AMA3B1KK-KBR $@


