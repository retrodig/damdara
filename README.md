# 🏰 Damdara 🦀

![Rust](https://img.shields.io/badge/made%20with-Rust-red)
![crate](https://img.shields.io/crates/v/damdara.svg)
![docs](https://docs.rs/damdara/badge.svg)
![Forks](https://img.shields.io/github/forks/retrodig/damdara)
![Stars](https://img.shields.io/github/stars/retrodig/damdara)
![License](https://img.shields.io/github/license/retrodig/damdara)

<p align="center">
  <img width="450" src="https://raw.githubusercontent.com/retrodig/damdara/main/assets/images/main_logo_cmp.png">
</p>

Damdara is a logic crate for retro fantasy that can be built in Rust, while fully reproducing the "Fukkatsu no Jumon"
system of the NES version of Dragon Quest,
Damdara is a logic crate for retro-fantasy, which can be built in Rust for elements such as status generation based on
the player's name, item equipping, and battle processing.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
    - [Cargo](#cargo)
    - [Brew](#brew)
- [Usage](#usage)
    - [Mode](#mode)
    - [Status Option](#status-option)
    - [Format Option](#format-option)
- [Additional Resources](#additional-resources)
- [Contributing to Damdara](#contributing-to-Damdara)
- [References](#references)
- [License](#license)
- [Author](#author)

## Features

- [x] Generate resurrection "Fukkatsu no Jumon" from parameters （Parameters to Password）
- [x] Parameters generated from resurrection "Fukkatsu no Jumon" (Password to Parameters)
- [x] Calculate parameters enhanced by name
- [x] See list of monsters
- [x] Refer to any monster information
- [x] See parameter list
- [x] Refer to any parameter
- [x] See list of Item
- [x] Refer to any Item information
- [x] See list of Weapon
- [x] Refer to any Weapon information
- [x] See list of Armor
- [x] Refer to any Armor information
- [x] See list of Shield
- [x] Refer to any Shield information
- [ ] Reproduction of battle scenes
- [ ] See list of Town
- [ ] Refer to any Town information
- [ ] Refer to any World information
- [ ] See list of Message
- [ ] Explore the field
- [ ] Explore the Town

Bit configuration mapping table for password generation.

 Byte Index | Field Description                               | Bits (from MSB to LSB) 
------------|-------------------------------------------------|------------------------
 0          | Checksum (CRC-8)                                | [7:0]                  
 1          | Experience (lower 8 bits)                       | [7:0]                  
 2          | Pattern[2] (1) + Necklace (1) + Name[2] (6)     | [7], [6], [5:0]        
 3          | Item[3] + Item[2]                               | [7:4], [3:0]           
 4          | Gold (lower 8 bits)                             | [7:0]                  
 5          | Name[0] (6) + Golem (1) + Pattern[1] (1)        | [7:2], [1], [0]        
 6          | Item[7] + Item[6]                               | [7:4], [3:0]           
 7          | Pattern[0] + Dragon (1) + Name[3] (6)           | [7], [6], [5:0]        
 8          | Weapon (3) + Armor (3) + Shield (2)             | [7:5], [4:2], [1:0]    
 9          | Gold (upper 8 bits)                             | [7:0]                  
 10         | Keys + Herbs                                    | [7:4], [3:0]           
 11         | Item[5] + Item[4]                               | [7:4], [3:0]           
 12         | Experience (upper 8 bits)                       | [7:0]                  
 13         | DragonScale (1) + Name[1] (6) + WarriorRing (1) | [7], [6:1], [0]        
 14         | Item[1] + Item[0]                               | [7:4], [3:0]           

## Installation

### Cargo

```
cargo install damdara
```

**Add to project**

```
cargo add damdara
```

### Brew

```
brew tap webcyou-org/tap
brew install damdara
```

## Usage

```
cargo run <input>
```

or

```
damdara <input>
```

If input is not present, the default brave value is returned.

```
cargo run

player name: ゆうてい
summary: PlayerSummary { name: "ゆうてい", level: 1, hp: 15, mp: 3, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 6, max_hp: 15, max_mp: 3, attack_power: 4, defense_power: 3, weapon: "なし", armor: "なし", shield: "なし" }
```

The name can be specified by giving -n

```
cargo run -- -n だい

player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 1, hp: 14, mp: 0, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 4, max_hp: 14, max_mp: 0, attack_power: 4, defense_power: 2, weapon: "なし", armor: "なし", shield: "なし" }
```

By granting options, you can change parameters, possess items, change equipment, and do many other things.

If you want to give 200 experience. The level is automatically reflected.

```
cargo run -- -n だい -e 200

player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 5, hp: 32, mp: 20, gold: 0, experience: 200 }
strength_status: StrengthStatus { level: 5, strength: 11, agility: 10, max_hp: 32, max_mp: 20, attack_power: 11, defense_power: 5, weapon: "なし", armor: "なし", shield: "なし" }
```

Furthermore, if you wish to grant 300 Gold.

```
cargo run -- -n だい -e 200 -g 300

player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 5, hp: 32, mp: 20, gold: 300, experience: 200 }
strength_status: StrengthStatus { level: 5, strength: 11, agility: 10, max_hp: 32, max_mp: 20, attack_power: 11, defense_power: 5, weapon: "なし", armor: "なし", shield: "なし" }
```

If you want to change the item possession, pass the corresponding item IDs separated by commas.

```
cargo run -- -n だい -i 2,3,4

player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 1, hp: 14, mp: 0, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 4, max_hp: 14, max_mp: 0, attack_power: 4, defense_power: 2, weapon: "なし", armor: "なし", shield: "なし" }
item: ["せいすい", "キメラのつばさ", "りゅうのうろこ", "なし", "なし", "なし", "なし", "なし"]
```

The equipment is also given an ID after specifying each option.

```
cargo run -- -n だい -w 3 -a 5 -s 3

summary: PlayerSummary { name: "た゛い\u{3000}", level: 1, hp: 14, mp: 0, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 4, max_hp: 14, max_mp: 0, attack_power: 14, defense_power: 46, weapon: "どうのつるぎ", armor: "はがねのよろい", shield: "みかがみのたて" }
item: ["なし", "なし", "なし", "なし", "なし", "なし", "なし", "なし"]
```

`flags` are a group of flags that indicate whether a player has equipped a particular item or defeated a boss monster.

They can be specified collectively as a 5-digit bit string with the command line argument `--flags`.

```
cargo run -- -n だい --flags 01010
```

### List of CLI options

| option             | type                     | default value                 | Description                                    |
|:-------------------|:-------------------------|:------------------------------|:-----------------------------------------------|
| `-n`, `--name`     | String                   | `"ゆうてい"`                      | Main character's name                          |
| `-e`, `--exp`      | u16                      | `0`                           | XP                                             |
| `-g`, `--gold`     | u16                      | `0`                           | Gold in possession                             |
| `-w`, `--weapon`   | u8                       | `0`                           | The number of the weapon you are equipped with |
| `-a`, `--armor`    | u8                       | `0`                           | The number of the armor you are equipped with  |
| `-s`, `--shield`   | u8                       | `0`                           | The number of the shield you are equipped with |
| `-i`, `--item`     | Vec<u8>(comma delimited) | not in possession             | List of item numbers                           |
| `-y`, `--herbs`    | u8                       | `0`                           | Number of herbs held                           |
| `-k`, `--keys`     | u8                       | `0`                           | Number of keys held                            |
| `--flags`          | Flags structure          | All false                     | status flag                                    |
| `-p`, `--password` | String                   | Maximum Strengthened Password | Fukkatsu no Jumon                              |

### Flags option details（--flags）

| digit position | bit | Field Name          | Description                             |
|:--------------:|:---:|:--------------------|:----------------------------------------|
|   1st digit    | 0/1 | has_dragon_scale    | Equipped with the scales of a dragon?   |
|   2st digit    | 0/1 | has_warrior_ring    | Are you equipped with a warrior's ring? |
|   3st digit    | 0/1 | has_cursed_necklace | Did you get the beak necklace?          |
|   4st digit    | 0/1 | defeated_dragon     | You slayed the dragon.                  |
|   5st digit    | 0/1 | defeated_golem      | You beat the golem.                     |

- Specify with **5 digits 0/1** like `--flags 01000`.
- If not specified, default `“00000”` (all false)

### Mode

You can specify the mode by giving `--mode` or the shortcut `-m`.

```
cargo run -- --mode <input>
cargo run -- -m <input>
```

 Mode Name | Description                                                                   
-----------|-------------------------------------------------------------------------------|
 start     | Calculate enhanced parameters from name only                                  |                  
 save      | Generate "Fukkatsu no Jumon" from arbitrary parameters (e.g. exp, gold, etc.) |                  
 load      | Restore name/parameters from the "Fukkatsu no Jumon"                          |
 display   | Supports list and individual display of master data                           |

**■ Start Mode**

Generate parameters from the names of brave men

**example**

```
player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 1, hp: 14, mp: 0, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 4, max_hp: 14, max_mp: 0, attack_power: 4, defense_power: 2, weapon: "なし", armor: "なし", shield: "なし" }
```

The default setting of the mode is here, so if you omit it, the default brave will be generated

**example**

```
cargo run

player name: ゆうてい
summary: PlayerSummary { name: "ゆうてい", level: 1, hp: 15, mp: 3, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 6, max_hp: 15, max_mp: 3, attack_power: 4, defense_power: 3, weapon: "なし", armor: "なし", shield: "なし" }
```

**■ Save Mode**

Generates the "Fukkatsu no Jumon" from the parameters of the hero.

**example**

```
cargo run -- -n だい -m save

ぢばげぞでぶいまももれぎざぞでぶいよごぜ
```

After giving the options explained above and changing the parameters, the Fufutsu no Jumon can be generated.

```
cargo run -- -n だい -e 7000 -m save

きがよわげずぢなののみやりわげずてだいか
```

**■ Load Mode**

Generates a brave man from the "Fukkatsu no Jumon" of fortune.

<img width="450" src="https://raw.githubusercontent.com/retrodig/damdara/main/assets/images/fukkatsu_no_jumon.png">

**example**

```
cargo run -- -m load -p ぢばげぞでぶいまももれぎざぞでぶいよごぜ

new_player from Password
player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 1, hp: 14, mp: 0, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 4, max_hp: 14, max_mp: 0, attack_power: 4, defense_power: 2, weapon: "なし", armor: "なし", shield: "なし" }
```

**■ Display Mode**

```
cargo run -- -m display --view item
[Equipment { name: "なし", price: 0, sell: 0, attack: 0, defense: 0 }, Equipment { name: "たいまつ", price: 8, sell: 4, attack: 0, defense: 0 }, Equipment { name: "せいすい", price: 12, sell: 6, attack: 0, defense: 0 }, Equipment { name: "キメラのつばさ", price: 24, sell: 12, attack: 0, defense: 0 }, Equipment { name: "りゅうのうろこ", price: 20, sell: 10, attack: 0, defense: 2 }, Equipment { name: "ようせいのふえ", price: 0, sell: 0, attack: 0, defense: 0 }, Equipment { name: "せんしのゆびわ", price: 0, sell: 0, attack: 2, defense: 0 }, Equipment { name: "ロトのしるし", price: 0, sell: 0, attack: 0, defense: 0 }, Equipment { name: "おうじょのあい", price: 0, sell: 0, attack: 0, defense: 0 }, Equipment { name: "のろいのベルト", price: 0, sell: 0, attack: 0, defense: 0 }, Equipment { name: "ぎんのたてごと", price: 0, sell: 0, attack: 0, defense: 0 }, Equipment { name: "しのくびかざり", price: 0, sell: 0, attack: 0, defense: 0 }, Equipment { name: "たいようのいし", price: 0, sell: 0, attack: 0, defense: 0 }, Equipment { name: "あまぐものつえ", price: 0, sell: 0, attack: 0, defense: 0 }, Equipment { name: "にじのしずく", price: 0, sell: 0, attack: 0, defense: 0 }]
```

```
cargo run -- -m display --view item 5
item: Equipment { name: "ようせいのふえ", price: 0, sell: 0, attack: 0, defense: 0 }
```

**Display Patterns**

- item
- weapon
- armor
- shield
- status
- monster
- town
- message

### Status Option

`--option` or the shortcut `-o` can be used to specify status options

```
cargo run -- -o <input>
```

If max is specified, the parameter and story state will be the strongest

<img width="450" src="https://raw.githubusercontent.com/retrodig/damdara/main/assets/images/strongest_parameters.png">


**example**

```
cargo run -- -o max
player name: ゆうてい
summary: PlayerSummary { name: "ゆうてい", level: 30, hp: 15, mp: 3, gold: 65535, experience: 65535 }
strength_status: StrengthStatus { level: 30, strength: 140, agility: 120, max_hp: 210, max_mp: 183, attack_power: 182, defense_power: 110, weapon: "ロトのつるぎ", armor: "ロトのよろい", shield: "みかがみのたて" }
```

### Format Option

The output format can be changed by giving the `--format`.

```
cargo run -- --format <input>
```

 Format Name | Description                                                       
-------------|-------------------------------------------------------------------|
 print       | It will be a print statement output. This is the default setting. |                  
 json        | Output in JSON format                                             |          

### Example

## Additional Resources

## Contributing to Damdara

To contribute to **Damdara**, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`.
3. Make your changes and commit them: `git commit -m '<commit_message>'`.
4. Push your changes to your branch: `git push origin <branch_name>`.
5. Create a pull request.

Alternatively, consult
the [GitHub documentation](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests) on how to create a
pull request.

## References

- [名前による成長率タイプの変化](https://way78.com/dq1/fc/name.html)
- [DQ1の「ふっかつのじゅもん」の仕組みを全解説](https://qiita.com/musemyuzu/items/eb08f7790df356434e0f?utm_source=pocket_shared)
- [復活の呪文を解析してみたよ](https://qiita.com/yoshi389111/items/29ade2f62483e9c095d9)
- [ふっかつのじゅもんって何？基本の使い方から仕組み・原理まで、動画一本で全てわかる！【FC】【ゆっくり解説】](https://youtu.be/a15mmjJqQKo?si=zJ2SahsbcKoeZSSP)
- [ドラゴンクエスト 攻略・解析](https://gcgx.games/dq1/)
- [ドラゴンクエスト (FC) 復活の呪文計算機](https://taotao54321.github.io/DQ1PasswordCalc/)
- [主人公の名前と能力成長の基礎 FCDQ1](https://dqff.sakura.ne.jp/dq1fc/data/lvup-name.html)
- [DQ1(FC版)強さ判断プログラム](https://sutton-kyouwa.com/cgi-bin/dq1.cgi)

## License

This project is licensed under the MIT License. See the [LICENSE](/LICENSE) file for details.

All monsters, images and other copyrights belong to Square Enix.

## Author

**Daisuke Takayama**

- [@webcyou](https://twitter.com/webcyou)
- [@panicdragon](https://twitter.com/panicdragon)
- <https://github.com/webcyou>
- <https://github.com/webcyou-org>
- <https://github.com/panicdragon>
- <https://www.webcyou.com/>