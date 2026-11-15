# mdlife

A tiny miny micro Markdown preview.
Lets you preview a `.md` file from the terminal, and supports hot-reload.
It is cross-platform, written in egui, Rust.

Made for [Hack Club](https://hackclub.com).

## ~ USAGE ~
```bash
cargo install --git https://github.com/artemtsitronov/mdlife
mdlife <file>
```
A file argument is NECESSARY. Otherwise, poor `mdlife` will apologize for something that's your fault!

## The HackClub challenge torture
- **HOT RELOAD**: Serve them while they're hot! Hot reloading is supported via `notify`
- **NO IF/ELSE, SWITCHES OR TERNARIES**: UGH. Fine. No `match` was used either. Branching's done with `Option` and array lookups.
- **CROSS PLATFORM**: Built on `eframe` and `notify` which support Linux, macOS, Windows.

I might as well advertise another crate of mine, but for a good reason: [fenth](https://github.com/artemtsitronov/fenth) was actually used for themes, mapped to egui's `Visuals`. The theme is `catppuccin-mocha`
