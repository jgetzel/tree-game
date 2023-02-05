use bevy::prelude::{Component, Handle, Image, Plugin, Query, Res};
use bevy::reflect::{GetTypeRegistration, Reflect};
use bevy::time::Time;
use bevy::{prelude::Resource, utils::HashMap};
use lerp::num_traits::Zero;

use crate::assets::SpriteEnum::*;
use crate::assets::{GameAssets, SpriteEnum};

pub struct AnimPlugin;

impl Plugin for AnimPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Animations::default())
            .add_system(animator_sys);
    }
}

#[derive(Clone)]
pub struct Animation {
    pub anim_enum: AnimEnum,
    pub sprites: Vec<SpriteEnum>,
    pub framerate: f32,
    pub one_shot: bool,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum AnimEnum {
    MouseyWalk,
    MouseyIdle,
    BugWalk,
    TrunkWalk,
    TrunkIdle,
    StaticSprite,
}

#[derive(Resource)]
pub struct Animations {
    pub map: HashMap<AnimEnum, Animation>,
}

impl Animations {
    pub fn get(&self, anim_enum: AnimEnum) -> Animation {
        self.map.get(&anim_enum).unwrap().clone()
    }
}

impl Default for Animations {
    fn default() -> Self {
        let mut map: HashMap<AnimEnum, Animation> = HashMap::new();
        map.insert(
            AnimEnum::MouseyIdle,
            Animation {
                anim_enum: AnimEnum::MouseyIdle,
                sprites: vec![MouseyIdle1, MouseyIdle2, MouseyIdle3],
                framerate: 6.,
                one_shot: false,
            },
        );
        map.insert(
            AnimEnum::MouseyWalk,
            Animation {
                anim_enum: AnimEnum::MouseyWalk,
                sprites: vec![
                    MouseyWalk1,
                    MouseyWalk2,
                    MouseyWalk3,
                    MouseyWalk4,
                    MouseyWalk5,
                ],
                framerate: 8.,
                one_shot: false,
            },
        );
        map.insert(
            AnimEnum::BugWalk,
            Animation {
                anim_enum: AnimEnum::BugWalk,
                sprites: vec![Bug1, Bug2, Bug3, Bug4],
                framerate: 8.,
                one_shot: false,
            },
        );
        map.insert(
            AnimEnum::TrunkWalk,
            Animation {
                anim_enum: AnimEnum::TrunkWalk,
                sprites: vec![
                    TrunkWalk2, TrunkWalk1, TrunkWalk3, TrunkWalk1,
                ],
                framerate: 6.,
                one_shot: false,
            },
        );
        map.insert(
            AnimEnum::TrunkIdle,
            Animation {
                anim_enum: AnimEnum::TrunkIdle,
                sprites: vec![
                    TrunkIdle1, TrunkIdle2, TrunkIdle3, TrunkIdle2,
                ],
                framerate: 6.,
                one_shot: false,
            },
        );

        Self { map }
    }
}

#[derive(Component, Clone)]
pub struct Animator {
    pub time: f32,
    pub current_anim: Animation,
    pub playing: bool,
    pub current_frame: usize,
}

impl Animator {
    pub fn new(anim: Animation) -> Self {
        Self {
            time: 0.,
            current_anim: anim,
            playing: false,
            current_frame: 0,
        }
    }

    pub fn play_anim(&mut self, anim: Animation) {
        if !anim.one_shot && self.current_anim.anim_enum == anim.anim_enum && self.playing {
            return;
        }
        self.time = 0.;
        self.current_anim = anim;
        self.playing = true;
        self.current_frame = 0;
    }

    pub fn pause(&mut self) {
        self.playing = false;
    }

    pub fn resume(&mut self) {
        self.playing = true;
    }

    pub fn play_sprite(&mut self, sprite: SpriteEnum) {
        self.playing = true;
        self.current_anim = Animation {
            anim_enum: AnimEnum::StaticSprite,
            sprites: vec![sprite],
            framerate: 1.,
            one_shot: true,
        };
        self.time = 0.;
        self.current_frame = 0;
    }
}

pub fn animator_sys(
    mut animators: Query<(&mut Animator, &mut Handle<Image>)>,
    assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (mut animator, mut handle) in animators.iter_mut() {
        if !animator.playing {
            return;
        }
        if animator.time.is_zero() {
            *handle = assets.get(animator.current_anim.sprites[0]);
        }

        let framerate = animator.current_anim.framerate;
        let anim_length = animator.current_anim.sprites.len();
        let time_per_frame = 1. / framerate;
        let time_to_next_sprite = time_per_frame * (animator.current_frame as f32 + 1.);
        if time_to_next_sprite < animator.time {
            animator.current_frame += 1;
            animator.current_frame %= anim_length;
            if animator.current_frame == 0 {
                animator.time %= time_per_frame * anim_length as f32;
            }
            *handle = assets.get(animator.current_anim.sprites[animator.current_frame]);
        }

        animator.time += time.delta_seconds();
    }
}
