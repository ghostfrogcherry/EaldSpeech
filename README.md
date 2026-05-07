# EaldSpeech — An Anglish Oversetter

A terminal UI that translates English into **Anglish** (English stripped of French and Latin influence, using only Germanic-root words). Written in Rust with ratatui.

## Quick Start

```bash
cargo run --release
```

Or grab the binary from the releases page.

## Usage

| Key | Action |
|---|---|
| `Ctrl+Q` / `Esc` | Quit |
| `Tab` | Cycle theme |
| `Ctrl+R` | Toggle direction (English↔Anglish) |
| `Ctrl+A` | About page |
| `Ctrl+C` | Copy output to clipboard |
| `Ctrl+L` | Clear input |
| `Ctrl+V` | Paste (bracketed paste supported) |

Type in the top panel — translation appears live below.

## Themes

Six dark palettes: Catppuccin, Nord, Dracula, Tokyo Night, One Dark, Gruvbox.
