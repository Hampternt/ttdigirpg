# ttdigirpg - Digital TTRPG Exploration

## The Goal

This project explores what digital computers can do for TTRPGs - similar to how Hearthstone evolved card games by leveraging computational power.

**The core question:** How can we use digital tools to make TTRPGs more fun while preserving what makes traditional tabletop gaming great?

## The Hearthstone Parallel

Hearthstone didn't just digitize Magic: The Gathering - it asked "what becomes possible when a computer handles the mechanics?" The result: random card generation, complex triggered effects, simultaneous resolution, and mechanics that would be impossible to track with physical cards.

This project asks the same question for TTRPGs: What becomes possible when we let computers handle the heavy lifting?

## Design Principles

**Leveraging computational advantages:**
- Handle complex systems that would bog down tabletop play (detailed combat, vehicle physics, economy simulation)
- Maintain persistent world state across sessions without spreadsheets
- Calculate dozens of modifiers and conditions in real-time
- Enable emergent complexity from simple, stackable rules

**Preserving TTRPG strengths:**
- Keep mechanics easy to understand and modify (homebrew friendly)
- Maintain transparent systems players can grasp intuitively
- Allow flexibility to change rules on the fly
- Focus on narrative and player agency, not fighting with systems

## Current Implementation

A Rust terminal application using World of Darkness-inspired mechanics as a testbed. The specific system is just a vehicle for experimentation.

**Features:**
- Character system with attributes and skills (1-5 dot scale, grounded in realistic human capability)
- SQLite database for persistent character and game data
- Object and inventory management with relational tracking
- UUID-based character identification for cross-system uniqueness
- Referential integrity with automatic cascade deletion

## Running the Project

```bash
cargo build
cargo run
cargo test
```
