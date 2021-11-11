gdb-server:
	JLinkGDBServer -select USB -device AMA3B1KK-KBR -endian little -if SWD -noir -noLocalhostOnly

rtt:
	stdbuf -i0 -e0 -o0 JLinkRTTClient < /dev/null

