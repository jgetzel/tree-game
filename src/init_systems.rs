use crate::animations::{Animations, Animator, AnimEnum};
use crate::assets::SpriteEnum::HouseFront;
use crate::assets::{AppState, GameAssets, SpriteEnum};
use crate::camera::{MainCamera, CameraBounds};
use crate::player::{Flippable, Player, TRUNK_FRICTION};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const TRUNK_COLLIDER_RADIUS: f32 = 100.;
const TRUNK_COLLIDER_Y_OFFSET: f32 = -100.;

pub const TRUNK_SCALE: f32 = 0.075;
pub const HOUSE_FRONT_SCALE: f32 = 0.15;

const CAMERA_LAYER: f32 = 100.;
const CAMERA_SCALE: f32 = 0.5;

pub struct EnvironmentInitPlugin;

impl Plugin for EnvironmentInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(init_player)
                .with_system(init_camera)
                .with_system(init_background),
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

fn init_player(
    mut commands: Commands, 
    assets: Res<GameAssets>,
    animations: Res<Animations>
) {
    commands
        .spawn((
            SpriteBundle {
                texture: assets.map.get(&SpriteEnum::TrunkJr).unwrap().clone(),
                ..default()
            },
            Player,
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
            Animator::new(animations.get(AnimEnum::TrunkWalk))
        ))
        .insert(SpatialBundle {
            transform: Transform {
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

            #[cfg(debug_assertions)]
            p.spawn(SpriteBundle {
                texture: assets.map.get(&SpriteEnum::DebugCircle).unwrap().clone(),
                transform: Transform {
                    scale: Vec3::ONE * 0.3,
                    translation: Vec3::new(0., 0., 1.),
                    ..default()
                },
                ..default()
            });
        });
}

fn init_background(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: assets.map.get(&HouseFront).unwrap().clone(),
            transform: Transform {
                scale: Vec3::ONE * HOUSE_FRONT_SCALE,
                translation: Vec3::new(0., 0., -500.),
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn(Collider::ball(100.))
                .insert(TransformBundle::from(Transform {
                    translation: Vec3::new(-222., 4., 0.) / HOUSE_FRONT_SCALE,
                    scale: Vec3::new(3., 1., 1.),
                    ..default()
                }));
            p.spawn(Collider::ball(100.))
                .insert(TransformBundle::from(Transform {
                    translation: Vec3::new(-532., -60., 0.) / HOUSE_FRONT_SCALE,
                    scale: Vec3::new(3., 1., 1.),
                    ..default()
                }));
            p.spawn(
                Collider::polyline(vec![
                    Vect::new(121., -13.),
                    Vect::new(464., -155.),
                    Vect::new(594., -80.),
                    Vect::new(716., -118.),
                    Vect::new(715., 270.),
                ], Some(vec![[0, 1], [1, 2], [2, 3], [3, 4], [4, 0]]))
            );
            p.spawn(Collider::polyline(
                vec![
                    Vect::new(-716., 33.),
                    Vect::new(-715., -256.),
                    Vect::new(713., -256.),
                    Vect::new(713., 69.),
                    Vect::new(162., 28.),
                    Vect::new(-255., 70.)
                ],
                Some(vec![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 0]]),
            ));
            p.spawn(Collider::ball(200.))
            .insert(TransformBundle::from(Transform {
                translation: Vec3::new(561., -120., 0.) / HOUSE_FRONT_SCALE,
                ..default()
            })
            );
        });

    commands.spawn(SpriteBundle {
        texture: assets.map.get(&SpriteEnum::HouseFrontBackground).unwrap().clone(),
        transform: Transform {
            scale: Vec3::ONE * 0.14,
            translation: Vec3::new(0., 0., -501.),
            ..default()
        },
        ..default()
    });

    commands
        .spawn(SpriteBundle {
            texture: assets
                .map
                .get(&SpriteEnum::HouseFrontHouse)
                .unwrap()
                .clone(),
            transform: Transform {
                scale: Vec3::ONE * HOUSE_FRONT_SCALE,
                ..default()
            },
            ..default()
        })
        .insert(YOffset(-13.))
        .insert(AutoSortOnY);

    commands
        .spawn(SpriteBundle {
            texture: assets
                .map
                .get(&SpriteEnum::HouseFrontTree1)
                .unwrap()
                .clone(),
            transform: Transform {
                scale: Vec3::ONE * HOUSE_FRONT_SCALE,
                ..default()
            },
            ..default()
        })
        .insert(YOffset(-41.5))
        .insert(AutoSortOnY);

    commands
        .spawn(SpriteBundle {
            texture: assets
                .map
                .get(&SpriteEnum::HouseFrontTree2)
                .unwrap()
                .clone(),
            transform: Transform {
                scale: Vec3::ONE * HOUSE_FRONT_SCALE,
                ..default()
            },
            ..default()
        })
        .insert(YOffset(9.7))
        .insert(AutoSortOnY);

    commands
        .spawn(SpriteBundle {
            texture: assets.get(SpriteEnum::TrashCan),
            transform: Transform {
                translation: Vec3::new(561., -91., 0.),
                scale: Vec3::ONE * HOUSE_FRONT_SCALE,
                ..default()
            },
            ..default()
        }).insert(AutoSortOnY)
        .insert(YOffset(-25.));
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., CAMERA_LAYER),
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

fn init_gravity(mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::new(0., 0.);
}
