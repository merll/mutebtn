# MuteBtn

Connects mute controllers such as MuteMe™ to apps

## Supported environments

Currently the only supported device is [MuteMe™](https://muteme.com/), because this is the only one I have available for testing. However, there is no reason why this should remain the only supported device. Even DIY devices could be added.

This app was developed on and for Linux using PulseAudio. In this environment it provides the most value over the vendor-provided app of MuteMe™. However, it is written in Rust, so it should be possible to adapt it to any environment. More audio servers will be added.

## Why?

The vendor-provided app of MuteMe™ worked for me, but instead of the heavy closed-source Electron app I wanted something lighter with the possibility to run as a system service.

Also with the closed-source app my confidence that Linux features will be developed much further is quite low. Especially Linux-specific FOSS apps will likely not get much more support.

# Additional features

Besides the vendor-provided app features (color setting, push-to-talk or toggle mode), the following is supported:
* Selecting the PulseAudio device: Select a specific audio-device or the selected default device separately for mute and unmute. The default is to mute/unmute all PulseAudio sources.
* Hybrid mode: If you prefer push-to-talk, but sometimes get tired of holding the button, you can double-tap, and it will leave the mic open until you touch once again, similar to toggle mode.

# Missing features

* There is no GUI yet.
* Currently settings cannot be changed at run-time. The app has to be restarted. This will change soon.

# Configuration

The app will look for configuration files in the following places:
* Any command line option provided with `-c <config_file>` or `--config <config_file>`.
* `mutebtn.toml` in the current working directory.
* `/etc/mutebtn.toml`

Format: Several formats such as JSON, YAML etc are supported, but TOML is the recommended option.
If any entry in the configuration file is invalid, all defaults apply.

Example with all options:

```toml
[main]
# Optional. If set to true, mutes selected devices on app start; if set to false, unmutes
# selected devices on app start. If not present, does nothing (default).
mute_on_startup = true

[muteme]
# Color when muted (default: red) or unmuted (default: green).
# Valid choices are "red", "green", "blue", "yelllow", "cyan", "purple", "white", and "nocolor".
muted_color = "red"
unmuted_color = "green"

# Operation mode. Valid choices are "toggle" (default), "pushtotalk", and "hybrid".
operation_mode = "hybrid"

# Only applies to "hybrid" mode: Maximum duration in milliseconds to detect a double-tap
# (1), and the following release (2). Defaults to values below:
double_tap_duration_1 = 300
double_tap_duration_2 = 250

[pulse]
# Device to mute. Choices are "all" (default setting), "default", and "selected". On
# "default", the current default audio source is re-detected on each mute/unmute operation.
mute_device = "all"
# Optional, separate selection of which device to unmute. Choices are the same as for
# mute_device; if not set unmutes the same as in mute_device. This example shows that you
# can always mute all audio sources, and only unmute the default device on-demand.
unmute_device = "default"

# Only applies if mute_device or unmute_device is set to "selected": Defines the specific
# device name. Available names can e.g. be listed using "pactl list sources".
selected_device_name = "my_device"
```

## Development plans

Next planned steps in development are:
* Provide some sort of interface to change settings comfortably at run-time.
* Provide systemd sample config for autostart.
* Support more apps (e.g. Mumble)
* Support more audio servers directly (e.g. Pipewire)

Contributions welcome, also for more devices!

## Disclaimer

Note that this app is not associated with or endorsed by MuteMe™ or any other vendor.
