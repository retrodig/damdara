use crate::constants::monster::{MONSTER_BEHAVIORS, MONSTER_MASTER, MonsterBehavior, MonsterStats};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Monster {
    pub hp: u8,
    pub stats: MonsterStats,
    pub behavior: MonsterBehavior,
}

impl Monster {
    pub fn new(index: usize) -> Self {
        let stats = MONSTER_MASTER.get(index).unwrap_or(&MONSTER_MASTER[0]);
        let behavior = MONSTER_BEHAVIORS
            .get(index)
            .unwrap_or(&MONSTER_BEHAVIORS[0]);

        let mut rng = rand::rng();
        let random_value: u16 = rng.random_range(0..=255);
        let reduction = (stats.hp as u16 * random_value) / 1024;
        let initial_hp = stats.hp.saturating_sub(reduction as u8);

        Self {
            hp: initial_hp,
            stats: stats.clone(),
            behavior: behavior.clone(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.stats.hp > 0
    }
}
