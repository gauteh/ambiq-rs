gdb-server:
	JLinkGDBServerCLExe -select USB -device AMA3B1KK-KBR -if SWD -nogui

jlink:
	JLinkExe -device AMA3B1KK-KBR -if SWD

rtt:
	# stdbuf -i0 -e0 -o0 JLinkRTTClient < /dev/null
	stdbuf -i0 -e0 -o0 nc localhost 19021

