# 🏰 Damdara （ドムドーラ） 🦀

![Rust](https://img.shields.io/badge/made%20with-Rust-red)
![crate](https://img.shields.io/crates/v/damdara.svg)
![docs](https://docs.rs/damdara/badge.svg)
![Forks](https://img.shields.io/github/forks/retrodig/damdara)
![Stars](https://img.shields.io/github/stars/retrodig/damdara)
![License](https://img.shields.io/github/license/retrodig/damdara)

<p align="center">
  <img width="450" src="https://raw.githubusercontent.com/retrodig/damdara/main/assets/images/main_logo_cmp.png">
</p>

Damdara（ドムドーラ） は、ファミコン版『ドラゴンクエスト』の「ふっかつのじゅもん」システムを完全再現しつつ、

プレイヤーの名前によるステータス生成、アイテム装備、戦闘処理などの機能を持つ、Rustで構築可能にしたレトロファンタジー向けコアロジッククレートです。

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

- [x] パラメータから「ふっかつのじゅもん」を生成する（パラメータからパスワード）。
- [x] 「ふっかつのじゅもん」から生成されるパラメータ（パスワードからパラメータ）
- [x] 名前で強化されたパラメータを計算
- [x] モンスター一覧を見る
- [x] モンスターの情報を参照する
- [x] 成長パラメータ一覧参照
- [x] 任意のパラメータを参照
- [x] アイテム一覧を見る
- [x] アイテム項目情報を参照
- [x] ぶき一覧を見る
- [x] ぶきの情報を参照する
- [x] よろい一覧を見る
- [x] よろいの情報を参照する
- [x] たて一覧を見る
- [x] たての情報を参照する
- [x] 戦闘シーンの再現
- [ ] 城、街一覧を見る
- [ ] 城、街情報を参照する
- [ ] 世界のあらゆる情報を参照する
- [ ] メッセージ一覧を見る
- [ ] フィールドを探索する
- [ ] 城、街を探索する

この図は「ふっかつのじゅもん」の120bit構造を表しています。

名前・経験値・ゴールド・アイテム・装備・キーアイテム・フラグ情報などを、厳密にビット単位でパッキングして格納しています。

<p align="center">
  <img width="700" src="https://raw.githubusercontent.com/retrodig/damdara/main/assets/images/base_binary.png">
</p>

こちらの構造より、「ふっかつのじゅもん」を生成するために再配置した構造は以下の通りです。

**ふっかつのじゅもん生成のためのビット構成マッピングテーブル**

 Byte Index | フィールド概要                                       | ビット数             
------------|-----------------------------------------------|------------------
 0          | チェックサム (CRC-8)                                | 8ビット             
 1          | 経験値 (下位8ビット)                                  | 8ビット             
 2          | パターン 3ビット目 + しのくびかざり入手フラグ + 名前の3文字目           | 1ビット, 1ビット, 6ビット 
 3          | アイテム 4つ目 + アイテム 3つ目                           | 4ビット, 4ビット       
 4          | ゴールド (下位8ビット)                                 | 8ビット             
 5          | 名前の1文字目 + ゴーレムを倒したかフラグ + パターン 2ビット目           | 6ビット, 1ビット, 1ビット 
 6          | アイテム 8つ目 + アイテム 7つ目                           | 4ビット, 4ビット       
 7          | パターン 1ビット目 + ドラゴンを倒したかフラグ + 名前の4文字目           | 1ビット, 1ビット, 6ビット 
 8          | ぶき + よろい + たて                                 | 3ビット, 3ビット, 2ビット 
 9          | ゴールド (上位8ビット)                                 | 8ビット             
 10         | かぎの数 + やくそうの数                                 | 4ビット, 4ビット       
 11         | アイテム 6つ目 + アイテム 5つ目                           | 4ビット, 4ビット       
 12         | 経験値 (上位8ビット)                                  | 8ビット             
 13         | りゅうのうろこを装備したかフラグ + 名前の2文字目 + せんしのゆびわを装備したかフラグ | 1ビット, 6ビット, 1ビット 
 14         | アイテム 2つ目 + アイテム 1つ目                           | 4ビット, 4ビット       

<p align="center">
  <img width="700" src="https://raw.githubusercontent.com/retrodig/damdara/main/assets/images/relocation.png">
</p>

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

入力指定がない場合は、デフォルトの勇者が生成されパラメータが表示します。

```
cargo run

player name: ゆうてい
summary: PlayerSummary { name: "ゆうてい", level: 1, hp: 15, mp: 3, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 6, max_hp: 15, max_mp: 3, attack_power: 4, defense_power: 3, weapon: "なし", armor: "なし", shield: "なし" }
```

名前を指定するには -n オプションを指定。

```
cargo run -- -n だい

player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 1, hp: 14, mp: 0, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 4, max_hp: 14, max_mp: 0, attack_power: 4, defense_power: 2, weapon: "なし", armor: "なし", shield: "なし" }
```

オプションを付与することで、パラメータの変更、アイテムの所持、装備の変更など、様々なパラメータが指定可能です。

経験値を200与えた場合、レベルは自動的に反映されます。

```
cargo run -- -n だい -e 200

player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 5, hp: 32, mp: 20, gold: 0, experience: 200 }
strength_status: StrengthStatus { level: 5, strength: 11, agility: 10, max_hp: 32, max_mp: 20, attack_power: 11, defense_power: 5, weapon: "なし", armor: "なし", shield: "なし" }
```

さらに、300ゴールドを与えたい場合は -g オプションを付与。

```
cargo run -- -n だい -e 200 -g 300

player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 5, hp: 32, mp: 20, gold: 300, experience: 200 }
strength_status: StrengthStatus { level: 5, strength: 11, agility: 10, max_hp: 32, max_mp: 20, attack_power: 11, defense_power: 5, weapon: "なし", armor: "なし", shield: "なし" }
```

アイテムの所持を変更したい場合は、対応するアイテムIDをカンマ区切りで渡します。

```
cargo run -- -n だい -i 2,3,4

player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 1, hp: 14, mp: 0, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 4, max_hp: 14, max_mp: 0, attack_power: 4, defense_power: 2, weapon: "なし", armor: "なし", shield: "なし" }
item: ["せいすい", "キメラのつばさ", "りゅうのうろこ", "なし", "なし", "なし", "なし", "なし"]
```

各オプションを指定した後、ID付与で任意の装備をつけることも可能です。

```
cargo run -- -n だい -w 3 -a 5 -s 3

summary: PlayerSummary { name: "た゛い\u{3000}", level: 1, hp: 14, mp: 0, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 4, max_hp: 14, max_mp: 0, attack_power: 14, defense_power: 46, weapon: "どうのつるぎ", armor: "はがねのよろい", shield: "みかがみのたて" }
item: ["なし", "なし", "なし", "なし", "なし", "なし", "なし", "なし"]
```

`flags`は、プレイヤーが特定のアイテムを装備したか、ボスモンスターを倒したかなどを示すフラグのグループです。

これらはコマンドライン引数 `--flags` で5桁のビット列としてまとめて指定できます。

```
cargo run -- -n だい --flags 01010
```

### List of CLI options

| オプション              | 型                        | デフォルト値    | 説明           |
|:-------------------|:-------------------------|:----------|:-------------|
| `-n`, `--name`     | String                   | `"ゆうてい"`  | 主人公の名前       |
| `-e`, `--exp`      | u16                      | `0`       | 経験値          |
| `-g`, `--gold`     | u16                      | `0`       | ゴールド         |
| `-w`, `--weapon`   | u8                       | `0`       | 装備しているぶきのID  |
| `-a`, `--armor`    | u8                       | `0`       | 装備しているよろいのID |
| `-s`, `--shield`   | u8                       | `0`       | 装備しているたてのID  |
| `-i`, `--item`     | Vec<u8>(comma delimited) | なし        | どうぐID一覧      |
| `-y`, `--herbs`    | u8                       | `0`       | やくそうの個数      |
| `-k`, `--keys`     | u8                       | `0`       | かぎの個数        |
| `--flags`          | Flags structure          | All false | ストーリーフラグ     |
| `-p`, `--password` | String                   | 最強強化パスワード | ふっかつのじゅもん    |

### Flags option details（--flags）

| digit position | bit | Field Name          | Description    |
|:--------------:|:---:|:--------------------|:---------------|
|      1桁目       | 0/1 | has_dragon_scale    | りゅうのうろこを装備したか？ |
|      2桁目       | 0/1 | has_warrior_ring    | せんしのゆびわを装備したか? |
|      3桁目       | 0/1 | has_cursed_necklace | しのくびかざりを入手したか? |
|      4桁目       | 0/1 | defeated_dragon     | ドラゴンを倒したか?     |
|      5桁目       | 0/1 | defeated_golem      | ゴーレムを倒したか?     |

- `--flags 01000`のように5桁の0/1で指定します。
- 指定がない場合、デフォルトは `00000`となります。 (すべてfalse)

### Mode

モードは `--mode` またはショートカット `-m` で指定できます。

```
cargo run -- --mode <input>
cargo run -- -m <input>
```

 モード名    | 説明                                      
---------|-----------------------------------------|
 start   | 名前のみから強化パラメーターを計算                       |                  
 save    | 任意のパラメータ（Expやゴールドなど）から「ふっかつのじゅもん」を生成する。 |                  
 load    | 「ふっかつのじゅもん」から、名前／パラメータなどを復元             |
 display | マスターデータの一覧表示と個別表示                       |
 battle  | 任意のモンスターと戦闘を行える                         |

**■ Start Mode**

勇者の名前からパラメータを生成します。（LV.1）

**example**

```
player name: た゛い
summary: PlayerSummary { name: "た゛い\u{3000}", level: 1, hp: 14, mp: 0, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 4, max_hp: 14, max_mp: 0, attack_power: 4, defense_power: 2, weapon: "なし", armor: "なし", shield: "なし" }
```

start モードは省略可能で、省略するとデフォルトの名前が付与された勇者が生成されます。

**example**

```
cargo run

player name: ゆうてい
summary: PlayerSummary { name: "ゆうてい", level: 1, hp: 15, mp: 3, gold: 0, experience: 0 }
strength_status: StrengthStatus { level: 1, strength: 4, agility: 6, max_hp: 15, max_mp: 3, attack_power: 4, defense_power: 3, weapon: "なし", armor: "なし", shield: "なし" }
```

**■ Save Mode**

主人公のパラメータに基づいた「ふっかつのじゅもん」を生成します。

<img width="400" src="https://raw.githubusercontent.com/retrodig/damdara/main/assets/images/fukkatsu_no_jumon.png">

**example**

```
cargo run -- -n だい -m save

ぢばげぞでぶいまももれぎざぞでぶいよごぜ
```

上記で説明したオプションを与え、パラメータを変更したうえで、「ふっかつのじゅもん」を生成できます。

```
cargo run -- -n だい -e 7000 -m save

きがよわげずぢなののみやりわげずてだいか
```

**■ Load Mode**

`-m load` オプションを付与することによって、ふっかつのじゅもんを元に勇者を生成することも可能です。

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

**■ Battle Mode**

さあ、勇者を誕生させたのなら、戦いに向かおう。

コマンドを入力し、スライムを倒せ！

```
cargo run -- -m battle

スライムがあらわれた！

ゆうてい HP: 15
スライム HP: 3

--- ゆうていのターン ---
コマンド？
1: たたかう
2: じゅもん
3: どうぐ
4: にげる
```

もしやられてしまったのならば、最強装備で再び挑戦しましょう！

```
cargo run -- -n だい -o max -m battle

スライムがあらわれた！

た゛い　 HP: 190
スライム HP: 3

--- た゛い　のターン ---
コマンド？
1: たたかう
2: じゅもん
3: どうぐ
4: にげる
```

`--view`オプション付与し、敵のidを指定することで、どんなモンスターとも戦うことができます。

最後のボス「りゅうおう」ともすぐに戦えます。

```
cargo run -- -n だい -o max -m battle --view 39

りゅうおうがあらわれた！

た゛い　 HP: 190
りゅうおう HP: 129

--- た゛い　のターン ---
コマンド？
1: たたかう
2: じゅもん
3: どうぐ
4: にげる
```

強い炎に注意。

```
 りゅうおうは  ほのおをはいた!
 た゛い　は 44ポイントの
 ダメージを うけた

た゛い　 HP: 146
りゅうおう HP: 129
```

### Status Option

ステータスのオプションを指定するには `--option` またはショートカット `-o` を使用します。

```
cargo run -- -o <input>
```

maxが指定された場合、パラメータとストーリーの状態は最強となります。

<img width="400" src="https://raw.githubusercontent.com/retrodig/damdara/main/assets/images/strongest_parameters.png">

**example**

```
cargo run -- -o max
player name: ゆうてい
summary: PlayerSummary { name: "ゆうてい", level: 30, hp: 15, mp: 3, gold: 65535, experience: 65535 }
strength_status: StrengthStatus { level: 30, strength: 140, agility: 120, max_hp: 210, max_mp: 183, attack_power: 182, defense_power: 110, weapon: "ロトのつるぎ", armor: "ロトのよろい", shield: "みかがみのたて" }
```

### Format Option

出力形式は `--format` を与えることで変更できます。

```
cargo run -- --format <input>
```

 フォーマット | 説明                         
--------|----------------------------|
 print  | プリント文が出力される。これはデフォルトの設定です。 |                  
 json   | JSON形式での出力                 |          

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

## 他の言語

- [English version is here](./README.md)

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