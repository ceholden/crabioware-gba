use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use alloc::vec::Vec;
use crabioware_core::ecs::{EntityId, World};

use crate::components::{PlayerComponent, SpriteComponent};

use super::ai::ghost_desired;
use super::components::{
    Direction, DirectionComponent, GhostComponent, LocationComponent, SpeedComponent,
};
use super::levels::Level;
use super::movement::apply_movement;

pub(crate) fn system_player(
    world: &World,
    level: &Level,
    player: &EntityId,
    buttons: &ButtonController,
) {
    world.with::<(
        &mut LocationComponent,
        &mut DirectionComponent,
        &SpeedComponent,
        &mut PlayerComponent,
        &mut SpriteComponent,
    ), _, _>(
        player,
        |(mut location, mut direction, speed, mut pc, mut spr)| {
            if buttons.is_pressed(Button::LEFT) {
                direction.desired = Direction::LEFT;
            } else if buttons.is_pressed(Button::RIGHT) {
                direction.desired = Direction::RIGHT;
            } else if buttons.is_pressed(Button::UP) {
                direction.desired = Direction::UP;
            } else if buttons.is_pressed(Button::DOWN) {
                direction.desired = Direction::DOWN;
            }

            // Decrement energized time
            pc.energized_time = pc.energized_time.saturating_sub(1);
            if !pc.is_energized() {
                spr.alt_mode = false;
            }

            apply_movement(
                &mut location,
                &mut direction,
                speed.0,
                level.tile_size as i32,
                |tile_x, tile_y| level.is_walkable_tile(tile_x, tile_y),
                |tile_x, tile_y| level.warp_destination(tile_x, tile_y),
            );
        },
    );
}

pub(crate) fn system_ghost(
    world: &World,
    level: &Level,
    ghosts: &[EntityId],
    player: &EntityId,
    rng: &mut RandomNumberGenerator,
) {
    // Snapshot player tile position for ghost AI before running player system
    let (player_tx, player_ty, player_dir, player_energized) =
        world.with::<(&LocationComponent, &DirectionComponent, &PlayerComponent), _, _>(
            player,
            |(loc, dir, pc)| {
                (
                    level.tile_of(loc.location.x),
                    level.tile_of(loc.location.y),
                    dir.direction,
                    pc.is_energized(),
                )
            },
        );

    for ghost in ghosts {
        world.with::<(
            &mut LocationComponent,
            &mut DirectionComponent,
            &SpeedComponent,
            &mut GhostComponent,
            &mut SpriteComponent,
        ), _, _>(
            ghost,
            |(mut location, mut direction, speed, mut ghost_comp, mut sprite_comp)| {
                let tx = level.tile_of(location.location.x);
                let ty = level.tile_of(location.location.y);
                ghost_comp.scared = player_energized;
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

                let (ghost_tx, ghost_ty) = apply_movement(
                    &mut location,
                    &mut direction,
                    speed.0,
                    level.tile_size as i32,
                    |tile_x, tile_y| {
                        level.is_ghost_walkable_tile(tile_x, tile_y)
                            || (!ghost_comp.exited
                                && level.is_door_tile(tile_x as u8, tile_y as u8))
                    },
                    |tile_x, tile_y| level.warp_destination(tile_x, tile_y),
                );

                if !ghost_comp.exited {
                    if level.is_walkable_tile(ghost_tx, ghost_ty) {
                        ghost_comp.exited = true;
                    }
                }

                sprite_comp.alt_mode = player_energized;
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
    dots_dirty: &mut Vec<usize>,
) {
    world.with::<(
        &LocationComponent,
        &mut PlayerComponent,
        &mut SpriteComponent,
    ), _, _>(player, |(loc_comp, mut player_comp, mut sprite_comp)| {
        let player_tx = level.tile_of(loc_comp.location.x);
        let player_ty = level.tile_of(loc_comp.location.y);

        if let Some(tile_index) = level.is_edible_dot_tile(player_tx, player_ty, dots_eaten) {
            dots_eaten[tile_index] = true;
            dots_dirty.push(tile_index);
        }
        if let Some(tile_index) = level.is_edible_pellet_tile(player_tx, player_ty, dots_eaten) {
            dots_eaten[tile_index] = true;
            dots_dirty.push(tile_index);
            player_comp.energized_time = 300;
            sprite_comp.alt_mode = true;
            true
        } else {
            false
        }
    });
}

/// Check collisions between ghosts and player, returning true if player has died
pub(crate) fn system_collision(
    world: &mut World,
    level: &Level,
    player: &EntityId,
    ghosts: &mut Vec<EntityId>,
) -> bool {
    let (player_tx, player_ty, is_energized) = world
        .with::<(&LocationComponent, &PlayerComponent), _, _>(player, |(loc, pc)| {
            (
                level.tile_of(loc.location.x),
                level.tile_of(loc.location.y),
                pc.energized_time > 0,
            )
        });

    let mut player_dead = false;
    ghosts.retain(|ghost| {
        let collided = world.with::<(&LocationComponent), _, _>(ghost, |loc| {
            level.tile_of(loc.location.x) == player_tx && level.tile_of(loc.location.y) == player_ty
        });

        match (collided, is_energized) {
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

    player_dead
}
