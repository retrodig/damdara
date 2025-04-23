#[derive(Debug, Clone)]
pub struct Equipment {
    pub name: &'static str,
    pub price: u16,
    pub sell: u16,
    pub attack: u16,
    pub defense: u16,
}

pub const WEAPON_MASTER: [Equipment; 8] = [
    Equipment {
        name: "なし",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "たけざお",
        price: 10,
        sell: 5,
        attack: 2,
        defense: 0,
    },
    Equipment {
        name: "こんぼう",
        price: 60,
        sell: 30,
        attack: 4,
        defense: 0,
    },
    Equipment {
        name: "どうのつるぎ",
        price: 180,
        sell: 90,
        attack: 10,
        defense: 0,
    },
    Equipment {
        name: "てつのおの",
        price: 560,
        sell: 280,
        attack: 15,
        defense: 0,
    },
    Equipment {
        name: "はがねのつるぎ",
        price: 1500,
        sell: 750,
        attack: 20,
        defense: 0,
    },
    Equipment {
        name: "ほのおのつるぎ",
        price: 9800,
        sell: 4900,
        attack: 28,
        defense: 0,
    },
    Equipment {
        name: "ロトのつるぎ",
        price: 2,
        sell: 1,
        attack: 40,
        defense: 0,
    },
];

pub const ARMOR_MASTER: [Equipment; 8] = [
    Equipment {
        name: "なし",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "ぬののふく",
        price: 20,
        sell: 10,
        attack: 0,
        defense: 2,
    },
    Equipment {
        name: "かわのふく",
        price: 70,
        sell: 35,
        attack: 0,
        defense: 4,
    },
    Equipment {
        name: "くさりかたびら",
        price: 300,
        sell: 150,
        attack: 0,
        defense: 8,
    },
    Equipment {
        name: "てつのよろい",
        price: 1000,
        sell: 500,
        attack: 0,
        defense: 16,
    },
    Equipment {
        name: "はがねのよろい",
        price: 3000,
        sell: 1500,
        attack: 0,
        defense: 24,
    },
    Equipment {
        name: "まほうのよろい",
        price: 7700,
        sell: 3850,
        attack: 0,
        defense: 24,
    },
    Equipment {
        name: "ロトのよろい",
        price: 2,
        sell: 1,
        attack: 0,
        defense: 28,
    },
];

pub const SHIELD_MASTER: [Equipment; 4] = [
    Equipment {
        name: "なし",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "かわのたて",
        price: 90,
        sell: 45,
        attack: 0,
        defense: 4,
    },
    Equipment {
        name: "てつのたて",
        price: 800,
        sell: 400,
        attack: 0,
        defense: 10,
    },
    Equipment {
        name: "みかがみのたて",
        price: 14800,
        sell: 7400,
        attack: 0,
        defense: 20,
    },
];

pub const ITEM_MASTER: [Equipment; 15] = [
    Equipment {
        name: "なし",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "たいまつ",
        price: 8,
        sell: 4,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "せいすい",
        price: 12,
        sell: 6,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "キメラのつばさ",
        price: 24,
        sell: 12,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "りゅうのうろこ",
        price: 20,
        sell: 10,
        attack: 0,
        defense: 2,
    }, // 装備効果あり
    Equipment {
        name: "ようせいのふえ",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "せんしのゆびわ",
        price: 0,
        sell: 0,
        attack: 2,
        defense: 0,
    }, // 装備効果あり
    Equipment {
        name: "ロトのしるし",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "おうじょのあい",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "のろいのベルト",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    }, // 呪いアイテム
    Equipment {
        name: "ぎんのたてごと",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "しのくびかざり",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "たいようのいし",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "あまぐものつえ",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
    Equipment {
        name: "にじのしずく",
        price: 0,
        sell: 0,
        attack: 0,
        defense: 0,
    },
];
