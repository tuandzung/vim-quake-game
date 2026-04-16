<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# src

All application source code and inline tests for vim-quake.

## Key Files
| File | Lines | Role |
|------|-------|------|
| `main.rs` | 54 | Binary entry — crossterm setup, event loop, delegates to game/renderer |
| `lib.rs` | 6 | Library root — `pub mod` re-exports all modules (incl. enemy) |
| `game.rs` | 763 | `App` state, `handle_event` dispatch, enemy turns, lives, win/loss, level progression |
| `player.rs` | 521 | `Player` struct + 13 motion impls (h/j/k/l/w/b/0/$/G/gg/f/t/dd) |
| `map.rs` | 714 | `Map` struct, 80×40 grid, 3 levels, 5 zones, `carve_level`, obstacles, `enemy_spawn_points()` |
| `renderer.rs` | 690 | ratatui TUI — title/gameplay/win/loss screens, viewport + sidebar, trail, glow, enemies |
| `enemy.rs` | 244 | `Enemy` struct + BFS pathfinding (`step_toward_player`), collision handling |
| `types.rs` | 306 | Position, Tile, Zone, VimMotion, Direction, GameState, PendingInput, Enemy, App |

## Where To Look
| Task | File | What to change |
|------|------|----------------|
| Add Vim motion | `player.rs` + `types.rs` | VimMotion enum, handle_motion match arm, game.rs parse_motion |
| Change dungeon | `map.rs` | carve_level (3 level variants), assign_zones, obstacle placement |
| Change UI | `renderer.rs` | Pure display only — never mutates state |
| Change game flow | `game.rs` | handle_event, pending_input two-phase for f/t/dd/gg |
| Change enemy AI | `enemy.rs` | BFS pathfinding, step_toward_player |
| Add shared type | `types.rs` | All modules import via `crate::types::*` |

## Internal Dependencies
```
types.rs ← (all modules)
game.rs  ← player.rs, map.rs, enemy.rs, types.rs
player.rs ← map.rs, types.rs
enemy.rs ← map.rs, types.rs
renderer.rs ← types.rs (read-only)
main.rs ← game.rs, renderer.rs, types.rs
```

## Conventions
- `grid[y][x]` row-major indexing — always bounds-check before access.
- Event handling: single-key motions execute immediately; f/t/dd/gg set `pending_input` for next keypress.
- `Tile` has `glyph()` for char + `Display` for string. `VimMotion` has `key_label()`, `display_name()`, `description()`.
- Enemies only on Level 3. `enemy_spawn_points()` returns spawn positions. BFS each enemy turn.
- 3 lives. Collision → `lives -= 1` + remove enemy. 0 lives → `GameState::Lost` → any key retries level.

## Tests
117 inline tests across 5 files (`#[cfg(test)] mod tests` at bottom):
| File | Tests | Coverage |
|------|-------|----------|
| `game.rs` | 35 | Motions, pending input, win/loss, level transitions, enemies, lives, trail, zone tracking |
| `map.rs` | 33 | Dimensions, tiles, passability, zones, corridors, obstacles, 3 levels, enemy spawns |
| `player.rs` | 25 | All 13 motions + boundaries + dd obstacle + motion recording |
| `enemy.rs` | 10 | BFS pathfinding, wall avoidance, diagonal, corridor following, collision |
| `types.rs` | 14 | Tile glyphs, motion labels/descriptions, zone titles, direction deltas, Enemy struct |
| `renderer.rs` | 0 | No tests |
| `main.rs` | 0 | No tests (thin wrapper) |
| `lib.rs` | 0 | No tests (re-exports only) |

Per-file test helpers: `test_map(w,h)`, `started_app_with_map(map,pos)`, `key_event(code)`.

## Notes
- Map start: (2,2), exit: (76,36). Zones: 16 columns each.
- Level 1 = basic maze. Level 2 = inverted maze + obstacles in earlier zones. Level 3 = zigzag + enemies.
- Zone 5 has obstacles (only area with `Tile::Obstacle`).

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
