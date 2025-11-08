// Import the WASM module
import init, { WasmGame } from './pkg/damdara.js';

let game;

// Initialize WASM module
async function initWasm() {
    try {
        await init();
        game = new WasmGame();
        console.log('WASM module loaded successfully');
        updateStatus();
    } catch (error) {
        console.error('Failed to load WASM module:', error);
        document.getElementById('playerStatus').innerHTML =
            '<p class="error">WASM モジュールの読み込みに失敗しました</p>';
    }
}

// Create a new player
window.createPlayer = function() {
    const name = document.getElementById('playerName').value.trim();
    if (!name) {
        alert('名前を入力してください');
        return;
    }

    try {
        const playerState = game.create_player(name);
        console.log('Player created:', playerState);
        updateStatus();
        document.getElementById('playerName').value = '';
    } catch (error) {
        console.error('Error creating player:', error);
        alert('プレイヤー作成エラー: ' + error);
    }
};

// Generate password from current player
window.generatePassword = function() {
    try {
        const password = game.generate_password();
        document.getElementById('password').value = password;
        document.getElementById('passwordOutput').innerHTML =
            `<p class="success">復活の呪文: ${password}</p>`;
    } catch (error) {
        console.error('Error generating password:', error);
        document.getElementById('passwordOutput').innerHTML =
            `<p class="error">エラー: ${error}</p>`;
    }
};

// Load player from password
window.loadFromPassword = function() {
    const password = document.getElementById('password').value.trim();
    if (!password) {
        alert('呪文を入力してください');
        return;
    }

    try {
        const playerState = game.load_from_password(password);
        console.log('Player loaded:', playerState);
        updateStatus();
        document.getElementById('passwordOutput').innerHTML =
            '<p class="success">プレイヤーをロードしました</p>';
    } catch (error) {
        console.error('Error loading from password:', error);
        document.getElementById('passwordOutput').innerHTML =
            `<p class="error">ロードエラー: ${error}</p>`;
    }
};

// Update player status display
function updateStatus() {
    const statusDiv = document.getElementById('playerStatus');

    if (!game.has_player()) {
        statusDiv.innerHTML = '<p>プレイヤーを作成してください</p>';
        return;
    }

    try {
        const state = game.get_player_state();
        const summary = state.summary;
        const strength = state.strength_status;

        statusDiv.innerHTML = `
            <p><strong>なまえ:</strong> ${summary.name}</p>
            <p><strong>レベル:</strong> ${summary.level}</p>
            <p><strong>HP:</strong> ${summary.hp} / ${strength.max_hp}</p>
            <p><strong>MP:</strong> ${summary.mp} / ${strength.max_mp}</p>
            <p><strong>ちから:</strong> ${strength.strength}</p>
            <p><strong>すばやさ:</strong> ${strength.agility}</p>
            <p><strong>こうげき力:</strong> ${strength.attack_power}</p>
            <p><strong>ぼうぎょ力:</strong> ${strength.defense_power}</p>
            <p><strong>ゴールド:</strong> ${summary.gold} G</p>
            <p><strong>けいけんち:</strong> ${summary.experience}</p>
            <p><strong>武器:</strong> ${strength.weapon}</p>
            <p><strong>防具:</strong> ${strength.armor}</p>
            <p><strong>盾:</strong> ${strength.shield}</p>
        `;
    } catch (error) {
        console.error('Error getting player state:', error);
        statusDiv.innerHTML = '<p class="error">ステータス取得エラー</p>';
    }
}

// Start a battle
window.startBattle = function() {
    if (!game.has_player()) {
        alert('まずプレイヤーを作成してください');
        return;
    }

    const monsterId = parseInt(document.getElementById('monsterId').value);
    const battleLog = document.getElementById('battleLog');

    try {
        // Clear previous input
        game.clear_battle_input();

        // Queue multiple attack actions (simple AI)
        for (let i = 0; i < 20; i++) {
            game.queue_battle_action('attack');
        }

        // Run the battle
        const result = game.run_battle(monsterId);
        console.log('Battle result:', result);

        // Display messages
        battleLog.textContent = result.messages.join('\n');

        // Update player status after battle
        updateStatus();

        // Show battle outcome
        let outcome = '\n\n=== 戦闘結果 ===\n';
        if (result.player_survived && result.monster_defeated) {
            outcome += '勝利！';
        } else if (result.player_escaped) {
            outcome += 'にげだした！';
        } else if (!result.player_survived) {
            outcome += '敗北...';
        } else if (result.monster_escaped) {
            outcome += 'モンスターが逃げた';
        }
        battleLog.textContent += outcome;

    } catch (error) {
        console.error('Battle error:', error);
        battleLog.textContent = `戦闘エラー: ${error}`;
    }
};

// Show master data
window.showMasterData = function(type) {
    const display = document.getElementById('masterDataDisplay');

    try {
        let data, headers, formatRow;

        switch (type) {
            case 'monsters':
                data = game.get_monsters();
                headers = ['ID', '名前', 'HP', '攻撃力', '守備力', '経験値', 'ゴールド'];
                formatRow = (m) => [m.id, m.name, m.hp, m.attack, m.defense, m.exp, m.gold];
                break;

            case 'weapons':
                data = game.get_weapons();
                headers = ['ID', '名前', '価格', '売値', '攻撃力'];
                formatRow = (w) => [w.id, w.name, w.price, w.sell, w.attack];
                break;

            case 'armors':
                data = game.get_armors();
                headers = ['ID', '名前', '価格', '売値', '守備力'];
                formatRow = (a) => [a.id, a.name, a.price, a.sell, a.defense];
                break;

            case 'spells':
                data = game.get_spells();
                headers = ['ID', '呪文名', '習得Lv', 'MP', '説明'];
                formatRow = (s) => [s.id, s.name, s.learn_level, s.mp_cost, s.description];
                break;

            default:
                display.innerHTML = '<p>データタイプが不正です</p>';
                return;
        }

        // Create table
        let html = '<table><thead><tr>';
        headers.forEach(h => html += `<th>${h}</th>`);
        html += '</tr></thead><tbody>';

        data.forEach(item => {
            html += '<tr>';
            formatRow(item).forEach(cell => html += `<td>${cell}</td>`);
            html += '</tr>';
        });

        html += '</tbody></table>';
        display.innerHTML = html;

    } catch (error) {
        console.error('Error loading master data:', error);
        display.innerHTML = `<p class="error">データ読み込みエラー: ${error}</p>`;
    }
};

// Initialize on page load
initWasm();
