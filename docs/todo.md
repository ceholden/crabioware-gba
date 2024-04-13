# TODO

<!-- vim-markdown-toc GFM -->

* [Common Concerns](#common-concerns)
* [Games](#games)
    * [Pong](#pong)
    * [Snake](#snake)
    * [Artillery](#artillery)
    * [Flappy Bird](#flappy-bird)
    * [Asteroids](#asteroids)
    * [Schmup](#schmup)
    * [Platformer](#platformer)
    * [PacCrab](#paccrab)
    * [Space Invaders](#space-invaders)
    * [Brawler](#brawler)

<!-- vim-markdown-toc -->

## Common Concerns

* [ ] Start screen
* [x] Game select
    * [x] MVP game selector
    * [ ] Game selector keeps the game you selected
* [ ] Micro-game sequencer
* [ ] Difficulty selection
* [ ] Saves
    * [ ] High score counter
    * [ ] Remembers preferences
* [x] Game over screen
    * MVP game over screen
* [ ] Background layer for UI elements (score/health/etc)
* [x] Sprites
    * Snake
* [x] Affine sprites
    * Pong
* [ ] Tilemap editor -> game art pipeline
    * [Tiled](https://www.mapeditor.org/) (?)
    * [Sprite Fusion](https://www.spritefusion.com/) (?)
    * [LDTK](https://ldtk.io/) (?)
* [ ] Background scrolling tilemap
* [ ] Collision with tilemap
* [ ] Collision detection
    * [ ] Rectangles
    * [ ] Circles
    * [ ] SAT
    * [ ] Rays
    * [ ] Continuous collision detection
    * [ ] Broad phase optimizations
* [ ] Collision resolution
* [ ] Games as crates in workspace, for fun
* [ ] Timed mode (instead of after win/loss)
* [ ] Rogueification
    * (?) could powerups transfer across mini-games?
        * e.g., speed -> faster paddle, snake brakes, higher turn rate, etc

## Games

### Pong

Crabs ping ponging back and forth

TODO:

* [ ] Involve A/B buttons somehow
* [ ] L/R change impact angle?
    * Blocked by not having oriented bounding box (OBB) collision

### Snake

Snake.. eats crabs?

TODO:

* [ ] More difficulty ~ mechanics
    * [ ] Spawn berries with different nutrition content (purple=2)
* [ ] Spawn berries not touching snake

### Artillery

Crab is Napoleon?

Increments capabilities related to,

* Translation of "position" information from world to screen coordinates
* Auto/rng generated tile backgrounds
* Tile background collision

### Flappy Bird

Crab flies trying to avoid cat paw swipes

* Gravity physics
* Scrolling / moving tile map
* RNG level generation

### Asteroids

Crab ship destroys meteors that split into smaller meteors.

* Rocks breaking apart -> split entity
* Propulsion physics
* Control ideas...
    * Up - forward
    * L/R - change rotation
    * Down - N/A
    * R/L can strafe?
    * A - laser?
    * B - bomb?


### Schmup

Crab in water attacked by birds, fish, etc with lasers/etc

* Probably need broad phase collision optimizations since
  projectile count will be large
* Cohorts of enemies spawn with path patterns to follow

### Platformer

Crabio rescues other crab that ventured into a trap.

### PacCrab

Crab eats berries, fish chase.

* Path finding
* Tile collision
* RNG level generation (?)

### Space Invaders

Prevent descending crab traps from reaching sea floor.

### Brawler

Crab versus claw crackers & forks
