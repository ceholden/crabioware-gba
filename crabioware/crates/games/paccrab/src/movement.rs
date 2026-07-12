use agb::fixnum::num;
use crabioware_core::types::Number;

use super::components::{Direction, DirectionComponent, LocationComponent};

fn tile_of(pos: Number, tile_size: i32) -> i32 {
    (pos / Number::new(tile_size)).floor()
}

fn tile_center(tile_idx: i32, tile_size: i32) -> Number {
    Number::new(tile_idx * tile_size + tile_size / 2)
}

fn nearest_tile_center(pos: Number, tile_size: i32) -> Number {
    tile_center(tile_of(pos, tile_size), tile_size)
}

fn nudge_to_center(pos: Number, speed: Number, tile_size: i32) -> Number {
    let center = nearest_tile_center(pos, tile_size);
    let delta = center - pos;
    if delta.abs() <= speed {
        center
    } else if delta > num!(0.) {
        pos + speed
    } else {
        pos - speed
    }
}

pub(crate) fn aligned_for_turn(pos: Number, speed: Number, tile_size: i32) -> bool {
    (pos - nearest_tile_center(pos, tile_size)).abs() <= speed
}

/// Walkable neighbors of tile `(tx, ty)`, omitting `exclude` (the direction
/// the entity came from).  Returns a fixed-size array; iterate with
/// `.into_iter().flatten()`.
pub(crate) fn open_neighbors<F>(
    tx: i32,
    ty: i32,
    exclude: Direction,
    fn_walkable: F,
) -> [Option<Direction>; 4]
where
    F: Fn(i32, i32) -> bool,
{
    let mut result = [None; 4];
    let mut i = 0;
    for d in [
        Direction::RIGHT,
        Direction::LEFT,
        Direction::UP,
        Direction::DOWN,
    ] {
        if d == exclude {
            continue;
        }
        let open = match d {
            Direction::RIGHT => fn_walkable(tx + 1, ty),
            Direction::LEFT => fn_walkable(tx - 1, ty),
            Direction::UP => fn_walkable(tx, ty - 1),
            Direction::DOWN => fn_walkable(tx, ty + 1),
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
pub(crate) fn apply_movement<FWalk, FWarp>(
    location: &mut LocationComponent,
    direction: &mut DirectionComponent,
    speed: Number,
    tile_size: i32,
    fn_walk: FWalk,
    fn_warp: FWarp,
) -> (i32, i32)
where
    FWalk: Fn(i32, i32) -> bool,
    FWarp: Fn(i32, i32) -> Option<(Number, Number)>,
{
    // Nudge perpendicular axis toward tile center
    match direction.direction {
        Direction::LEFT | Direction::RIGHT => {
            location.location.y = nudge_to_center(location.location.y, speed, tile_size);
        }
        Direction::UP | Direction::DOWN => {
            location.location.x = nudge_to_center(location.location.x, speed, tile_size);
        }
    }

    // Compute tile position once (after nudge, reused by both turn and move)
    let tx = tile_of(location.location.x, tile_size);
    let ty = tile_of(location.location.y, tile_size);
    let cx = tile_center(tx, tile_size);
    let cy = tile_center(ty, tile_size);

    // Attempt turn into desired direction
    let desired = direction.desired;
    let current = direction.direction;
    if desired != current {
        let is_uturn = desired == current.opposite();
        let aligned = is_uturn
            || match desired {
                Direction::LEFT | Direction::RIGHT => {
                    aligned_for_turn(location.location.y, speed, tile_size)
                }
                Direction::UP | Direction::DOWN => {
                    aligned_for_turn(location.location.x, speed, tile_size)
                }
            };
        if aligned {
            let tile_open = match desired {
                Direction::RIGHT => fn_walk(tx + 1, ty),
                Direction::LEFT => fn_walk(tx - 1, ty),
                Direction::UP => fn_walk(tx, ty - 1),
                Direction::DOWN => fn_walk(tx, ty + 1),
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
            } else if fn_walk(tx + 1, ty) {
                location.location.x = nx;
            } else if let Some((wx, wy)) = fn_warp(tx, ty) {
                location.location.x = wx;
                location.location.y = wy;
            } else {
                location.location.x = cx;
            }
        }
        Direction::LEFT => {
            let nx = x - speed;
            if x > cx {
                location.location.x = if nx > cx { nx } else { cx };
            } else if fn_walk(tx - 1, ty) {
                location.location.x = nx;
            } else if let Some((wx, wy)) = fn_warp(tx, ty) {
                location.location.x = wx;
                location.location.y = wy;
            } else {
                location.location.x = cx;
            }
        }
        Direction::UP => {
            let ny = y - speed;
            if y > cy {
                location.location.y = if ny > cy { ny } else { cy };
            } else if fn_walk(tx, ty - 1) {
                location.location.y = ny;
            } else if let Some((wx, wy)) = fn_warp(tx, ty) {
                location.location.x = wx;
                location.location.y = wy;
            } else {
                location.location.y = cy;
            }
        }
        Direction::DOWN => {
            let ny = y + speed;
            if y < cy {
                location.location.y = if ny < cy { ny } else { cy };
            } else if fn_walk(tx, ty + 1) {
                location.location.y = ny;
            } else if let Some((wx, wy)) = fn_warp(tx, ty) {
                location.location.x = wx;
                location.location.y = wy;
            } else {
                location.location.y = cy;
            }
        }
    }

    (tx, ty)
}
