# Game Simulator

A simple text-based battle simulator written in Rust where a hero and monster engage in turn-based combat.

## Features

- 🦸‍♂️ Character system with:
  - Health points
  - Attack power
  - Defense stats
  - Name customization
- ⚔️ Combat actions:
  - Attack: Deal damage based on attack power and target's defense
  - Defense: Increase defense stats
  - Heal: Restore health points
- 🎮 Turn-based gameplay
- 💫 Interactive command-line interface
- 🛡️ Dynamic battle mechanics

## Game Mechanics

Each character has the following attributes:
- Health Points (HP)
- Attack Power
- Defense Rating
- Status tracking (alive/defeated)

### Available Actions

1. **Attack** 
   - Deals damage based on attacker's power minus defender's defense
   - If damage is less than defense, attack is blocked

2. **Defense**
   - Increases character's defense by 5 points
   - Useful for blocking powerful attacks

3. **Heal**
   - Restores 10 health points
   - Can be used strategically during battle

## How to Play

1. Start the game:
```bash
cargo run
```

2. Each turn, choose an action:
   - 1: Attack
   - 2: Defense
   - 3: Heal

3. Battle continues until either:
   - Hero is defeated
   - Monster is defeated

## Example Battle

```
hero health: 100
monster health: 80
Choose an action: 1. Attack 2. Defence 3. Heal for hero
1
monster health: 65
Choose an action: 1. Attack 2. Defence 3. Heal for monster
```

## Building from Source

```bash
git clone <repository-url>
cd game-simulator
cargo build
cargo run
```

## Requirements

- Rust (Latest stable version)
- Cargo (Rust's package manager)

## Future Improvements

- Add more character types
- Implement special abilities
- Add inventory system
- Include multiple monsters
- Add experience and leveling system