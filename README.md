# rusty-irrigation-system
An irrigation system written in rust.
For now, runs only on Raspberry-Pie (tested with zero 2 w), more platforms are welcome
Allow you to open a single valve (more features to come, multi-valve, etc')

## How to use
Download latest [release](https://github.com/ShtiviOmer/rusty_irrigation-system/releases/latest)
copy it to your Raspberry-Pie
Create a file in the same directory called config.yaml
``` yaml
valve_type: RaspberryPie
gpio_pins: 4
version: 1
watering_clock:
  start_time: 05:00:00
  duration: 30
  interval: 24
```
make the rusty-irrigation-system executable:
``` bash
chmod a+x rusty_irrigation_system
```

## Dev
In order to compile the code, needs a different target:
First add [cross](https://github.com/cross-rs/cross)
Best to follow this [guide](https://amritrathie.vercel.app/posts/2020/03/06/cross-compiling-rust-from-macos-to-raspberry-pi/) which worked for me
Afterwards, we can run commands:
```
cross test --target armv7-unknown-linux-musleabihf
```
