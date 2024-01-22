# Image to Audio

The app takes an image and "turns it into" sound, a process called sonification.

## About
Was made as an experiment for a uni assignment. It's very specific and currently only supports .png images.\
Every image will be drawn at 600x600, however, the buffer will still contain all the image data, pre-scaling.\
Exporting to .wav will not save the playback speed!

## How it works
It will basically treat images as a vector of brightness values, scaled to -1 to 1.\
These values are then used as samples to "plot" a waveform.

## Usage
I - Load image\
L/R Arrow - Change playback speed\
S - Save current image (buffer) as a .wav\
D - Display some info\
Esc - Quit

![image](https://github.com/definitelynico/macroquad_img_to_audio/assets/101659586/f2af2cc4-1ddd-4c49-b2c3-998db5e6fa6d)

![image](https://github.com/definitelynico/macroquad_img_to_audio/assets/101659586/bd9a0e4b-e2ec-4254-856d-5850eac1d223)

These images are AI-generated
