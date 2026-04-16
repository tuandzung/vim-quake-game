use std::collections::VecDeque;

use crate::map::Map;
use crate::types::{Enemy, Position};

impl Enemy {
    pub fn new(pos: Position) -> Self {
        Self {
            position: pos,
            glyph: 'e',
        }
    }

    pub fn step_toward_player(&mut self, player_pos: Position, map: &Map) -> bool {
        if self.position == player_pos {
            return false;
        }

        let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dx, dy) in &directions {
            let nx = self.position.x as isize + dx;
            let ny = self.position.y as isize + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx >= map.width || ny >= map.height {
                continue;
            }
            if (Position { x: nx, y: ny }) == player_pos {
                self.position = player_pos;
                return true;
            }
        }

        let mut visited = vec![vec![false; map.width]; map.height];
        let mut parent: Vec<Vec<Option<Position>>> = vec![vec![None; map.width]; map.height];
        let mut queue = VecDeque::new();

        visited[self.position.y][self.position.x] = true;
        queue.push_back(self.position);

        while let Some(pos) = queue.pop_front() {
            if pos == player_pos {
                break;
            }

            for (dx, dy) in &directions {
                let nx = pos.x as isize + dx;
                let ny = pos.y as isize + dy;
                if nx < 0 || ny < 0 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if nx >= map.width || ny >= map.height {
                    continue;
                }
                if visited[ny][nx] {
                    continue;
                }
                let neighbor = Position { x: nx, y: ny };
                if !map.is_passable(nx, ny) && neighbor != player_pos {
                    continue;
                }

                visited[ny][nx] = true;
                parent[ny][nx] = Some(pos);
                queue.push_back(neighbor);
            }
        }

        if !visited[player_pos.y][player_pos.x] {
            return false;
        }

        let mut step = player_pos;
        while let Some(prev) = parent[step.y][step.x] {
            if prev == self.position {
                self.position = step;
                return true;
            }
            step = prev;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Tile, Zone};

    fn test_map(width: usize, height: usize) -> Map {
        Map {
            grid: vec![vec![Tile::Floor; width]; height],
            zones: vec![vec![Zone::Zone1; width]; height],
            width,
            height,
            start: Position { x: 0, y: 0 },
            exit: Position {
                x: width - 1,
                y: height - 1,
            },
            enemy_spawns: vec![],
        }
    }

    #[test]
    fn enemy_new_has_default_glyph() {
        let enemy = Enemy::new(Position { x: 3, y: 5 });

        assert_eq!(enemy.position, Position { x: 3, y: 5 });
        assert_eq!(enemy.glyph, 'e');
    }

    #[test]
    fn enemy_steps_toward_player() {
        let map = test_map(5, 5);
        let mut enemy = Enemy::new(Position { x: 0, y: 0 });
        let player_pos = Position { x: 3, y: 0 };

        let moved = enemy.step_toward_player(player_pos, &map);

        assert!(moved);
        assert_eq!(enemy.position, Position { x: 1, y: 0 });
    }

    #[test]
    fn enemy_steps_toward_player_diagonal() {
        let map = test_map(5, 5);
        let mut enemy = Enemy::new(Position { x: 0, y: 0 });
        let player_pos = Position { x: 3, y: 3 };

        let moved = enemy.step_toward_player(player_pos, &map);

        assert!(moved);
        assert_eq!(enemy.position, Position { x: 1, y: 0 });
    }

    #[test]
    fn enemy_does_not_walk_through_walls() {
        let mut map = test_map(5, 1);
        map.set_tile(1, 0, Tile::Wall);
        map.set_tile(2, 0, Tile::Wall);
        let mut enemy = Enemy::new(Position { x: 0, y: 0 });
        let player_pos = Position { x: 4, y: 0 };

        let moved = enemy.step_toward_player(player_pos, &map);

        assert!(!moved);
        assert_eq!(enemy.position, Position { x: 0, y: 0 });
    }

    #[test]
    fn enemy_adjacent_moves_onto_player_tile() {
        let map = test_map(5, 5);
        let mut enemy = Enemy::new(Position { x: 1, y: 0 });
        let player_pos = Position { x: 2, y: 0 };

        let moved = enemy.step_toward_player(player_pos, &map);

        assert!(moved);
        assert_eq!(enemy.position, player_pos);
    }

    #[test]
    fn enemy_adjacent_vertical_moves_onto_player_tile() {
        let map = test_map(5, 5);
        let mut enemy = Enemy::new(Position { x: 2, y: 1 });
        let player_pos = Position { x: 2, y: 2 };

        let moved = enemy.step_toward_player(player_pos, &map);

        assert!(moved);
        assert_eq!(enemy.position, player_pos);
    }

    #[test]
    fn enemy_no_path_stays_put() {
        let mut map = test_map(5, 5);
        for x in 0..5 {
            map.set_tile(x, 2, Tile::Wall);
        }
        let mut enemy = Enemy::new(Position { x: 2, y: 0 });
        let player_pos = Position { x: 2, y: 4 };

        let moved = enemy.step_toward_player(player_pos, &map);

        assert!(!moved);
        assert_eq!(enemy.position, Position { x: 2, y: 0 });
    }

    #[test]
    fn enemy_already_on_player_does_not_move() {
        let map = test_map(5, 5);
        let pos = Position { x: 2, y: 2 };
        let mut enemy = Enemy::new(pos);

        let moved = enemy.step_toward_player(pos, &map);

        assert!(!moved);
        assert_eq!(enemy.position, pos);
    }

    #[test]
    fn enemy_follows_corridor_path() {
        let mut map = test_map(5, 5);
        for y in 0..5 {
            for x in 0..5 {
                map.set_tile(x, y, Tile::Wall);
            }
        }
        map.set_tile(0, 0, Tile::Floor);
        map.set_tile(0, 1, Tile::Floor);
        map.set_tile(0, 2, Tile::Floor);
        map.set_tile(1, 2, Tile::Floor);
        map.set_tile(2, 2, Tile::Floor);

        let mut enemy = Enemy::new(Position { x: 0, y: 0 });
        let player_pos = Position { x: 2, y: 2 };

        let moved = enemy.step_toward_player(player_pos, &map);

        assert!(moved);
        assert_eq!(enemy.position, Position { x: 0, y: 1 });
    }

    #[test]
    fn enemy_takes_shortest_path() {
        let map = test_map(3, 3);
        let mut enemy = Enemy::new(Position { x: 0, y: 0 });
        let player_pos = Position { x: 2, y: 2 };

        let moved = enemy.step_toward_player(player_pos, &map);

        assert!(moved);
        let valid_steps = [Position { x: 1, y: 0 }, Position { x: 0, y: 1 }];
        assert!(valid_steps.contains(&enemy.position));
    }
}
