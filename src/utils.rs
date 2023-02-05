use std::f32::consts::PI;
use crate::init_systems::{AutoSizeOnY, AutoSortOnY, YOffset, TRUNK_SCALE, Mousey};
use bevy::hierarchy::{BuildChildren, Children};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Commands, Entity, Query, Transform, TransformBundle, With, Without, Component, EventReader, Parent, Res};
use bevy::time::Time;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody};
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{ActiveEvents, Damping, Sensor, Velocity};
use crate::animations::{Animations, Animator, AnimEnum};
use crate::assets::{SpriteEnum};
use crate::init_systems::environment::DoorInter;
use crate::keyboard_input::PlayerInput;
use crate::player::{DEADZONE, Flippable, InteractEvent, Player, PlayerInteractor, TRUNK_FRICTION};

pub const Y_SCALE_FACTOR: f32 = 0.001;

#[derive(Component, Clone, Copy, Debug)]
pub struct Interactable;

pub fn update_size_on_y(mut query: Query<&mut Transform, With<AutoSizeOnY>>) {
    for mut trans in query.iter_mut() {
        trans.scale = Vec3::ONE * TRUNK_SCALE * (1. - trans.translation.y * Y_SCALE_FACTOR);
    }
}

pub fn auto_sort_on_y(mut query: Query<(&mut Transform, Option<&YOffset>), With<AutoSortOnY>>) {
    for (mut trans, y_off) in query.iter_mut() {
        let y_off = match y_off {
            Some(y_off) => y_off.0,
            None => 0.,
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


pub const MOUSE_TRASH_ANIM_TIME: f32 = 1.;

#[derive(Component)]
pub struct MouseTrashAnimated(pub f32);

pub fn mousey_interact(
    mut ev: EventReader<InteractEvent>,
    mut q: Query<&Parent, (With<Mousey>, With<Interactable>)>,
    mut player: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    for ev in ev.iter() {
        let Ok(parent) = q.get(ev.interactable) else { return; };
        for player in player.iter() {
            commands.entity(player).remove::<Player>();
        }

        commands.entity(parent.get())
            .insert(MouseTrashAnimated(0.));
    }
}

#[derive(Component)]
pub struct BeginAnimPos(pub Vec2);

#[derive(Component)]
pub struct WalkingMouse;

pub fn mouse_trash_animator(
    mut q: Query<(&mut MouseTrashAnimated, &mut Transform, Entity, Option<&BeginAnimPos>)>,
    time: Res<Time>,
    assets: Res<Animations>,
    mut commands: Commands,
) {
    for (mut mouse, mut trans, entity, begin) in q.iter_mut() {
        let Some(begin) = begin else {
            commands.entity(entity).insert(BeginAnimPos(trans.translation.truncate()));
            continue;
        };
        trans.translation.x = begin.0.x - (mouse.0 / MOUSE_TRASH_ANIM_TIME) * 300.;
        trans.translation.y = begin.0.y + mouse_trash_interp(mouse.0 / MOUSE_TRASH_ANIM_TIME) * 150.;

        if mouse.0 > MOUSE_TRASH_ANIM_TIME {
            commands.entity(entity).remove::<MouseTrashAnimated>()
                .remove::<BeginAnimPos>()
                .insert(Player)
                .insert((
                    YOffset(0.),
                    WalkingMouse,
                    PlayerInteractor,
                    Animator::new(assets.map.get(&AnimEnum::MouseyWalk).unwrap().clone()),
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
                ));
            continue;
        }
        mouse.0 += time.delta_seconds();
    }
}

pub fn mouse_walk_anim(
    mut q: Query<&mut Animator, With<WalkingMouse>>,
    input: Res<PlayerInput>,
    anims: Res<Animations>,
) {
    for mut anim in q.iter_mut() {
        if input.movement.length() < DEADZONE {
            anim.play_sprite(SpriteEnum::MouseyWalk1);
        } else {
            anim.play_anim(anims.get(AnimEnum::MouseyWalk));
        }
    }
}

pub fn mouse_idle_anim(
    mut q: Query<(&mut Animator), (With<Mousey>, Without<WalkingMouse>)>,
    anims: Res<Animations>,
) {
    for mut anim in q.iter_mut() {
        anim.play_anim(anims.get(AnimEnum::MouseyIdle));
    }
}

pub const MOUSE_DOOR_HOP_TIME: f32 = 1.;

#[derive(Component)]
pub struct MouseDoorHopAnimated(pub f32);

pub fn door_interact(
    mut ev: EventReader<InteractEvent>,
    q: Query<Entity, (With<DoorInter>, With<Interactable>)>,
    players: Query<Entity, (With<Player>, With<Mousey>)>,
    mut commands: Commands,
) {
    for ev in ev.iter() {
        println!("{:?}", ev);
        let Ok(_) = q.get(ev.interactable) else { return; };
        let Ok(player) = players.get(ev.interactor) else { return; };
        commands.entity(player).remove::<Player>()
            .insert(Sensor)
            .insert(MouseDoorHopAnimated(0.));
    }
}

pub fn mouse_door_anim_player(
    mut q: Query<(&mut MouseDoorHopAnimated, &mut Transform, Entity, Option<&BeginAnimPos>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut mouse, mut trans, entity, begin) in q.iter_mut() {
        let Some(begin) = begin else {
            commands.entity(entity).insert(BeginAnimPos(trans.translation.truncate()));
            continue;
        };
        trans.translation.x = begin.0.x + (mouse.0 / MOUSE_DOOR_HOP_TIME) * 100.;
        trans.translation.y = begin.0.y + mouse_door_hop_interp(mouse.0 / MOUSE_DOOR_HOP_TIME) * 150.;

        if mouse.0 > MOUSE_DOOR_HOP_TIME {
            commands.entity(entity).remove::<MouseDoorHopAnimated>()
                .remove::<BeginAnimPos>()
                .insert(Player)
                .remove::<Sensor>();
            continue;
        }
        mouse.0 += time.delta_seconds();
    }
}

fn mouse_trash_interp(x: f32) -> f32 {
    ((PI - 1.) * x + 1.).sin() - 0.8415
}

fn mouse_door_hop_interp(x: f32) -> f32 {
    (PI * x).sin()
}