use crate::assets::{AppState, GameAssets, SpriteEnum};
use crate::camera::MainCamera;
use crate::player::{Player, TRUNK_FRICTION};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::ColliderMassProps::Mass;
use bevy_rapier2d::rapier::prelude::ColliderPosition;

const TRUNK_COLLIDER_RADIUS: f32 = 100.;
const TRUNK_COLLIDER_Y_OFFSET: f32 = -100.;

pub const TRUNK_SCALE: f32 = 0.1;

const CAMERA_LAYER: f32 = 100.;

pub struct EnvironmentInitPlugin;

impl Plugin for EnvironmentInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(init_player)
                .with_system(init_camera)
                .with_system(init_ball)
        );

        if app.is_plugin_added::<RapierPhysicsPlugin>() {
            app.add_startup_system(init_gravity);
        }
    }
}

#[derive(Component)]
pub struct AutoSizeOnY;

#[derive(Component)]
pub struct AutoSortOnY;

fn init_player(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            SpriteBundle {
                texture: assets.map.get(&SpriteEnum::TrunkJr).unwrap().clone(),
                transform: Transform {
                    scale: Vec3::ONE * TRUNK_SCALE,
                    ..default()
                },
                ..default()
            },
            Player,
            AutoSizeOnY,
            AutoSortOnY,
            Velocity::default(),
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: TRUNK_FRICTION,
                ..default()
            },
            RigidBody::Dynamic,
        ))
        .with_children(|p| {
            p.spawn(Collider::ball(TRUNK_COLLIDER_RADIUS))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.,
                    TRUNK_COLLIDER_Y_OFFSET,
                    0.,
                )))
                .insert(ColliderMassProperties::Density(1.0));
        });
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., CAMERA_LAYER),
            ..default()
        },
        MainCamera,
    ));
}

fn init_ball(world: &mut World){
    world.spawn(Collider::ball(100.));
}

fn init_gravity(mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::new(0., 0.);
}
