use crate::constants::encounter::{
    ENCOUNTER_GROUPS, FIXED_ENCOUNTERS, FixedEncounter, encounter_group, encounter_one_in,
    group0_battle_chance, repelled_by_fairy_water,
};
use crate::constants::monster::MONSTER_MASTER;
use crate::player::Player;

const FIELD_MAP_ID: u8 = 1;
const GOLEM: u8 = 24;
const DRAGON: u8 = 30;

/// 戦闘の発生に関わるプレイヤーの状態
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EncounterContext {
    pub defeated_dragon: bool,
    pub defeated_golem: bool,
    /// せいすい の効果中ならプレイヤーの守備力
    pub fairy_water_defense: Option<u8>,
}

impl EncounterContext {
    pub fn from_player(player: &Player, fairy_water: bool) -> Self {
        Self {
            defeated_dragon: player.flags.defeated_dragon,
            defeated_golem: player.flags.defeated_golem,
            fairy_water_defense: fairy_water.then(|| player.defense_power()),
        }
    }

    fn is_defeated(&self, fixed: &FixedEncounter) -> bool {
        fixed.once
            && match fixed.monster {
                DRAGON => self.defeated_dragon,
                GOLEM => self.defeated_golem,
                _ => false,
            }
    }
}

pub fn fixed_encounter_at(map_id: u8, x: u8, y: u8) -> Option<&'static FixedEncounter> {
    FIXED_ENCOUNTERS
        .iter()
        .find(|f| f.map_id == map_id && f.x == x && f.y == y)
}

/// 1歩歩いたときに戦闘になる敵 (MONSTER_MASTER の添字)。戦闘にならなければ None。
/// 決まった場所の敵 → 出現率 → グループ → 1体 の順に判定する。
/// rng(n) は 0..n の乱数を返す
pub fn roll_encounter(
    map_id: u8,
    x: u8,
    y: u8,
    tile_id: u8,
    context: &EncounterContext,
    rng: &mut impl FnMut(u32) -> u32,
) -> Option<u8> {
    if let Some(fixed) = fixed_encounter_at(map_id, x, y) {
        if !context.is_defeated(fixed) {
            return Some(fixed.monster);
        }
    }

    let group = encounter_group(map_id, x, y)?;
    if rng(encounter_one_in(tile_id, x, y) as u32) != 0 {
        return None;
    }
    if map_id == FIELD_MAP_ID && group == 0 {
        let (num, den) = group0_battle_chance(tile_id);
        if rng(den as u32) >= num as u32 {
            return None;
        }
    }

    let monsters = ENCOUNTER_GROUPS[group as usize];
    let monster = monsters[rng(monsters.len() as u32) as usize];

    if let (FIELD_MAP_ID, Some(defense)) = (map_id, context.fairy_water_defense) {
        if repelled_by_fairy_water(MONSTER_MASTER[monster as usize].attack, defense) {
            return None;
        }
    }
    Some(monster)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::encounter::HARP_MONSTERS;
    use crate::constants::map::MAP_MASTER;

    /// 決まった値を順に返す乱数
    fn sequence(values: &[u32]) -> impl FnMut(u32) -> u32 + '_ {
        let mut iter = values.iter();
        move |n| iter.next().copied().unwrap_or(0) % n
    }

    #[test]
    fn test_groups_in_range() {
        for map in MAP_MASTER.iter() {
            for y in 0..map.height {
                for x in 0..map.width {
                    if let Some(group) = encounter_group(map.id, x, y) {
                        assert!((group as usize) < ENCOUNTER_GROUPS.len(), "map {}", map.id);
                    }
                }
            }
        }
        for &monster in ENCOUNTER_GROUPS.iter().flatten().chain(&HARP_MONSTERS) {
            assert!((monster as usize) < MONSTER_MASTER.len());
        }
        for fixed in FIXED_ENCOUNTERS.iter() {
            assert!((fixed.monster as usize) < MONSTER_MASTER.len());
        }
    }

    #[test]
    fn test_around_tantegel_is_group0() {
        assert_eq!(encounter_group(1, 43, 43), Some(0));
        let names: Vec<&str> = ENCOUNTER_GROUPS[0]
            .iter()
            .map(|&m| MONSTER_MASTER[m as usize].name)
            .collect();
        for name in names {
            assert!(
                ["スライム", "スライムベス", "ドラキー"].contains(&name),
                "{}",
                name
            );
        }
    }

    #[test]
    fn test_no_encounter_in_towns() {
        // ラダトーム、ラダトームのしろ
        assert_eq!(encounter_group(8, 5, 5), None);
        assert_eq!(encounter_group(4, 5, 5), None);
        let context = EncounterContext::default();
        assert_eq!(
            roll_encounter(8, 5, 5, 4, &context, &mut sequence(&[0, 0, 0])),
            None
        );
    }

    #[test]
    fn test_roll_encounter_order() {
        let context = EncounterContext::default();
        // 出現率の判定で外れる
        assert_eq!(
            roll_encounter(1, 100, 100, 0, &context, &mut sequence(&[1])),
            None
        );
        // 当たり → 5体の 3番目
        let group = encounter_group(1, 100, 100).unwrap() as usize;
        assert_eq!(
            roll_encounter(1, 100, 100, 0, &context, &mut sequence(&[0, 2])),
            Some(ENCOUNTER_GROUPS[group][2])
        );
        // グループ0 は半分取りやめ
        assert_eq!(
            roll_encounter(1, 43, 44, 0, &context, &mut sequence(&[0, 1, 0])),
            None
        );
        assert_eq!(
            roll_encounter(1, 43, 44, 0, &context, &mut sequence(&[0, 0, 0])),
            Some(0)
        );
    }

    #[test]
    fn test_fixed_encounters() {
        let mut context = EncounterContext::default();
        // ゴーレムは乱数に関係なく出て、倒すと出ない
        assert_eq!(
            roll_encounter(1, 73, 100, 0, &context, &mut sequence(&[1])),
            Some(GOLEM)
        );
        context.defeated_golem = true;
        assert_eq!(
            roll_encounter(1, 73, 100, 0, &context, &mut sequence(&[1])),
            None
        );
        // あくまのきしは何度でも出る
        assert_eq!(
            roll_encounter(3, 18, 12, 4, &context, &mut sequence(&[1])),
            Some(33)
        );
    }

    #[test]
    fn test_fairy_water_repels_weak_monsters() {
        let context = EncounterContext {
            fairy_water_defense: Some(100),
            ..Default::default()
        };
        assert_eq!(
            roll_encounter(1, 43, 44, 0, &context, &mut sequence(&[0, 0, 0])),
            None
        );
        // フィールド以外では効かない
        let monster = roll_encounter(15, 1, 1, 4, &context, &mut sequence(&[0, 0]));
        assert!(monster.is_some());
    }
}
