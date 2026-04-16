use crate::types::{Position, Tile, Zone};

pub struct Map {
    pub grid: Vec<Vec<Tile>>,
    pub zones: Vec<Vec<Zone>>,
    pub width: usize,
    pub height: usize,
    pub start: Position,
    pub exit: Position,
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    pub fn new() -> Self {
        let width = 80;
        let height = 40;
        let mut map = Self {
            grid: vec![vec![Tile::Wall; width]; height],
            zones: vec![vec![Zone::Zone1; width]; height],
            width,
            height,
            start: Position { x: 2, y: 2 },
            exit: Position { x: 76, y: 36 },
        };

        map.assign_zones();
        map.carve_level();
        map.set_tile(map.exit.x, map.exit.y, Tile::Exit);

        map
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        if x >= self.width || y >= self.height {
            Tile::Wall
        } else {
            self.grid[y][x]
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            self.grid[y][x] = tile;
        }
    }

    pub fn is_passable(&self, x: usize, y: usize) -> bool {
        matches!(self.get_tile(x, y), Tile::Floor | Tile::Exit)
    }

    pub fn zone_at(&self, pos: Position) -> Zone {
        self.zones[pos.y.min(self.height - 1)][pos.x.min(self.width - 1)]
    }

    fn assign_zones(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.zones[y][x] = match x {
                    0..=15 => Zone::Zone1,
                    16..=31 => Zone::Zone2,
                    32..=47 => Zone::Zone3,
                    48..=63 => Zone::Zone4,
                    _ => Zone::Zone5,
                };
            }
        }
    }

    fn carve_level(&mut self) {
        self.carve_horizontal(2, 2, 12);
        self.carve_vertical(12, 2, 8);
        self.carve_horizontal(8, 5, 12);
        self.carve_vertical(5, 8, 16);
        self.carve_horizontal(16, 5, 14);
        self.carve_vertical(14, 16, 20);

        self.carve_horizontal(20, 14, 16);
        self.carve_horizontal(20, 19, 21);
        self.carve_horizontal(20, 24, 26);
        self.carve_horizontal(20, 29, 31);
        self.carve_horizontal(22, 19, 21);
        self.carve_horizontal(22, 24, 26);
        self.carve_horizontal(22, 29, 31);
        self.carve_vertical(31, 20, 25);

        self.carve_horizontal(25, 31, 33);
        self.carve_horizontal(25, 45, 47);
        self.carve_vertical(47, 25, 30);
        self.carve_horizontal(30, 32, 34);
        self.carve_horizontal(30, 45, 47);
        self.carve_vertical(34, 30, 34);
        self.carve_horizontal(34, 32, 34);
        self.carve_horizontal(34, 45, 49);

        self.carve_horizontal(34, 55, 56);
        self.carve_horizontal(34, 61, 62);
        self.carve_vertical(62, 34, 35);
        self.carve_horizontal(35, 49, 62);
        self.carve_vertical(49, 35, 36);
        self.carve_horizontal(36, 49, 58);
        self.carve_horizontal(36, 60, 68);

        self.set_tile(59, 36, Tile::Wall);
        self.set_tile(69, 36, Tile::Obstacle);
        self.set_tile(73, 36, Tile::Obstacle);
        self.carve_horizontal(36, 70, 72);
        self.carve_horizontal(36, 74, 76);

        self.carve_vertical(68, 35, 37);
        self.carve_horizontal(37, 64, 68);
        self.set_tile(66, 37, Tile::Obstacle);
        self.set_tile(67, 37, Tile::Obstacle);

        self.carve_vertical(76, 34, 36);
        self.carve_horizontal(34, 72, 76);
        self.set_tile(74, 34, Tile::Obstacle);

        self.set_tile(self.start.x, self.start.y, Tile::Floor);
    }

    fn carve_horizontal(&mut self, y: usize, start_x: usize, end_x: usize) {
        for x in start_x.min(end_x)..=start_x.max(end_x) {
            self.set_tile(x, y, Tile::Floor);
        }
    }

    fn carve_vertical(&mut self, x: usize, start_y: usize, end_y: usize) {
        for y in start_y.min(end_y)..=start_y.max(end_y) {
            self.set_tile(x, y, Tile::Floor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_new_creates_80x40_grid() {
        let map = Map::new();

        assert_eq!(map.width, 80);
        assert_eq!(map.height, 40);
        assert_eq!(map.grid.len(), 40);
        assert!(map.grid.iter().all(|row| row.len() == 80));
    }

    #[test]
    fn map_start_is_floor() {
        let map = Map::new();

        assert_eq!(map.get_tile(2, 2), Tile::Floor);
    }

    #[test]
    fn map_exit_is_exit_tile() {
        let map = Map::new();

        assert_eq!(map.get_tile(76, 36), Tile::Exit);
    }

    #[test]
    fn map_default_tiles_are_wall() {
        let map = Map::new();

        assert_eq!(map.get_tile(0, 0), Tile::Wall);
        assert_eq!(map.get_tile(79, 0), Tile::Wall);
        assert_eq!(map.get_tile(0, 39), Tile::Wall);
    }

    #[test]
    fn map_get_tile_out_of_bounds_returns_wall() {
        let map = Map::new();

        assert_eq!(map.get_tile(80, 0), Tile::Wall);
        assert_eq!(map.get_tile(0, 40), Tile::Wall);
        assert_eq!(map.get_tile(80, 40), Tile::Wall);
    }

    #[test]
    fn map_set_and_get_roundtrip() {
        let mut map = Map::new();

        map.set_tile(10, 10, Tile::Obstacle);

        assert_eq!(map.get_tile(10, 10), Tile::Obstacle);
    }

    #[test]
    fn map_is_passable_floor() {
        let map = Map::new();

        assert!(map.is_passable(2, 2));
    }

    #[test]
    fn map_is_passable_exit() {
        let map = Map::new();

        assert!(map.is_passable(76, 36));
    }

    #[test]
    fn map_is_passable_wall() {
        let map = Map::new();

        assert!(!map.is_passable(0, 0));
    }

    #[test]
    fn map_is_passable_obstacle() {
        let map = Map::new();

        assert!(!map.is_passable(69, 36));
    }

    #[test]
    fn map_zone_at_zone1() {
        let map = Map::new();

        assert_eq!(map.zone_at(Position { x: 15, y: 0 }), Zone::Zone1);
    }

    #[test]
    fn map_zone_at_zone2() {
        let map = Map::new();

        assert_eq!(map.zone_at(Position { x: 16, y: 0 }), Zone::Zone2);
        assert_eq!(map.zone_at(Position { x: 31, y: 0 }), Zone::Zone2);
    }

    #[test]
    fn map_zone_at_zone3() {
        let map = Map::new();

        assert_eq!(map.zone_at(Position { x: 32, y: 0 }), Zone::Zone3);
        assert_eq!(map.zone_at(Position { x: 47, y: 0 }), Zone::Zone3);
    }

    #[test]
    fn map_zone_at_zone4() {
        let map = Map::new();

        assert_eq!(map.zone_at(Position { x: 48, y: 0 }), Zone::Zone4);
        assert_eq!(map.zone_at(Position { x: 63, y: 0 }), Zone::Zone4);
    }

    #[test]
    fn map_zone_at_zone5() {
        let map = Map::new();

        assert_eq!(map.zone_at(Position { x: 64, y: 0 }), Zone::Zone5);
        assert_eq!(map.zone_at(Position { x: 79, y: 0 }), Zone::Zone5);
    }

    #[test]
    fn map_has_carved_corridors() {
        let map = Map::new();
        let carved_tiles = map
            .grid
            .iter()
            .flatten()
            .filter(|tile| **tile != Tile::Wall)
            .count();

        assert!(carved_tiles > 0);
    }

    #[test]
    fn map_has_obstacles_in_zone5() {
        let map = Map::new();
        let zone5_obstacles = map
            .grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, tile)))
            .filter(|(x, _, tile)| *x >= 64 && **tile == Tile::Obstacle)
            .count();

        assert!(zone5_obstacles > 0);
    }
}
