# hyprlayer

A minimal transparent overlay for Wayland compositors, built with
Rust, GTK 4, and gtk4-layer-shell.
Intended as a base for overlays in gaming,
system status displays, or other use cases requiring HUD-style visuals.

## Features

- Transparent, borderless overlay window
- CSS-styled label rendering
- Lifecycle logic to show/hide dynamically
- Designed for use on Wayland (tested on Hyprland)
- Fully reproducible using Nix and Rust

## Requirements

- A Wayland compositor (e.g., Hyprland or Sway)
- Nix with flakes enabled
- Rust toolchain provided by the flake

## Usage

```bash
nix run github:clemenscodes/hyprlayer
```

## Development

```bash
nix develop -C $SHELL
```

## License

MIT
