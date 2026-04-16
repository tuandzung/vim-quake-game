<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-16 | Updated: 2026-04-16 -->

# src

All application source code and inline tests for vim-quake.

## Key Files
| File | Lines | Role |
|------|-------|------|
| `main.rs` | 54 | Binary entry — crossterm setup, event loop, delegates to game/renderer |
| `lib.rs` | 5 | Library root — `pub mod` re-exports all modules |
| `game.rs` | 365 | `App` state, `handle_event` dispatch, `parse_motion`, `execute_motion`, win check |
| `player.rs` | 412 | `Player` struct + 11 motion impls (h/j/k/l/w/b/0/$/f/t/dd) |
| `map.rs` | 287 | `Map` struct, 80×40 grid, 5 zones, `carve_level`, obstacles in Zone 5 |
| `renderer.rs` | 247 | ratatui TUI — title/gameplay/win screens, viewport + sidebar |
| `types.rs` | 252 | Position, Tile, Zone, VimMotion, Direction, GameState, PendingInput, App |

## Where To Look
| Task | File | What to change |
|------|------|----------------|
| Add Vim motion | `player.rs` + `types.rs` | VimMotion enum, handle_motion match arm, game.rs parse_motion |
| Change dungeon | `map.rs` | carve_level, assign_zones, obstacle placement |
| Change UI | `renderer.rs` | Pure display only — never mutates state |
| Change game flow | `game.rs` | handle_event, pending_input two-phase for f/t/dd |
| Add shared type | `types.rs` | All modules import via `crate::types::*` |

## Internal Dependencies
```
types.rs ← (all modules)
game.rs  ← player.rs, map.rs, types.rs
player.rs ← map.rs, types.rs
renderer.rs ← types.rs (read-only)
main.rs ← game.rs, renderer.rs, types.rs
```

## Conventions
- `grid[y][x]` row-major indexing — always bounds-check before access.
- Event handling: single-key motions execute immediately; f/t/dd set `pending_input` for next keypress.
- `Tile` has `glyph()` for char + `Display` for string. `VimMotion` has `key_label()`, `display_name()`, `description()`.

## Tests
55 inline tests across 4 files (`#[cfg(test)] mod tests` at bottom):
| File | Tests | Coverage |
|------|-------|----------|
| `player.rs` | 20 | All 11 motions + boundaries + motion recording |
| `game.rs` | 15 | Start/quit/motions/pending input/win + zone tracking |
| `map.rs` | 13 | Dimensions, tiles, passability, zones, corridors, obstacles |
| `types.rs` | 7 | Tile glyphs, motion labels, zone titles, direction deltas |
| `renderer.rs` | 0 | No tests |
| `main.rs` | 0 | No tests (thin wrapper) |
| `lib.rs` | 0 | No tests (re-exports only) |

Per-file test helpers: `test_map(w,h)`, `started_app_with_map(map,pos)`, `key_event(code)`.

## Notes
- Map start: (2,2), exit: (76,36). Zones: 16 columns each.
- Zone 5 has obstacles (only area with `Tile::Obstacle`).

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
