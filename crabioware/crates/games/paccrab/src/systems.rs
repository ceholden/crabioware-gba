use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use crabioware_core::ecs::{EntityId, World};

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

        apply_movement(
            &mut location,
            &mut direction,
            speed.0,
            |tile_x, tile_y| level.is_walkable_tile(tile_x, tile_y),
        );
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
                apply_movement(
                    &mut location,
                    &mut direction,
                    speed.0,
                    |tile_x, tile_y| level.is_ghost_walkable_tile(tile_x, tile_y),
                );
            },
        );
    }
}
