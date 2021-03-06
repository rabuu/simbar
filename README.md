# simbar
Simple status bar for [suckless.org](suckless.org)'s [dwm](https://dwm.suckless.org/) written in Rust.
`simbar` is intended to be easily configurable and extendable.

There is color support but you have to [patch](https://dwm.suckless.org/patches/status2d/) dwm.

## dependencies
You need `Xlib` to get `simbar` working.

## config
Per default the config directory is `${XDG_CONFIG_HOME:-$HOME/.config}/simbar`.
In this directory you have to create a `simbar.toml` config file.

In the `simbar.toml` file you have to call the modules you want to enable.
You can configure how often they should be rerun and their color.
Here is an [example config](example_config).

You can change your config at runtime but don't remove or move the `simbar.toml` file.

### Xresources
Normally, you can set a specific color by adding it directly into your config,
but you can replace a color code with `xresources` and the color is generated by querying XRDB.

## external scripts
You can update the status bar from external scripts simply by running:
```
simbar --once
```
No signalling needed.
