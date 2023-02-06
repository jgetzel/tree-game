pub mod environment;
pub mod house_inside;

use crate::animations::{Animation, Animations, Animator, AnimEnum};
use crate::assets::{AppState, GameAssets, SpriteEnum};
use crate::camera::{CameraBounds, MainCamera};
use crate::player::{Flippable, Player, PlayerInteractor, Trunk, TRUNK_FRICTION};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::animations::AnimEnum::MouseyIdle;
use crate::assets::SpriteEnum::MouseyIdle1;
use crate::init_systems::environment::init_door;
use crate::init_systems::LevelState::{HouseFront, HouseInside};
use crate::utils::{Interactable, mouse_door_anim_finish};

const TRUNK_COLLIDER_RADIUS: f32 = 150.;
const TRUNK_COLLIDER_Y_OFFSET: f32 = -100.;

pub const TRUNK_SCALE: f32 = 0.075;

const CAMERA_LAYER: f32 = 100.;
const CAMERA_SCALE: f32 = 0.5;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LevelState {
    HouseFront,
    HouseInside,
    HouseBack,
}

pub struct EnvironmentInitPlugin;

impl Plugin for EnvironmentInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(init_player)
                .with_system(init_camera)
                .with_system(init_mousey)
                .with_system(environment::init_background)
                .with_system(init_door)
                .with_system(environment::init_music)
        );
        app.add_state(LevelState::HouseFront);

        app.add_system_set(
            SystemSet::on_enter(HouseInside)
                .with_system(house_inside::init_background)
                .with_system(house_inside::init_camera)
                .with_system(house_inside::init_mousey)
        );
        app.add_system_set(
            SystemSet::on_update(HouseInside)
                .with_system(mouse_door_anim_finish)
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

#[derive(Component, Copy, Clone, Reflect)]
pub struct YOffset(pub f32);

fn init_player(mut commands: Commands, assets: Res<GameAssets>, animations: Res<Animations>) {
    commands
        .spawn((
            SpriteBundle {
                texture: assets.sprites.get(&SpriteEnum::TrunkWalk1).unwrap().clone(),
                ..default()
            },
            Player,
            Trunk,
            Flippable { right_facing: true },
            AutoSizeOnY,
            AutoSortOnY,
            YOffset(-70.),
            Velocity::default(),
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: TRUNK_FRICTION,
                ..default()
            },
            RigidBody::Dynamic,
            Animator::new(animations.get(AnimEnum::TrunkWalk)),
        ))
        .insert(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(-400., 0., 0.),
                scale: Vec3::ONE * TRUNK_SCALE,
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn(Collider::ball(TRUNK_COLLIDER_RADIUS))
                .insert(YOffset(TRUNK_COLLIDER_Y_OFFSET))
                .insert(SpatialBundle {
                    transform: Transform::from_xyz(0., TRUNK_COLLIDER_Y_OFFSET, 0.),
                    ..default()
                });

            p.spawn(PlayerInteractor)
                .insert(Collider::ball(700.))
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(TransformBundle::from(Transform::from_xyz(20., 0., 0.)));

            p.spawn(SpriteBundle {
                texture: assets.get(SpriteEnum::Shadow),
                transform: Transform {
                    translation: Vec3::new(0., -90. / TRUNK_SCALE, -100.),
                    ..default()
                },
                ..default()
            });

            #[cfg(debug_assertions)]
            p.spawn(SpriteBundle {
                texture: assets.sprites.get(&SpriteEnum::DebugCircle).unwrap().clone(),
                transform: Transform {
                    scale: Vec3::ONE * 0.3,
                    translation: Vec3::new(0., 0., 1.),
                    ..default()
                },
                ..default()
            });
        });
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(-400., 0., CAMERA_LAYER),
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0., 0., 0.)),
            },
            projection: OrthographicProjection {
                scale: CAMERA_SCALE,
                ..default()
            },
            ..default()
        },
        CameraBounds(-714., 714.),
        MainCamera,
    ));
}

#[derive(Component, Copy, Clone)]
pub struct Mousey;

fn init_mousey(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut anims: Res<Animations>
) {
    commands.spawn(
        SpriteBundle {
            texture: assets.get(MouseyIdle1),
            transform: Transform {
                translation: Vec3::new(565., -52., 0.),
                scale: Vec3::ONE * 0.1,
                ..default()
            },
            ..default()
        },
    ).insert(AutoSortOnY)
        .insert(YOffset(-65.))
        .insert(Mousey)
        .insert(Animator::new(anims.get(MouseyIdle)))
        .with_children(|p| {
            p.spawn(Collider::ball(100.))
                .insert(TransformBundle::from(Transform::default()))
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Mousey)
                .insert(Interactable);
        });
}

fn init_gravity(mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::new(0., 0.);
}
