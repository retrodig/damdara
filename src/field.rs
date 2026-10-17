use crate::constants::map::{GameMap, WARP_MASTER, find_map};

/// フィールドのマップID
pub const FIELD_MAP_ID: u8 = 1;
/// 探索開始位置（ラダトームのしろの入口）
pub const START_POSITION: (u8, u8, u8) = (4, 11, 29);

const TILE_STAIRS_UP: u8 = 3;
const TILE_FLOOR: u8 = 4;
const TILE_STAIRS_DOWN: u8 = 5;
const TILE_DOOR: u8 = 17;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveEvent {
    Moved,
    Blocked,
    /// 別のマップへ移動した（町や城への出入り・階段）
    Warped {
        from_map: u8,
        to_map: u8,
    },
    /// かぎを使って扉を開けた
    DoorOpened,
    /// かぎがなく扉を開けられない
    DoorLocked,
}

/// 通れないタイル（カウンター・海・壁・山・看板・ローラひめ）
pub fn is_passable(tile: u8) -> bool {
    !matches!(tile, 14 | 15 | 16 | 18 | 19 | 20 | 23)
}

/// 現在地と、開けた扉（マップを出るまで有効）を持つ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub map_id: u8,
    pub x: u8,
    pub y: u8,
    pub steps: u32,
    opened_doors: Vec<(u8, u8)>,
}

impl Default for Field {
    fn default() -> Self {
        let (map_id, x, y) = START_POSITION;
        Self::new(map_id, x, y)
    }
}

impl Field {
    pub fn new(map_id: u8, x: u8, y: u8) -> Self {
        Self {
            map_id,
            x,
            y,
            steps: 0,
            opened_doors: Vec::new(),
        }
    }

