# connect to OpenOCD on TCP port 3333
target extended-remote :3333

# print demangled function/variable symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault

# *try* stopping at the user entry point (it might be gone due to inlining)
break main

monitor arm semihosting enable

# load the application binary onto the Pico's flash
load

# start the process but immediately halt the processor
stepi
