# termori-on
A Tenori-on (ish) musical toy for your terminal

## What on earth?
The Tenori-on was a musical instrument made by Yamaha in the mid-2000s. It was a grid of buttons, each with an LED, that represented notes. Each column was a time step in a piece of music, each row was a pitch, and by pressing the buttons to light up notes, you could have it play a short melody or loop.

It spawned several imitations and similar grid-devices like Thinkgeek's Bliptronic, Novation's Launchpad, and was itself inspired by the Monome 40h.

## How do I?

To build, `cargo build --release` and then just run the `termori-on` program. You can use the arrow keys to move the cursor and enter to toggle notes, or you can use the mouse. When the time is right, your notes will play as simple sine waves.

Here is an example:

https://odysee.com/@ross.andrews:d/termori-on-1:f?r=HQbBZWVMvC4D9Sg3gFv91UPWoPCPHtbr

## Why won't it?

Because it's not done yet. More features are coming:

- Other, better waveforms
- Selectable tempo, volume, filter, etc
- Save and replay tunes

## Who would ever?

This fun noisemaker by Ross Andrews, October 2022.
