use crate::environment_init::{AutoSizeOnY, AutoSortOnY, YOffset, TRUNK_SCALE};
use crate::keyboard_input::PlayerInput;
use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{RigidBody, Velocity};

pub const TRUNK_ACCEL: f32 = 1000.;
pub const TRUNK_MAX_SPEED: f32 = 100.;
pub const TRUNK_FRICTION: f32 = 5.;

pub const Y_SCALE_FACTOR: f32 = 0.002;

#[derive(Component)]
pub struct Player;

pub fn move_player(
    mut query: Query<&mut Velocity, With<Player>>,
    input: Res<PlayerInput>,
    time: Res<Time>,
) {
    for (mut velocity) in query.iter_mut() {
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

pub fn update_size_on_y(mut query: Query<(&mut Transform, &Children), With<AutoSizeOnY>>) {
    for (mut trans, children) in query.iter_mut() {
        trans.scale = Vec3::ONE * TRUNK_SCALE * (1. - trans.translation.y * Y_SCALE_FACTOR);
    }
}

pub fn auto_sort_on_y(mut query: Query<(&mut Transform), With<AutoSortOnY>>) {
    for mut trans in query.iter_mut() {
        trans.translation.z = -trans.translation.y / 100.;
    }
}

pub fn reinsert_colliders(
    mut query: Query<(Entity, &Children, &Transform), With<RigidBody>>,
    c_query: Query<(Entity, &Collider, &YOffset), Without<RigidBody>>,
    mut commands: Commands,
) {
    for (parent, children, p_trans) in query.iter_mut() {
        for &child in children.iter() {
            if let Ok((entity, collider, &y_off)) = c_query.get(child) {
                commands.entity(entity).despawn();
                commands.entity(parent).with_children(|p| {
                    p.spawn((
                        collider.clone(),
                        TransformBundle::from(Transform {
                            translation: Vec3::new(0., y_off.0, 0.) * p_trans.scale * 10.,
                            scale: Vec3::new(3., 1., 1.),
                            ..default()
                        }),
                        y_off,
                    ));
                });
            }
        }
    }
}
