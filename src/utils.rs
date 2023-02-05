use bevy::prelude::{Commands, default, Entity, Query, Transform, TransformBundle, With, Without};
use bevy::hierarchy::{BuildChildren, Children};
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;
use bevy::math::Vec3;
use crate::init_systems::{AutoSizeOnY, AutoSortOnY, TRUNK_SCALE, YOffset};

pub const Y_SCALE_FACTOR: f32 = 0.001;

pub fn update_size_on_y(mut query: Query<&mut Transform, With<AutoSizeOnY>>) {
    for mut trans in query.iter_mut() {
        trans.scale = Vec3::ONE * TRUNK_SCALE * (1. - trans.translation.y * Y_SCALE_FACTOR);
    }
}

pub fn auto_sort_on_y(mut query: Query<(&mut Transform, Option<&YOffset>), With<AutoSortOnY>>) {
    for (mut trans, y_off) in query.iter_mut() {
        let y_off = match y_off {
            Some(y_off) => y_off.0,
            None => 0.
        };
        trans.translation.z = -(trans.translation.y + y_off) / 100.;
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
                            scale: Vec3::new(4., 1., 1.),
                            ..default()
                        }),
                        y_off,
                    ));
                });
            }
        }
    }
}
