# ttdigirpg - Digital TTRPG Exploration

## The Goal

This project explores what digital computers can do for TTRPGs - similar to how Hearthstone evolved card games by leveraging computational power.

**The core question:** How can we use digital tools to make TTRPGs more fun while preserving what makes traditional tabletop gaming great?

## The Hearthstone Parallel

Hearthstone didn't just digitize Magic: The Gathering - it asked "what becomes possible when a computer handles the mechanics?" The result: random card generation, complex triggered effects, simultaneous resolution, and mechanics that would be impossible to track with physical cards.

This project asks the same question for TTRPGs: What becomes possible when we let computers handle the heavy lifting?

## What We're Exploring

**Computational advantages:**
- Complex systems that would bog down tabletop play (detailed combat, vehicle physics, economy simulation)
- Persistent world state across sessions without spreadsheets
- Real-time calculation of dozens of modifiers and conditions
- Emergent complexity from simple, stackable rules

**Preserving TTRPG strengths:**
- Easy to understand and modify (homebrew friendly)
- Transparent mechanics players can grasp intuitively
- Flexibility to change rules on the fly
- Focus on narrative and player agency, not fighting with systems

## Current Implementation

Building a Rust terminal application with World of Darkness-inspired mechanics as a testbed. The specific system doesn't matter - it's a vehicle for experimentation.

Right now: Basic character system with attributes and skills (1-5 dot scale, grounded in realistic human capability).

## Running the Project

```bash
cargo build
cargo run
cargo test
```

## What's Next

Experimenting with systems that benefit most from computational power:
- Combat with positioning, environmental effects, and complex interactions
- Sandbox world management
- Economic systems
- Vehicle mechanics

The goal isn't to build a specific game - it's to discover what digital tools unlock for TTRPG design.e 
