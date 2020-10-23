# DIY Music Box

It is created to make playing music or audio books simple for children (and maybe adults).
The software is written in Rust. It uses the following componentes:

- STM32f103C8T6 (Blue Pill)
- DFPlayer
- RC522 Tag Reader
- Some other stuff (RFID Tag, Speaker, Resistor, Powerbank)

The project is inspired by [TonUINO](https://github.com/xfjx/TonUINO), that has nearly the same function, but is written for an Arduino UNO.  
The goal of this project is to write the software using Rust.

## Features
- [ ] Select a Folder with an RFID Tag
- [ ] Select Volume by pressing one of the three buttons#
- [ ] Skip a track
- [ ] Pause playing with one of the buttons
- [ ] Program an RFID Tag
- [ ] Auto turn of after some time
- [ ] Different Playback modes (Loop, Shuffle, Loop One)


## Software architecture 

For the software the [RTIC](https://rtic.rs) framework is used. 
Different things schould happen at the same time (pressing a button, reading Tags, communicating the the DFPlayer). 
This is achived by the softwaretask and schedule feature of the RTIC framework.

### Task Organization
The main application logic is placed in the idle function. This functions is waiting for events generated in other task. The events are send with a MPSC queu to the main application logic.

Possible Events are (list not complete):
- Button is pressed and released
- Tag has been placed on the the musicbox
- Tag has been removed from the musicbox
- A Track ended

The differnt events are linked to one hardware resources (Button -> GPIO, Tag -> RC522, Tracks Ended -> DFPlayer)
This events are generated in sepratat tasks that are triggerd regular (Tag) or with interrupts (Buttons)

The idle function acts on an incomming event with an action (most of the time it will send a command to the DFPlayer). Therefore also a task is spawned that does the desired action. 

### Button Evaluation
The Button down event is detected with an interrupt. The Interrupt schedules a task that checks the state of the pin and disables the interrupt. If the button is released within 1 second a short button press is detected. If the button stays down for longer than 1 second a long button press is detected

### TAG Detection and readout
The Idle task spanws a periodic task that checks the tag reader for presence of a tag. If a new tag is detected, therelevant datafiels are read out and send as an event to the applikation