    pub fn map(&self) -> &'static GameMap {
        find_map(self.map_id).expect("map not found")
    }

    /// 指定座標のタイル。範囲外は border_tile、開けた扉は床になる
    pub fn tile(&self, x: i32, y: i32) -> u8 {
        let map = self.map();
        if x < 0 || y < 0 || x >= map.width as i32 || y >= map.height as i32 {
            return map.border_tile;
        }
        if self.opened_doors.contains(&(x as u8, y as u8)) {
            return TILE_FLOOR;
        }
        map.tiles[y as usize * map.width as usize + x as usize]
    }

    /// 1歩移動する。扉はかぎ (keys) を1つ使って開ける
    pub fn try_move(&mut self, direction: Direction, keys: &mut u8) -> MoveEvent {
        let map = self.map();
        let (dx, dy) = direction.delta();
        let (nx, ny) = (self.x as i32 + dx, self.y as i32 + dy);

        let out_of_range = nx < 0 || ny < 0 || nx >= map.width as i32 || ny >= map.height as i32;
        if out_of_range && self.map_id != FIELD_MAP_ID {
            // 町や城の外に出るとフィールドの入口へ戻る
            return match self.field_exit() {
                Some((map_id, x, y)) => self.warp(map_id, x, y),
                None => MoveEvent::Blocked,
            };
        }

        let tile = self.tile(nx, ny);
        if tile == TILE_DOOR {
            if *keys == 0 {
                return MoveEvent::DoorLocked;
            }
            *keys -= 1;
            self.opened_doors.push((nx as u8, ny as u8));
            return MoveEvent::DoorOpened;
        }
        if out_of_range || !is_passable(tile) || self.has_npc(nx as u8, ny as u8) {
            return MoveEvent::Blocked;
        }

        self.x = nx as u8;
        self.y = ny as u8;
        self.steps += 1;

        match self.warp_destination(tile) {
            Some((map_id, x, y)) => self.warp(map_id, x, y),
            None => MoveEvent::Moved,
        }
    }

    fn has_npc(&self, x: u8, y: u8) -> bool {
        self.map().npcs.iter().any(|npc| npc.x == x && npc.y == y)
    }

    /// 入口・階段に乗ったときの移動先。階段は WARP_MASTER の逆方向にも使える
    fn warp_destination(&self, tile: u8) -> Option<(u8, u8, u8)> {
        let here = (self.map_id, self.x, self.y);
        if let Some(warp) = WARP_MASTER.iter().find(|w| w.from == here) {
            return Some(warp.to);
        }
        if matches!(tile, TILE_STAIRS_UP | TILE_STAIRS_DOWN) {
            return WARP_MASTER.iter().find(|w| w.to == here).map(|w| w.from);
        }
        None
    }

    /// このマップの入口のうち、現在地にいちばん近いフィールド上の位置
    fn field_exit(&self) -> Option<(u8, u8, u8)> {
        let distance = |(_, x, y): (u8, u8, u8)| {
            (x as i32 - self.x as i32).abs() + (y as i32 - self.y as i32).abs()
        };
        WARP_MASTER
            .iter()
            .filter(|w| w.from.0 == FIELD_MAP_ID && w.to.0 == self.map_id)
            .min_by_key(|w| distance(w.to))
            .map(|w| w.from)
    }

    fn warp(&mut self, map_id: u8, x: u8, y: u8) -> MoveEvent {
        let from_map = self.map_id;
        self.map_id = map_id;
        self.x = x;
        self.y = y;
        self.opened_doors.clear();
        MoveEvent::Warped {
            from_map,
            to_map: map_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn moves(field: &mut Field, directions: &[Direction]) -> Vec<MoveEvent> {
        let mut keys = 0;
        directions
            .iter()
            .map(|&d| field.try_move(d, &mut keys))
            .collect()
    }

    #[test]
    fn test_exit_castle_to_field_and_enter_again() {
        let mut field = Field::default();
        let events = moves(&mut field, &[Direction::Down]);
        assert_eq!(
            events,
            vec![MoveEvent::Warped {
                from_map: 4,
                to_map: FIELD_MAP_ID
            }]
        );
        assert_eq!((field.map_id, field.x, field.y), (1, 43, 43));

        // 一度離れて戻ると城に入る
        moves(&mut field, &[Direction::Right]);
        let events = moves(&mut field, &[Direction::Left]);
        assert_eq!(
            events,
            vec![MoveEvent::Warped {
                from_map: 1,
                to_map: 4
            }]
        );
        assert_eq!((field.map_id, field.x, field.y), START_POSITION);
    }

    #[test]
    fn test_blocked_by_sea_at_field_edge() {
        let mut field = Field::new(FIELD_MAP_ID, 0, 60);
        let before = field.clone();
        assert_eq!(
            moves(&mut field, &[Direction::Left]),
            vec![MoveEvent::Blocked]
        );
        assert_eq!(field, before);
    }

    #[test]
    fn test_stairs_both_ways() {
        // ラダトームのしろ → ちか
        let mut field = Field::new(4, 29, 28);
        assert_eq!(
            moves(&mut field, &[Direction::Down]),
            vec![MoveEvent::Warped {
                from_map: 4,
                to_map: 12
            }]
        );
        assert_eq!((field.map_id, field.x, field.y), (12, 0, 4));

        // 階段から一度降りて、また乗ると戻る
        let (x, y) = (field.x as i32, field.y as i32);
        let direction = [
            Direction::Right,
            Direction::Down,
            Direction::Up,
            Direction::Left,
        ]
        .into_iter()
        .find(|d| {
            let (dx, dy) = d.delta();
            is_passable(field.tile(x + dx, y + dy))
        })
        .unwrap();
        let back = match direction {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        };
        moves(&mut field, &[direction]);
        assert_eq!(
            moves(&mut field, &[back]),
            vec![MoveEvent::Warped {
                from_map: 12,
                to_map: 4
            }]
        );
        assert_eq!((field.map_id, field.x, field.y), (4, 29, 29));
    }

    #[test]
    fn test_door_needs_key() {
        let map = find_map(4).unwrap();
        let index = map.tiles.iter().position(|&t| t == TILE_DOOR).unwrap();
        let (dx, dy) = (
            (index % map.width as usize) as u8,
            (index / map.width as usize) as u8,
        );
        let mut field = Field::new(4, dx, dy + 1);

        let mut keys = 0;
        assert_eq!(
            field.try_move(Direction::Up, &mut keys),
            MoveEvent::DoorLocked
        );

        let mut keys = 1;
        assert_eq!(
            field.try_move(Direction::Up, &mut keys),
            MoveEvent::DoorOpened
        );
        assert_eq!(keys, 0);
        assert_eq!(field.try_move(Direction::Up, &mut keys), MoveEvent::Moved);
        assert_eq!((field.x, field.y), (dx, dy));
    }

    #[test]
    fn test_all_field_entrances_can_exit() {
        for warp in WARP_MASTER.iter().filter(|w| w.from.0 == FIELD_MAP_ID) {
            let (map_id, x, y) = warp.to;
            let field = Field::new(map_id, x, y);
            assert!(field.field_exit().is_some(), "map {}", map_id);
        }
    }
}
