# simple fully open source smartwatch without bullshit like telemetry or pedometer 
![image](https://github.com/Talandar99/based_smartwatch/assets/32677600/a8b15e11-c897-40c0-be7d-c0b6ae6ad2fb)
## Goal (functionality)
- music control via bluetooth
- display notifications from phone (bluetooth)
- display time
- snake 
 
## How to flash?
- connect board via usb-C
- run `install_env_var_and_deps.sh` script for installing dependancies
- run `build_and_run.sh` script for building and flashing 
- **!!`build_and_run.sh` may require some adjustment based on your serial port (USB port)!!**


# Development
## Currently working on
- [ ] working bluetooth connection
## TODO/Milestones

- [X] setup and flash scripts
- [X] working buttons
- [X] working ssd1306 display
- [X] working fonts, shapes and pixels
- [X] displaying button input
- [X] displaying numbers via 7seg display
- [X] working clock
- [X] update time in proper intervals
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
- https://github.com/ivmarkov/rust-esp32-std-demo/tree/main
## Good to know 
- ssd1306 resolution is 128x64
- ssd1306 is monochromatic
- DON'T connect 5V pin to any GPIO, it will kill board
## Connections
#### Buttons
- 3V(esp32c3) into button(0)(left)  into gpio2 (esp32c3)
- 3V(esp32c3) into button(1)(down)  into gpio3 (esp32c3)
- 3V(esp32c3) into button(2)(up)    into gpio21(esp32c3)
- 3V(esp32c3) into button(3)(right) into gpio20(esp32c3)
#### monochromatic ssd1306
- 5V(esp32c3) into VCC(ssd1306)
- SDA(esp32c3) into SDA(ssd1306)
- SCL(esp32c3) into SCL(ssd1306)
- GND(esp32c3) into GND(ssd1306)
## Board Pinout Diagram
![image](https://github.com/Talandar99/xiaio_esp32c3_rust/assets/32677600/4272fa4f-edb4-428f-9e6a-cc33f96864be)

