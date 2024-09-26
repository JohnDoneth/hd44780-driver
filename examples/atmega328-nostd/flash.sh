#!/usr/bin/bash

FILENAME="$1"

# To change the flasher via CLI, use "cargo run -- <FLASHER>"
#FLASHER=arduino
FLASHER=usbasp

if [ -n "$2" ]; then
    FLASHER="$2"
fi

avrdude -c "$FLASHER" -p atmega328p -b 57600 -e -D -U "flash:w:$FILENAME:e"
