# Damdara Web Example

This is a web-based demo of the Damdara WASM module, showcasing the Dragon Quest password system and battle mechanics.

🌐 **Live Demo**: This example is hosted on GitHub Pages for easy access.

## Features

- **プレイヤー作成** - Create a new player with a custom name
- **復活の呪文** - Generate and load 20-character hiragana passwords
- **ステータス表示** - View detailed player statistics
- **戦闘システム** - Battle against monsters with queued actions
- **マスターデータ** - Browse monsters, weapons, armor, and spells

## Running the Example

### Prerequisites

1. Build the WASM package first (from the project root):
   ```bash
   # From the project root directory
   ./build-wasm.sh dev
   ```

   This will:
   - Build the WASM package to `pkg/`
   - Automatically copy it to `docs/pkg/` for local development and GitHub Pages

### Start the Web Server

You can use any static file server. Here are a few options:

**Python 3:**
```bash
python -m http.server 8080
```

**Python 2:**
```bash
python -m SimpleHTTPServer 8080
```

**Node.js (http-server):**
```bash
npx http-server -p 8080
```

**PHP:**
```bash
php -S localhost:8080
```

### Access the Application

Open your browser and navigate to:
```
http://localhost:8080
```

## Usage Guide

### Creating a Player

1. Enter a name (up to 8 characters, Japanese supported)
2. Click "作成" (Create)
3. Your player stats will appear in the "プレイヤー情報" section

### Password System

**Generate Password:**
1. Create or load a player
2. Click "生成" (Generate) in the password section
3. The 20-character hiragana password will appear

**Load from Password:**
1. Paste or type a password in the text field
2. Click "ロード" (Load)
3. Your player will be restored from the password

### Battle System

1. Create a player first
2. Select a monster from the dropdown
3. Click "戦闘開始" (Start Battle)
4. The battle will run automatically with pre-queued attack actions
5. Battle log and results will appear below

**Note:** The current implementation uses a simple attack-only strategy. In a real game, you would implement interactive turn-by-turn combat.

### Viewing Master Data

Click the buttons in the "マスターデータ" section to view:
- **モンスター** - All 40 monsters with stats
- **武器** - All weapons and their attributes
- **防具** - All armor pieces
- **呪文** - All spells and their effects

## Architecture

```
docs/
├── index.html    # Main HTML structure
├── style.css     # Dragon Quest-themed styling
├── app.js        # JavaScript application logic
├── pkg/          # WASM package (copied from ../pkg/)
│   ├── damdara.js
│   ├── damdara_bg.wasm
│   └── damdara.d.ts
└── README.md     # This file
```

**Note**: This directory is served by GitHub Pages, making the demo accessible online.

## Browser Compatibility

This example requires a modern browser with:
- WebAssembly support
- ES6 modules support
- Fetch API

Tested on:
- Chrome/Edge 90+
- Firefox 89+
- Safari 15+

## Development

To modify the example:

1. Edit the source files (index.html, style.css, app.js)
2. Refresh your browser (no build step needed for the web files)
3. If you modify Rust code, rebuild with `./build-wasm.sh`

## Limitations

- Battle is fully automated (no turn-by-turn interaction yet)
- No inventory management UI
- No map/exploration system
- Read-only master data display

## Future Enhancements

- Interactive turn-by-turn battles
- Spell and item selection during battle
- Equipment management interface
- Save/load system using localStorage
- Character progression visualization
- Mobile-responsive design improvements
