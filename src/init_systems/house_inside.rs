use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::math::{vec2, vec3};
use bevy::prelude::{BuildChildren, Camera, Camera2d, Camera2dBundle, Color, Commands, default, Entity, OrthographicProjection, Query, Res, Sprite, Transform, TransformBundle, Vec3, With};
use bevy::sprite::SpriteBundle;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Damping, LockedAxes, RigidBody, Sensor, Velocity};
use crate::animations::{Animations, Animator, AnimEnum};
use crate::animations::AnimEnum::{MouseyIdle, MouseyWalk};
use crate::assets::{GameAssets, SpriteEnum};
use crate::assets::SpriteEnum::MouseyIdle1;
use crate::camera::{CameraBounds, LockedCamera, MainCamera};
use crate::init_systems::{AutoSortOnY, CAMERA_LAYER, CAMERA_SCALE, Mousey, TRUNK_SCALE, YOffset};
use crate::init_systems::environment::HOUSE_FRONT_SCALE;
use crate::player::{Flippable, PlayerInteractor, TRUNK_FRICTION};
use crate::utils::{BeginAnimPos, MOUSE_DOOR_HOP_TIME, MouseDoorHopAnimated, MouseDoorHopFinishAnim, MouseTrashAnimated, WalkingMouse};

pub fn init_background(
    mut commands: Commands,
    assets: Res<GameAssets>
) {
    commands.spawn(SpriteBundle {
        texture: assets.get(SpriteEnum::RoomBackground2),
        ..default()
    }).insert(Transform {
        translation: vec3(0., -1000., -500.),
        scale: Vec3::ONE * HOUSE_FRONT_SCALE,
        ..default()
    }).with_children(|p| {
        p.spawn(SpriteBundle {
            texture: assets.get(SpriteEnum::RoomFurniture1),
            ..default()
        });

        p.spawn(SpriteBundle {
            texture: assets.get(SpriteEnum::RoomFurniture2),
            ..default()
        });

        p.spawn(SpriteBundle {
            texture: assets.get(SpriteEnum::RoomFurniture3),
            ..default()
        });
        p.spawn(SpriteBundle {
            texture: assets.get(SpriteEnum::RoomDoor),
            ..default()
        }).insert(Transform::from_translation(
            vec3(-409., -51., 1.) / HOUSE_FRONT_SCALE));

        p.spawn(SpriteBundle {
            texture: assets.get(SpriteEnum::RoomBirdDoor),
            ..default()
        }).insert(Transform::from_translation(
            vec3(383., 217., 1.) / HOUSE_FRONT_SCALE));

        p.spawn(SpriteBundle {
            texture: assets.get(SpriteEnum::RoomSlidingDoor),
            ..default()
        }).insert(Transform::from_translation(
            vec3(345., -38., 1.) / HOUSE_FRONT_SCALE));

        p.spawn(Collider::polyline(vec![
            vec2(-481., -1230.),
            vec2(-354., -1146.),
            vec2(-308., -1159.),
            vec2(-240., -1112.),
            vec2(-233., -1052.),
            vec2(-169., -1049.),
            vec2(-169., -1103.),
            vec2(116., -1113.),
            vec2(236., -1081.),
            vec2(500., -1272.),
            vec2(-477., -1274.),
            vec2(-481., -1230.),
        ], None));
    });

    commands.spawn(SpriteBundle {
        texture: assets.get(SpriteEnum::GirlIdle1),
        transform: Transform {
            translation: vec3(406., -1031., -501.),
            scale: Vec3::ONE * 0.15,
            ..default()
        },
        ..default()
    });

}

pub fn init_mousey(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut anims: Res<Animations>
) {
    commands.spawn(
        SpriteBundle {
            texture: assets.get(MouseyIdle1),
            transform: Transform {
                translation: Vec3::new(-413., -977., 0.),
                scale: Vec3::ONE * 0.1,
                ..default()
            },
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            ..default()
        },
    ).insert((
        YOffset(0.),
        WalkingMouse,
        PlayerInteractor,
        Flippable { right_facing: false },
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Damping {
            linear_damping: TRUNK_FRICTION,
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(100.),
        ActiveEvents::COLLISION_EVENTS

    ))
        .insert(Mousey)
        .insert(MouseDoorHopFinishAnim(0.))
        .insert(Animator::new(anims.get(MouseyWalk)))
        .with_children(|p| {
            p.spawn(Collider::ball(100.))
                .insert(TransformBundle::from(Transform::default()))
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Mousey);
        });
}

pub fn init_camera(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Camera), With<MainCamera>>
) {
    let (ent, mut main_cam) = q.single_mut();
    main_cam.is_active = false;
    commands.entity(ent).remove::<MainCamera>();
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(0., -1000., CAMERA_LAYER),
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
    ))
        .insert(LockedCamera);
}
