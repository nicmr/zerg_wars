# zerg_wars
A game AI experiment about two zerg tribes fighting for control over a planet. local 1v1 works,  1vAI / AIvAI is WIP.

Built on top of ggez.


## Dependencies
- An up to date Rust install
- Linux: OpenSSL (installed by default on most distributions)

## How to build
Standard cargo build process.
The game will check for required sprites on launch and download all required ones from imgur, so you need an internet connection for the initial launch.

```bash
git clone https://github.com/nicmr/zerg_wars.git
cd zerg_wars
cargo run --release
```



## How to play

Spend your resources, counter your opponent's army composition and destroy the opposing base to win!

Controls
| Command | Hotkey Player1 | Hotkey Player2 |
| --- | --- | --- |
| Spawn Zergling | 1 | Numpad 1 |
| Spawn Hydra | 2 | Numpad 2 |
| Spawn Baneling | 3 | Numpad 3|

Unit Stats

| Unit | HP | Damage | Speed | Range | Targets|
| --- | --- | --- | --- | --- | --- |
| Zergling | Low | Low | High | Melee | Single |
| Hydra | Average | High | Average | Long range | Single |
| Baneling | Average | High | Low | Short range | AoE |


## How the AI works
WIP



## Quick source file reference
- `constants.rs`: constants that scale game speed, movement speed, damage and map scale
- `gameobject.rs`: the various units types and bases found in the game
- `gamestate.rs`: the global game state & event handlers
- `main.rs`: entry point for the program, asset fetching, game launch
- `player.rs`: human and AI-player related code
- `traits.rs`: traits used in the other source files