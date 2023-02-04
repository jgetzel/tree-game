use crate::keyboard_input::PlayerInput;
use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::Velocity;
use crate::environment_init::{AutoSizeOnY, AutoSortOnY, TRUNK_SCALE};

pub const TRUNK_ACCEL: f32 = 1000.;
pub const TRUNK_MAX_SPEED: f32 = 100.;
pub const TRUNK_FRICTION: f32 = 5.;

pub const Y_SCALE_FACTOR: f32 = 0.002;

#[derive(Component)]
pub struct Player;

pub fn move_player(
    mut query: Query<(&mut Velocity), With<Player>>,
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

pub fn update_size_on_y(
    mut query: Query<(&mut Transform, &Children), With<AutoSizeOnY>>,
    mut c_query: Query<(&mut Transform, &Collider), Without<AutoSizeOnY>>
) {
    for (mut trans, children) in query.iter_mut() {
        let old_scale = trans.scale;
        trans.scale = Vec3::ONE * TRUNK_SCALE * (1. - trans.translation.y * Y_SCALE_FACTOR);
        let scale_diff = old_scale - trans.scale;
        for &child in children.iter() {
            if let Ok((mut trans_c, collider)) = c_query.get_mut(child) {
                trans_c.translation *= Vec3::ONE * 2. + scale_diff;
                println!("{collider:?}")
            }
        }
    }
}

pub fn auto_sort_on_y(
    mut query: Query<(&mut Transform), With<AutoSortOnY>>
) {
    for mut trans in query.iter_mut() {
        trans.translation.z = -trans.translation.y / 100.;
    }
}

