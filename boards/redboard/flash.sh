# JRun -device AMA3B1KK-KBR -if swd --speed 1000 --nortt --nosemihost $1
# ../../tools/objcopy-bootload-svl.sh $1

# JLinkExe -device AMA3B1KK-KBR -autoconnect 1 -if swd -speed 4000 &

# jlink=$!

# tmux split-window 'nc localhost 19021 | less'

# wait $jlink

# gdb-multiarch -x flash.gdb

