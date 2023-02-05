use bevy::prelude::{Audio, Commands, default, Res, SpriteBundle, Transform, TransformBundle};
use bevy::math::Vec3;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::math::Vect;
use bevy::hierarchy::BuildChildren;
use crate::assets::{GameAssets, SpriteEnum};
use crate::assets::AudioEnum::MusicMainTheme;
use crate::assets::SpriteEnum::HouseFront;
use crate::init_systems::{AutoSortOnY, YOffset};

pub const HOUSE_FRONT_SCALE: f32 = 0.15;

pub fn init_background(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: assets.sprites.get(&HouseFront).unwrap().clone(),
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
            p.spawn(Collider::polyline(
                vec![
                    Vect::new(121., -13.),
                    Vect::new(464., -155.),
                    Vect::new(594., -80.),
                    Vect::new(716., -118.),
                    Vect::new(715., 270.),
                ],
                Some(vec![[0, 1], [1, 2], [2, 3], [3, 4], [4, 0]]),
            ));
            p.spawn(Collider::polyline(
                vec![
                    Vect::new(-716., 33.),
                    Vect::new(-715., -256.),
                    Vect::new(713., -256.),
                    Vect::new(713., 69.),
                    Vect::new(162., 28.),
                    Vect::new(-255., 70.),
                ],
                Some(vec![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 0]]),
            ));
            p.spawn(Collider::ball(200.))
                .insert(TransformBundle::from(Transform {
                    translation: Vec3::new(561., -120., 0.) / HOUSE_FRONT_SCALE,
                    ..default()
                }));
        });

    commands.spawn(SpriteBundle {
        texture: assets
            .sprites
            .get(&SpriteEnum::HouseFrontBackground)
            .unwrap()
            .clone(),
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
                .sprites
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
                .sprites
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
                .sprites
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
        })
        .insert(AutoSortOnY)
        .insert(YOffset(-25.));
}

pub fn init_music(
    audio: Res<Audio>,
    assets: Res<GameAssets>
) {
    audio.play(assets.audio.get(&MusicMainTheme).unwrap().clone());
}
