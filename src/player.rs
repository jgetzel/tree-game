use crate::keyboard_input::PlayerInput;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub const TRUNK_ACCEL: f32 = 5000.;
pub const TRUNK_MAX_SPEED: f32 = 200.;
pub const TRUNK_FRICTION: f32 = 7.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Flippable {
    pub right_facing: bool
}

const DEADZONE: f32 = 0.15;

pub fn move_player(
    mut query: Query<&mut Velocity, With<Player>>,
    input: Res<PlayerInput>,
    time: Res<Time>,
) {
    for mut velocity in query.iter_mut() {
        let new_velocity = velocity.linvel + (TRUNK_ACCEL * input.movement * time.delta_seconds());
        velocity.linvel = if [TRUNK_MAX_SPEED, velocity.linvel.length()]
            .iter()
            .all(|v| new_velocity.length() > *v)
        {
            new_velocity.clamp_length_max(TRUNK_MAX_SPEED)
        } else {
            new_velocity
        };
    }
}

pub fn flip_flippables(
    mut query: Query<(&Velocity, &mut Sprite, &Flippable)>,
) {
    for (vel, mut sprite, flip) in query.iter_mut() {
        if vel.linvel.x.abs() > DEADZONE {
            let flip_inv = if flip.right_facing { 1. } else { -1. };
            sprite.flip_x = vel.linvel.x * flip_inv < 0.
        }
    }
}
