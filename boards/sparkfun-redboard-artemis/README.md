# Sparkfun Artemis Redboard (through native rust hal)

1) Build an example:

```sh
$ cargo objcopy --release --example blink -- O binary target/blink.bin
```

2) Flash:
```sh
$ python3 ../../tools/svl/svl.py -f target/blink.bin /dev/ttyUSB0 -v
```


