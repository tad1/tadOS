Result of College assignment: a tad OS written for Raspberry Pi 3B\
based on [rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)

### Installation
1. Prepare SD card ([link](https://projects.raspberrypi.org/en/projects/raspberry-pi-setting-up/2))
2. Replace config.txt and kernel8.img on boot volume

### Requirements
You need a UART to USB converter (i.e., CP21xx) to connect with Raspberry.

## Overview
It is a single-core and single-process OS. The base functionality allows the execution of static ELF.\
OS exports a minimal functionality kernel call; you can check example programs for tadOS [here](https://github.com/tad1/tadOSv0.1-Utils).

**But.. can it run DOOM?**\
it should, yet there's no generic doom for embedded Rust (a future project? maybe)
 

## History
- v0.1 - current version
- [early development](https://tad1.dev/notes/Projects/OS+Project/Raw+Log)
