# vim-quake

A terminal-based roguelike dungeon game that teaches Vim motions through gameplay. Navigate three 80×40 dungeon levels using real Vim keybindings, dodge enemies, and reach the exit.

## Features

- **3 dungeon levels** with distinct layouts — Level 2 is an inverted maze, Level 3 is a zigzag descent with enemy patrols
- **5 zone-gated areas** per level with distinct color palettes (gray → cyan → magenta → red → gold)
- **Level progression** — stats carry over, trail resets, new map loads on reaching the exit
- **Enemy encounters** — Level 3 spawns BFS-chasing enemies that step toward you each turn
- **Lives and retry** — you start with 3 lives; enemy collisions cost a life, losing all lives triggers a loss screen, and any key retries the current level
- **Figlet-style ASCII art** title screen with motion reference
- **Player trail** — fading green dots show your recent path
- **Animated exit glow** — pulsing `►` beacon guides you to the goal
- **Depth-aware walls** — glyph variation (█▓▒#) based on neighbor analysis
- **Victory screen** — ASCII trophy, zone-by-zone completion breakdown with progress bars, and motion mastery rating (up to 13 motions)

## Motions

| Key | Motion | Zone |
|-----|--------|------|
| `h` `j` `k` `l` | Left / Down / Up / Right | 1 |
| `w` `b` | Word forward / back | 2 |
| `0` `$` `G` `gg` | Line start / end / last row / first row | 3 |
| `f<char>` `t<char>` | Find / till char | 4 |
| `dd` | Delete obstacle | 5 |

The dungeon is divided into 5 zone-gated areas. Each zone unlocks progressively harder motions. Level 1 teaches basic movement, Level 2 adds obstacles, and Level 3 introduces enemies.

## Quick Start

```bash
cargo run
```

## Controls

- Move with the Vim motions listed above
- `q` / `Esc` — quit
- Any key — start from title screen

Reach the exit (`>`) on each level. Complete all 3 levels to win. Lose all lives and you can retry the current level with a fresh map.

## Build & Test

```bash
cargo build    # Compile
cargo test     # Run 117 inline tests
cargo run      # Play
```

## Architecture

```
src/main.rs     Terminal setup + event loop
src/game.rs     App state, event handling, enemy turns, win/loss conditions, trail tracking
src/player.rs   Player + 13 motion implementations
src/map.rs      80×40 grid, 5 zones, corridor carving, 3 dungeon levels, enemy spawn points
src/enemy.rs    Enemy struct with BFS pathfinding toward the player
src/renderer.rs ratatui TUI rendering (zone colors, trail, glow, enemies, loss/win screens, ASCII art)
src/types.rs    Shared types (Position, Tile, Zone, VimMotion, Enemy, GameState, App, …)
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
