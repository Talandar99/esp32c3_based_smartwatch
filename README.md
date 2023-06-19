# BASEDwatch 
![image](https://github.com/Talandar99/based_smartwatch/assets/32677600/a8b15e11-c897-40c0-be7d-c0b6ae6ad2fb)
## Goal (functionality)
- music control via bluetooth
- display notifications from phone (bluetooth)
- display time
- snake 
- mini jack
 
## How to flash?
- connect board via usb-C
- run `install_env_var_and_deps.sh` script for installing dependancies
- run `build_and_run.sh` script for building and flashing 
- **!!`build_and_run.sh` may require some adjustment based on your serial port !!**


# Development
## Currently working on
- [ ] update time in proper intervals

## TODO/Milestones

- [X] setup and flash scripts
- [X] working buttons
- [X] working ssd1306 display
- [X] working fonts, shapes and pixels
- [X] displaying button input
- [X] displaying numbers via 7seg display
- [X] working clock
- [ ] update time in proper intervals
- [ ] working bluetooth connection
- [ ] connection diagram
- [ ] controller can be powered by battery
- [ ] displaying battery %
- [ ] sleep mode 

## Usefull links
- https://github.com/esp-rs/esp-idf-template/tree/master
- https://github.com/esp-rs/espflash/tree/main/espflash#usage
- https://github.com/esp-rs/esp-idf-hal
- https://wiki.seeedstudio.com/XIAO_ESP32C3_Getting_Started/
- https://github.com/jamwaffles/ssd1306
- https://github.com/esp-rs/esp-wifi/blob/main/examples-esp32c3/examples/ble.rs
- https://github.com/esp-rs/awesome-esp-rust#video-courses
- https://lib.rs/crates/bluedroid
- https://lib.rs/crates/embedded-time
## Good to know 
- ssd1306 resolution 128x64

## Pinout Diagram
![image](https://github.com/Talandar99/xiaio_esp32c3_rust/assets/32677600/4272fa4f-edb4-428f-9e6a-cc33f96864be)

