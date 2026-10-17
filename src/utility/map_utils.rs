use crate::constants::map::{GameMap, MAP_MASTER, TILE_MASTER, Tile, WARP_MASTER, Warp, find_map};

/// タイル配列を除いたマップのメタ情報
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapSummary {
    pub id: u8,
    pub name: &'static str,
    pub name_en: &'static str,
    pub kind: &'static str,
    pub width: u8,
    pub height: u8,
    pub border_tile: u8,
    pub npc_count: usize,
}

impl From<&GameMap> for MapSummary {
    fn from(map: &GameMap) -> Self {
        Self {
            id: map.id,
            name: map.name,
            name_en: map.name_en,
            kind: map.kind,
            width: map.width,
            height: map.height,
            border_tile: map.border_tile,
            npc_count: map.npcs.len(),
        }
    }
}

pub fn map_summaries() -> Vec<MapSummary> {
    MAP_MASTER.iter().map(MapSummary::from).collect()
}

/// 指定座標のタイルIDを返す。範囲外は border_tile、マップが存在しなければ None
pub fn tile_at(map_id: u8, x: i32, y: i32) -> Option<u8> {
    let map = find_map(map_id)?;
    let (w, h) = (map.width as i32, map.height as i32);
    if x < 0 || y < 0 || x >= w || y >= h {
        return Some(map.border_tile);
    }
    map.tiles.get((y * w + x) as usize).copied()
}

pub fn find_tile(id: u8) -> Option<&'static Tile> {
    TILE_MASTER.iter().find(|t| t.id == id)
}

pub fn warps_from(map_id: u8) -> Vec<&'static Warp> {
    WARP_MASTER.iter().filter(|w| w.from.0 == map_id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_map_field() {
        let map = find_map(1).unwrap();
        assert_eq!(map.kind, "field");
        assert_eq!(map.width, 120);
    }

    #[test]
    fn test_find_map_non_existing() {
        assert!(find_map(0).is_none());
    }

    #[test]
    fn test_tiles_length_matches_size() {
        for map in MAP_MASTER.iter() {
            assert_eq!(
                map.tiles.len(),
                map.width as usize * map.height as usize,
                "map id {}",
                map.id
            );
        }
    }

    #[test]
    fn test_tile_at_in_range() {
        let map = find_map(1).unwrap();
        assert_eq!(tile_at(1, 0, 0), Some(map.tiles[0]));
        assert_eq!(tile_at(1, 5, 2), Some(map.tiles[2 * 120 + 5]));
    }

    #[test]
    fn test_tile_at_out_of_range_returns_border() {
        let map = find_map(1).unwrap();
        assert_eq!(tile_at(1, -1, 0), Some(map.border_tile));
        assert_eq!(tile_at(1, 0, 120), Some(map.border_tile));
    }

    #[test]
    fn test_tile_at_unknown_map() {
        assert!(tile_at(0, 0, 0).is_none());
    }

    #[test]
    fn test_all_tiles_defined() {
        for map in MAP_MASTER.iter() {
            assert!(find_tile(map.border_tile).is_some(), "map id {}", map.id);
            for &t in map.tiles {
                assert!(find_tile(t).is_some(), "map id {} tile {}", map.id, t);
            }
        }
    }

    #[test]
    fn test_warps_reference_existing_maps() {
        for warp in WARP_MASTER.iter() {
            assert!(find_map(warp.from.0).is_some());
            assert!(find_map(warp.to.0).is_some());
        }
        assert!(!warps_from(1).is_empty());
    }

    #[test]
    fn test_map_summaries_count() {
        assert_eq!(map_summaries().len(), MAP_MASTER.len());
    }
}
