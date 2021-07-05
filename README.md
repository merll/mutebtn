# MuteBtn

Connects mute hardware buttons to apps

## Supported environments

Currently the only supported device is [MuteMe™](https://muteme.com/), because this is the only one I have available for testing. However, there is no reason why this should remain the only supported device. Even DIY devices could be added.

This app was developed on and for Linux using PulseAudio. In this environment it provides the most value over the vendor-provided app of MuteMe™. However, it is written in Rust, so it should be possible to adapt it to any environment. More audio servers will be added.

## Why?

The vendor-provided app of MuteMe™ worked for me, but instead of the heavy closed-source Electron app I wanted something lighter with the possibility to run as a system service.

Also with the closed-source app my confidence that Linux features will be developed much further is quite low. Especially Linux-specific FOSS apps will likely not get much more support.

## Development plans

Next planned steps in development are:
* Support more apps (e.g. Mumble)
* Support more audio servers directly (e.g. Pipewire)

Contributions welcome, also for more devices!

## Disclaimer

Note that this app is not associated with or endorsed by MuteMe™ or any other vendor.
