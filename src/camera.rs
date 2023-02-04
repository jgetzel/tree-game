use bevy::prelude::{Component, Query, Res, Resource, Time, Transform, With, Without};
use lerp::Lerp;
use crate::player::Player;

#[derive(Resource, Component)]
pub struct MainCamera;

const CAMERA_SMOOTHING: f32 = 2.;

pub fn camera_follow(
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    time: Res<Time>
) {
    let Ok(mut cam_trans) = camera_q.get_single_mut()
        else { return; };
    let Ok(player_trans) = player_q.get_single()
        else { return; };

    cam_trans.translation.x = cam_trans.translation.x.lerp(
        player_trans.translation.x,
        time.delta_seconds() * CAMERA_SMOOTHING
    );

}