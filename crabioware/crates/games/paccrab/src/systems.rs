use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use alloc::vec::Vec;
use crabioware_core::ecs::{EntityId, World};
use crabioware_core::games::{GameState, Games};

use crate::components::PlayerComponent;

use super::ai::ghost_desired;
use super::components::{
    Direction, DirectionComponent, GhostComponent, LocationComponent, SpeedComponent,
};
use super::levels::Level;
use super::movement::{apply_movement, tile_of};

pub(crate) fn system_player(
    world: &World,
    player: &EntityId,
    level: &Level,
    buttons: &ButtonController,
) {
    world.with::<(
        &mut LocationComponent,
        &mut DirectionComponent,
        &SpeedComponent,
    ), _, _>(player, |(mut location, mut direction, speed)| {
        if buttons.is_pressed(Button::LEFT) {
            direction.desired = Direction::LEFT;
        } else if buttons.is_pressed(Button::RIGHT) {
            direction.desired = Direction::RIGHT;
        } else if buttons.is_pressed(Button::UP) {
            direction.desired = Direction::UP;
        } else if buttons.is_pressed(Button::DOWN) {
            direction.desired = Direction::DOWN;
        }

        apply_movement(&mut location, &mut direction, speed.0, |tile_x, tile_y| {
            level.is_walkable_tile(tile_x, tile_y)
        });
    });
}

pub(crate) fn system_ghost(
    world: &World,
    ghosts: &[EntityId],
    level: &Level,
    player_tx: i32,
    player_ty: i32,
    player_dir: Direction,
    rng: &mut RandomNumberGenerator,
) {
    for ghost in ghosts {
        world.with::<(
            &mut LocationComponent,
            &mut DirectionComponent,
            &SpeedComponent,
            &mut GhostComponent,
        ), _, _>(
            ghost,
            |(mut location, mut direction, speed, mut ghost_comp)| {
                let tx = tile_of(location.location.x);
                let ty = tile_of(location.location.y);
                direction.desired = ghost_desired(
                    &mut ghost_comp,
                    tx,
                    ty,
                    direction.direction,
                    player_tx,
                    player_ty,
                    player_dir,
                    level,
                    rng,
                );
                // FIXME: ghosts shouldn't move into house _after_ first exit
                apply_movement(&mut location, &mut direction, speed.0, |tile_x, tile_y| {
                    level.is_ghost_walkable_tile(tile_x, tile_y)
                });
            },
        );
    }
}

/// Player <> dots & pellets
pub(crate) fn system_dots(
    world: &World,
    level: &Level,
    player: &EntityId,
    dots_eaten: &mut [bool],
) {
    let (player_tx, player_ty, mut player_component) =
        world.with::<(&LocationComponent, &mut PlayerComponent), _, _>(player, |(loc, pc)| {
            (tile_of(loc.location.x), tile_of(loc.location.y), pc)
        });

    if let Some(tile_index) = level.is_edible_dot_tile(player_tx, player_ty, dots_eaten) {
        dots_eaten[tile_index] = true;
    }
    if let Some(tile_index) = level.is_edible_pellet_tile(player_tx, player_ty, dots_eaten) {
        dots_eaten[tile_index] = true;
        player_component.energized = true;
    }
}

/// Check collisions between ghosts and player
pub(crate) fn system_collision(
    world: &mut World,
    player: &EntityId,
    ghosts: &mut Vec<EntityId>,
) -> GameState {
    let (player_tx, player_ty, energized) = world
        .with::<(&LocationComponent, &PlayerComponent), _, _>(player, |(loc, pc)| {
            (
                tile_of(loc.location.x),
                tile_of(loc.location.y),
                pc.energized,
            )
        });

    let mut player_dead = false;
    ghosts.retain(|ghost| {
        let collided = world.with::<(&LocationComponent), _, _>(ghost, |loc| {
            tile_of(loc.location.x) == player_tx && tile_of(loc.location.y) == player_ty
        });

        match (collided, energized) {
            (true, false) => {
                player_dead = true;
                true
            }
            (true, true) => {
                world.destroy(ghost);
                false
            }
            _ => true,
        }
    });

    if player_dead {
        GameState::GameOver
    } else {
        GameState::Running(Games::PacCrab)
    }
}
