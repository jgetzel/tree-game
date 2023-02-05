use bevy::{prelude::{Component, Query, Res, Resource, Time, Transform, With, Without, OrthographicProjection, Commands, Vec3, default, GlobalTransform, Camera, Vec2}, sprite::SpriteBundle};
use lerp::Lerp;
use crate::{player::Player, assets::{GameAssets, SpriteEnum}};

#[derive(Resource, Component)]
pub struct MainCamera;

const CAMERA_SMOOTHING: f32 = 2.;

#[derive(Component)]
pub struct CameraBounds(pub f32, pub f32);

#[derive(Component)]
pub struct LockedCamera;

pub fn camera_follow(
    mut camera_q: Query<(&mut Transform, &Camera, Option<&CameraBounds>), (With<MainCamera>, Without<LockedCamera>)>,
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    time: Res<Time>
) {
    let Ok((mut cam_trans, cam, bounds)) = camera_q.get_single_mut()
        else { return; };
    let Ok(player_trans) = player_q.get_single()
        else { return; };

    cam_trans.translation.x = cam_trans.translation.x.lerp(
        player_trans.translation.x,
        time.delta_seconds() * CAMERA_SMOOTHING
    );

    let Some(bounds) = bounds 
        else { return; };

    let left = ndc_to_world(&cam_trans, cam, Vec2::new(-1., 0.));
    let right = ndc_to_world(&cam_trans, cam, Vec2::new(1., 0.));

    if left.x < bounds.0 {
        cam_trans.translation.x = bounds.0 + (cam_trans.translation.x - left.x);
    }
    if right.x > bounds.1 {
        cam_trans.translation.x = bounds.1 - (right.x - cam_trans.translation.x);
    }
}

#[derive(Component)]
pub struct CameraDebugCircle;

pub fn camera_debug_circle(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut circle_q: Query<&mut Transform, (With<CameraDebugCircle>, Without<OrthographicProjection>)>,
    assets: Res<GameAssets>,
    mut commands: Commands
) {
    let Ok((trans, camera)) = camera_q.get_single() 
    else { return; };

    let Ok(mut circle) = circle_q.get_single_mut() 
    else {
        commands.spawn((
            SpriteBundle {
                texture: assets.map.get(&SpriteEnum::DebugCircle).unwrap().clone(),
                transform: Transform {
                    scale: Vec3::ONE * 0.01,
                    ..default()
                },
                ..default()
            },
            CameraDebugCircle
        ));
        return;
    };

    let ndc_to_world = trans.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(Vec3::new(1., 0., -1.));

    circle.translation = world_pos.truncate().extend(0.);
}

fn ndc_to_world(trans: &Transform, cam: &Camera, ndc: Vec2) -> Vec2 {
    let ndc_to_world = trans.compute_matrix() * cam.projection_matrix().inverse();
    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.));
    world_pos.truncate()
}
