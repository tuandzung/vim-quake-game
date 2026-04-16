# vim-quake

A terminal-based roguelike dungeon game that teaches Vim motions through gameplay. Navigate an 80×40 dungeon using real Vim keybindings to reach the exit.

## Motions

| Key | Motion | Zone |
|-----|--------|------|
| `h` `j` `k` `l` | Left / Down / Up / Right | 1 |
| `w` `b` | Word forward / back | 2 |
| `0` `$` | Line start / end | 3 |
| `f<char>` `t<char>` | Find / till char | 4 |
| `dd` | Delete obstacle | 5 |

The dungeon is divided into 5 zone-gated areas. Each zone unlocks progressively harder motions.

## Quick Start

```bash
cargo run
```

## Controls

- Move with the Vim motions listed above
- `q` — quit
- `Enter` — start from title screen

Reach the exit (`>`) to win.

## Build & Test

```bash
cargo build    # Compile
cargo test     # Run 55 inline tests
cargo run      # Play
```

## Architecture

```
src/main.rs     Terminal setup + event loop
src/game.rs     App state, event handling, win condition
src/player.rs   Player + 11 motion implementations
src/map.rs      80×40 grid, 5 zones, corridor carving
src/renderer.rs ratatui TUI rendering
src/types.rs    Shared types (Position, Tile, Zone, VimMotion, …)
src/lib.rs      Module re-exports
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| [ratatui](https://crates.io/crates/ratatui) 0.29 | Terminal UI framework |
| [crossterm](https://crates.io/crates/crossterm) 0.28 | Cross-platform terminal control |
| [anyhow](https://crates.io/crates/anyhow) 1.0 | Error handling |

## License

MIT
