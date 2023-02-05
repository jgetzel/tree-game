use bevy::ecs::schedule::ShouldRun::No;
use crate::{
    animations::{AnimEnum, Animations, Animator},
    assets::SpriteEnum,
    keyboard_input::PlayerInput,
};
use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{CollisionEvent, Sensor, Velocity};
use crate::utils::Interactable;

pub const TRUNK_ACCEL: f32 = 4000.;
pub const TRUNK_MAX_SPEED: f32 = 100.;
pub const TRUNK_FRICTION: f32 = 15.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Trunk;

#[derive(Component)]
pub struct Flippable {
    pub right_facing: bool,
}

#[derive(Component)]
pub struct PlayerInteractor;

#[derive(Debug)]
pub struct InteractEvent {
    pub interactor: Entity,
    pub interactable: Entity
}

pub const DEADZONE: f32 = 0.15;

pub fn move_player(
    mut query: Query<&mut Velocity, With<Player>>,
    input: Res<PlayerInput>,
    time: Res<Time>,
) {
    for mut velocity in query.iter_mut() {
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

pub fn flip_interactor(
    mut query: Query<(Entity, &Parent), With<PlayerInteractor>>,
    sprite_q: Query<&Sprite, (With<Player>, Without<PlayerInteractor>)>,
    mut commands: Commands,
) {
    for (entity, parent) in query.iter_mut() {
        let Ok(sprite) = sprite_q.get(parent.get())
            else { return; };
        let factor = if sprite.flip_x { -1. } else { 1. };
        commands.entity(entity).despawn();
        commands.entity(parent.get()).with_children(|p| {
            p.spawn(PlayerInteractor)
                .insert(Collider::ball(700.))
                .insert(Sensor)
                .insert(TransformBundle::from(Transform::from_xyz(20. * factor, 0., 0.)));
        });
    }
}

pub fn flip_flippables(mut query: Query<(&Velocity, &mut Sprite, &Flippable)>) {
    for (vel, mut sprite, flip) in query.iter_mut() {
        if vel.linvel.x.abs() > DEADZONE {
            let flip_inv = if flip.right_facing { 1. } else { -1. };
            sprite.flip_x = vel.linvel.x * flip_inv < 0.
        }
    }
}

pub fn player_anim_controller(
    mut query: Query<(&mut Animator, Option<&Player>), With<Trunk>>,
    input: Res<PlayerInput>,
    anims: Res<Animations>,
) {
    for (mut anim , player) in query.iter_mut() {
        if input.movement.length() > DEADZONE && player.is_some() {
            anim.play_anim(anims.get(AnimEnum::TrunkWalk));
        } else {
            anim.play_anim(anims.get(AnimEnum::TrunkIdle));
        }
    }
}

#[derive(Component)]
pub struct Interacting {
    pub entity: Entity
}

pub fn interact_col_event_sys(
    mut col_events: EventReader<CollisionEvent>,
    interactor_q: Query<(&PlayerInteractor, Option<&Parent>)>,
    interactable_q: Query<(&Interactable, Option<&Parent>)>,
    mut commands: Commands
) {
    for ev in col_events.iter() {
        match ev {
            CollisionEvent::Started(e1, e2, _) => {
                let Some((&interactor, parent)) = [e1, e2]
                    .iter().find_map(|&v| {
                    return match interactor_q.get(*v) {
                        Ok((_, parent)) => Some((v, parent)),
                        _ => None,
                    };
                })
                    else { return; };
                let Some((&interactable, i_parent)) = [e1, e2]
                    .iter().find_map(|&v| {
                    return match interactable_q.get(*v) {
                        Ok((_, parent)) => Some((v, parent)),
                        _ => None,
                    };
                })
                    else { return; };

                let interacting_ent = match parent {
                    Some(parent) => parent.get(),
                    None => interactor
                };

                if let Some(p) = i_parent {
                    if commands.entity(p.get()).id() == commands.entity(interacting_ent).id() {
                        return;
                    }
                }

                commands.entity(interacting_ent).insert(Interacting {
                    entity: interactable
                });
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                let Some((&interactor, parent)) = [e1, e2]
                    .iter().find_map(|&v| {
                    return match interactor_q.get(*v) {
                        Ok((_, parent)) => Some((v, parent)),
                        _ => None,
                    };
                }) else { return; };

                let interacting_ent = match parent {
                    Some(parent) => parent.get(),
                    None => interactor
                };

                commands.entity(interacting_ent).remove::<Interacting>();
            }
        }
    }
}

pub fn interact_events_pt2(
    mut evt_writer: EventWriter<InteractEvent>,
    q: Query<(Entity, &Interacting), With<Player>>,
    input: Res<PlayerInput>,
) {
    for (ent, interacting) in q.iter() {

        if input.just_interacted {
            evt_writer.send(InteractEvent {
                interactor: ent,
                interactable: interacting.entity,
            });
        }
    }
}
