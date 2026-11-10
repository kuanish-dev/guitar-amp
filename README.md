# Guitar Amp

A simple standalone guitar amp for macOS, written in Rust.

- Amp with gain, bass, mid, tone and volume
- Effects: gate, compressor, auto wah, distortion, phaser, chorus, tremolo, delay, reverb, limiter
- 14 presets
- Built-in tuner

## Use

1. Plug your guitar into an audio interface.
2. Open the app, pick the input and output, press **Start**.
3. Pick a preset or move the sliders.

Use headphones or an interface. The laptop microphone and speakers will feed back.

## Build

```bash
cargo run --release
```

The CLAP plugin version lives in [clap-plugins](https://github.com/kuanish-dev/clap-plugins).
