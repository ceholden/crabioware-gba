use agb::fixnum::num;
use crabioware_core::types::Number;

use super::components::{Direction, DirectionComponent, LocationComponent};
use super::levels::{Level, TILE_SIZE};

pub(crate) fn tile_of(pos: Number) -> i32 {
    (pos / Number::new(TILE_SIZE)).floor()
}

pub(crate) fn tile_center(tile_idx: i32) -> Number {
    Number::new(tile_idx * TILE_SIZE + TILE_SIZE / 2)
}

pub(crate) fn nearest_tile_center(pos: Number) -> Number {
    tile_center(tile_of(pos))
}

fn nudge_to_center(pos: Number, speed: Number) -> Number {
    let center = nearest_tile_center(pos);
    let delta = center - pos;
    if delta.abs() <= speed {
        center
    } else if delta > num!(0.) {
        pos + speed
    } else {
        pos - speed
    }
}

pub(crate) fn aligned_for_turn(pos: Number, speed: Number) -> bool {
    (pos - nearest_tile_center(pos)).abs() <= speed
}

/// Walkable neighbors of tile `(tx, ty)`, omitting `exclude` (the direction
/// the entity came from).  Returns a fixed-size array; iterate with
/// `.into_iter().flatten()`.
pub(crate) fn open_neighbors(
    level: &Level,
    tx: i32,
    ty: i32,
    exclude: Direction,
) -> [Option<Direction>; 4] {
    let mut result = [None; 4];
    let mut i = 0;
    for d in [Direction::RIGHT, Direction::LEFT, Direction::UP, Direction::DOWN] {
        if d == exclude {
            continue;
        }
        let open = match d {
            Direction::RIGHT => level.is_walkable_tile(tx + 1, ty),
            Direction::LEFT => level.is_walkable_tile(tx - 1, ty),
            Direction::UP => level.is_walkable_tile(tx, ty - 1),
            Direction::DOWN => level.is_walkable_tile(tx, ty + 1),
        };
        if open {
            result[i] = Some(d);
            i += 1;
        }
    }
    result
}

/// Shared movement kernel for any maze entity (player or ghost).
///
/// The caller sets `direction.desired` before calling.  This function:
/// - nudges the perpendicular axis toward the tile center
/// - commits the buffered turn when aligned and the target tile is open
/// - advances along the current direction, stopping cleanly at tile centers
pub(crate) fn apply_movement(
    location: &mut LocationComponent,
    direction: &mut DirectionComponent,
    speed: Number,
    level: &Level,
) {
    // Nudge perpendicular axis toward tile center
    match direction.direction {
        Direction::LEFT | Direction::RIGHT => {
            location.location.y = nudge_to_center(location.location.y, speed);
        }
        Direction::UP | Direction::DOWN => {
            location.location.x = nudge_to_center(location.location.x, speed);
        }
    }

    // Compute tile position once (after nudge, reused by both turn and move)
    let tx = tile_of(location.location.x);
    let ty = tile_of(location.location.y);
    let cx = tile_center(tx);
    let cy = tile_center(ty);

    // Attempt turn into desired direction
    let desired = direction.desired;
    let current = direction.direction;
    if desired != current {
        let is_uturn = desired == current.opposite();
        let aligned = is_uturn || match desired {
            Direction::LEFT | Direction::RIGHT => aligned_for_turn(location.location.y, speed),
            Direction::UP | Direction::DOWN => aligned_for_turn(location.location.x, speed),
        };
        if aligned {
            let tile_open = match desired {
                Direction::RIGHT => level.is_walkable_tile(tx + 1, ty),
                Direction::LEFT => level.is_walkable_tile(tx - 1, ty),
                Direction::UP => level.is_walkable_tile(tx, ty - 1),
                Direction::DOWN => level.is_walkable_tile(tx, ty + 1),
            };
            if tile_open || is_uturn {
                match desired {
                    Direction::LEFT | Direction::RIGHT => location.location.y = cy,
                    Direction::UP | Direction::DOWN => location.location.x = cx,
                }
                direction.direction = desired;
            }
        }
    }

    // Tile-center-based movement: approach center freely, gate on next-tile walkability
    let x = location.location.x;
    let y = location.location.y;
    match direction.direction {
        Direction::RIGHT => {
            let nx = x + speed;
            if x < cx {
                location.location.x = if nx < cx { nx } else { cx };
            } else if level.is_walkable_tile(tx + 1, ty) {
                location.location.x = nx;
            } else {
                location.location.x = cx;
            }
        }
        Direction::LEFT => {
            let nx = x - speed;
            if x > cx {
                location.location.x = if nx > cx { nx } else { cx };
            } else if level.is_walkable_tile(tx - 1, ty) {
                location.location.x = nx;
            } else {
                location.location.x = cx;
            }
        }
        Direction::UP => {
            let ny = y - speed;
            if y > cy {
                location.location.y = if ny > cy { ny } else { cy };
            } else if level.is_walkable_tile(tx, ty - 1) {
                location.location.y = ny;
            } else {
                location.location.y = cy;
            }
        }
        Direction::DOWN => {
            let ny = y + speed;
            if y < cy {
                location.location.y = if ny < cy { ny } else { cy };
            } else if level.is_walkable_tile(tx, ty + 1) {
                location.location.y = ny;
            } else {
                location.location.y = cy;
            }
        }
    }
}
