<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->
<!-- Commit: a2b11d1 | Branch: main -->

# vim-quake

Terminal-based roguelike dungeon game (Rust + ratatui) teaching Vim motions through gameplay. 3 dungeon levels (80×40), 5 zone-gated areas, 13 Vim keybindings, BFS-chasing enemies, lives system.

## Structure
```
vim-quake/
├── src/          # All source + inline tests (see src/AGENTS.md)
├── Cargo.toml    # Edition 2024, deps: anyhow, crossterm, ratatui
├── Cargo.lock
├── README.md
└── .gitignore    # target/, *.rs.bk, *.pdb, mutants.out*/, .omc/
```

## Architecture
```
main.rs     → Terminal setup + event loop (54 lines)
game.rs     → App state, event handling, enemy turns, win/loss, level progression (763 lines)
player.rs   → Player + 13 motion implementations (521 lines)
map.rs      → 80×40 grid, 3 dungeon levels, 5 zones, corridor carving, obstacles, enemy spawns (714 lines)
renderer.rs → ratatui TUI: title, viewport, sidebar, win/loss screens, enemies, trail, glow (690 lines)
enemy.rs    → Enemy struct + BFS pathfinding toward player (244 lines)
types.rs    → Position, Tile, Zone, VimMotion, Direction, GameState, PendingInput, Enemy, App (306 lines)
lib.rs      → Re-exports: enemy, game, map, player, renderer, types (6 lines)
```

## Where To Look
| Task | Location | Notes |
|------|----------|-------|
| Add a new Vim motion | `src/player.rs` + `src/types.rs` (VimMotion enum) | Also update game.rs parse_motion |
| Change dungeon layout | `src/map.rs` (carve_level, assign_zones) | grid[y][x] row-major indexing |
| Add UI elements | `src/renderer.rs` | Pure display — never mutates state |
| Change game flow | `src/game.rs` (handle_event, App) | Two-phase input for f/t/dd/gg |
| Change enemy behavior | `src/enemy.rs` | BFS pathfinding, collision handling |
| Add new types | `src/types.rs` | All modules import via `crate::types::*` |
| Fix a bug | Check tests first: 117 inline tests across 5 files | renderer.rs, main.rs, lib.rs have no tests |

## Conventions
- Rust edition 2024. No clippy/rustfmt config — defaults apply.
- Inline tests only (`#[cfg(test)] mod tests`). No `tests/` directory, no test frameworks.
- Test helpers per-file: `test_map()`, `started_app_with_map()`, `key_event()`.
- `lib.rs` re-exports all modules. `main.rs` is thin (~54 lines).
- `is_passable` = `Tile::Floor` or `Tile::Exit` only.
- `renderer.rs` is read-only — never mutates App state.
- `Player::handle_motion` takes `&mut Map` (dd deletes obstacles).
- No `unsafe`, no `unwrap()` in non-test code, no `panic!()`.

## Commands
```bash
cargo build          # Compile
cargo test           # Run 117 inline tests
cargo run            # Launch game in terminal
```

## Dependencies
| Crate | Version | Used In |
|-------|---------|---------|
| anyhow | 1.0 | main.rs (error propagation) |
| crossterm | 0.28 | main.rs (terminal), game.rs (KeyCode) |
| ratatui | 0.29 | renderer.rs exclusively |

## Notes
- No CI/CD configured. No Makefile, build.rs, or custom scripts.
- No config files beyond Cargo.toml (no .editorconfig, clippy.toml, rustfmt.toml).
- Coordinate system: `grid[y][x]` — always bounds-check before access.
- 3 dungeon levels: Level 1 (basic maze), Level 2 (inverted maze + obstacles in earlier zones), Level 3 (zigzag + enemies).
- Enemies spawn only on Level 3 via `Map::enemy_spawn_points()`. BFS pathfinding in `enemy.rs`.
- Lives: start with 3. Enemy collision → lose 1 life + remove enemy. 0 lives → loss screen → any key retries level.
- `total_levels()` in types.rs = 3. Completing all 3 → victory screen with zone-by-zone breakdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
