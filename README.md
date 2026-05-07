 ███████╗ █████╗ ██╗     ██████╗ ███████╗██████╗ ███████╗███████╗ ██████╗██╗  ██╗
 ██╔════╝██╔══██╗██║     ██╔══██╗██╔════╝██╔══██╗██╔════╝██╔════╝██╔════╝██║  ██║
 █████╗  ███████║██║     ██║  ██║███████╗██████╔╝█████╗  █████╗  ██║     ███████║
 ██╔══╝  ██╔══██║██║     ██║  ██║╚════██║██╔═══╝ ██╔══╝  ██╔══╝  ██║     ██╔══██║
 ███████╗██║  ██║███████╗██████╔╝███████║██║     ███████╗███████╗╚██████╗██║  ██║
 ╚══════╝╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝╚═╝     ╚══════╝╚══════╝ ╚═════╝╚═╝  ╚═╝

                    Speak the old tongue once more.

> A terminal-driven Anglish translator forged for folk who think English had enough French thrust upon it already.

---

# EaldSpeech

`EaldSpeech` is a TUI Anglish translator that shifts modern English toward a more Germanic, plainspoken tongue.

Built for:
- terminal wanderers
- Linux gremlins
- language nerds
- history enjoyers
- people who see "utilize" and take psychic damage

---

## Features

- ⚔️ Real-time Anglish translation
- 🖥️ Clean terminal interface
- 📜 Word-root replacement system
- 🌲 Optional archaic flavoring
- 🔥 Pipe stdin directly into the translator
- 🧙 Multiple translation depths
- 🐧 Linux-first workflow

---

## Example

### Input

```bash
ealdspeech "Please commence utilization of the beverage container."
```

### Output

```text
Please start use of the drink holder.
```

### Deep Mode

```bash
ealdspeech --deep
```

```text
Pray begin use of the drink cask.
```

---

## Installation

### Cargo

```bash
cargo install ealdspeech
```

### Build from source

```bash
git clone https://github.com/ghostfrogcherry/EaldSpeech
cd EaldSpeech
cargo build --release
```

---

## Usage

### Interactive TUI

```bash
ealdspeech
```

### Pipe text

```bash
echo "Commence the operation immediately." | ealdspeech
```

### Translate a file

```bash
ealdspeech speech.txt
```

### Wyrd Mode™

```bash
ealdspeech --wyrd
```

May become unreadable.
This is expected.

---

## Philosophy

English was forever changed in 1066.

`EaldSpeech` asks:

> "What if we made it weirder again?"

This is not a scholarly reconstruction tool.

It is:
- part language project
- part terminal toy
- part cultural necromancy

---

## Planned Features

- [x] Full ratatui interface
- [x] CLI argument, file, and pipe input
- [x] Deep and wyrd translation modes
- [ ] Configurable translation profiles
- [ ] Etymology inspection
- [ ] Vim keybinds
- [ ] Offline wordhoard
- [ ] WASM/web build
- [ ] Beowulf mode (unsafe)

---

## Screenshot

```text
┌─ EaldSpeech ───────────────────────────────┐
│                                            │
│  commence  → start                         │
│  utilize   → use                           │
│  beverage  → drink                         │
│                                            │
│  > _                                       │
│                                            │
└────────────────────────────────────────────┘
```

---

## License

MIT

Unlike England after Hastings.
