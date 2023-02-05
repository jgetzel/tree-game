use crate::{
    animations::{AnimEnum, Animations, Animator},
    assets::SpriteEnum,
    keyboard_input::PlayerInput,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub const TRUNK_ACCEL: f32 = 4000.;
pub const TRUNK_MAX_SPEED: f32 = 100.;
pub const TRUNK_FRICTION: f32 = 15.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Flippable {
    pub right_facing: bool,
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

pub fn flip_flippables(mut query: Query<(&Velocity, &mut Sprite, &Flippable)>) {
    for (vel, mut sprite, flip) in query.iter_mut() {
        if vel.linvel.x.abs() > DEADZONE {
            let flip_inv = if flip.right_facing { 1. } else { -1. };
            sprite.flip_x = vel.linvel.x * flip_inv < 0.
        }
    }
}

pub fn player_anim_controller(
    mut query: Query<(&mut Animator), With<Player>>,
    input: Res<PlayerInput>,
    anims: Res<Animations>,
) {
    for mut anim in query.iter_mut() {
        if input.movement.length() < DEADZONE {
            anim.play_anim(anims.get(AnimEnum::TrunkIdle));
        } else {
            anim.play_anim(anims.get(AnimEnum::TrunkWalk));
        }
    }
}
