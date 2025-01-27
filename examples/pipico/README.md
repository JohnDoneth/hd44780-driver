# Pi pico board example
This example is based on rp2040-project-template repo from rp-rs: https://github.com/rp-rs/rp2040-project-template.
The specific configs are for flashing using picoprobe (you have to compile openOCD from this fork: https://github.com/raspberrypi/openocd - all the instructions can be found 
in pi pico getting started pdf). This enables you to quickly flash your binary by running openOCD and executing cargo run.
Many of the cofigs are taken from this article series: https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry?s=r.
There's also configuration for (neo)vim fans using coc-rust-analyzer to have it working smoothly.
Please remember to connect RW to GND if you're not using busy flags. This will save you some time ;)
