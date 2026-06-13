use agb::rng::RandomNumberGenerator;

use super::components::{Direction, GhostComponent, GhostKind};
use super::levels::Level;
use super::movement::open_neighbors;

/// Choose the desired direction for a ghost this frame.
/// Called every frame; `apply_movement` commits the turn only when aligned.
pub(crate) fn ghost_desired(
    ghost_comp: &mut GhostComponent,
    ghost_tx: i32,
    ghost_ty: i32,
    current: Direction,
    player_tx: i32,
    player_ty: i32,
    player_dir: Direction,
    level: &Level,
    rng: &mut RandomNumberGenerator,
) -> Direction {
    let scatter_tx = ghost_comp.scatter_tx;
    let scatter_ty = ghost_comp.scatter_ty;
    let neighbors = open_neighbors(ghost_tx, ghost_ty, current.opposite(), |tile_x, tile_y| {
        level.is_ghost_walkable_tile(tile_x, tile_y)
    });

    if ghost_comp.scared {
        return navigate(
            neighbors, ghost_tx, ghost_ty, player_tx, player_ty, current, -1,
        );
    }

    match &mut ghost_comp.kind {
        GhostKind::Chase => navigate(
            neighbors, ghost_tx, ghost_ty, player_tx, player_ty, current, 1,
        ),
        GhostKind::Ambush => {
            let (tx, ty) = ahead(player_tx, player_ty, player_dir, 4);
            navigate(neighbors, ghost_tx, ghost_ty, tx, ty, current, 1)
        }
        GhostKind::Shy => {
            let dist = (ghost_tx - player_tx).abs() + (ghost_ty - player_ty).abs();
            let (tx, ty) = if dist > 8 {
                (player_tx, player_ty)
            } else {
                (scatter_tx, scatter_ty)
            };
            navigate(neighbors, ghost_tx, ghost_ty, tx, ty, current, 1)
        }
        GhostKind::Random => {
            let count = neighbors.into_iter().flatten().count();
            if count == 0 {
                return current;
            }
            let pick = (rng.gen().unsigned_abs() as usize) % count;
            neighbors.into_iter().flatten().nth(pick).unwrap_or(current)
        }
        GhostKind::Patrol {
            chase_ticks,
            shy_ticks,
            timer,
            chasing,
        } => {
            *timer += 1;
            let threshold = if *chasing { *chase_ticks } else { *shy_ticks };
            if *timer >= threshold {
                *timer = 0;
                *chasing = !*chasing;
            }
            let (tx, ty) = if *chasing {
                (player_tx, player_ty)
            } else {
                (scatter_tx, scatter_ty)
            };
            navigate(neighbors, ghost_tx, ghost_ty, tx, ty, current, 1)
        }
    }
}

/// Pick the neighbor direction with minimum Manhattan distance to `(target_tx, target_ty)`.
/// Provide `sign = 1` to move _towards_ the target, or `sign = -1` to flee.
fn navigate(
    neighbors: [Option<Direction>; 4],
    ghost_tx: i32,
    ghost_ty: i32,
    target_tx: i32,
    target_ty: i32,
    fallback: Direction,
    sign: i32,
) -> Direction {
    neighbors
        .into_iter()
        .flatten()
        .min_by_key(|&d| {
            let (nx, ny) = match d {
                Direction::RIGHT => (ghost_tx + 1, ghost_ty),
                Direction::LEFT => (ghost_tx - 1, ghost_ty),
                Direction::UP => (ghost_tx, ghost_ty - 1),
                Direction::DOWN => (ghost_tx, ghost_ty + 1),
            };
            sign * ((nx - target_tx).abs() + (ny - target_ty).abs())
        })
        .unwrap_or(fallback)
}

/// Tile position `tiles` steps ahead in `dir` from `(tx, ty)`.
fn ahead(tx: i32, ty: i32, dir: Direction, tiles: i32) -> (i32, i32) {
    match dir {
        Direction::RIGHT => (tx + tiles, ty),
        Direction::LEFT => (tx - tiles, ty),
        Direction::UP => (tx, ty - tiles),
        Direction::DOWN => (tx, ty + tiles),
    }
}
