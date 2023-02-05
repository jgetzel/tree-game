use bevy::prelude::{Plugin, Component, Query, Res, Image, Handle};
use bevy::reflect::{Reflect, GetTypeRegistration};
use bevy::time::Time;
use bevy::{utils::HashMap, prelude::Resource};

use crate::assets::{SpriteEnum, GameAssets};
use crate::assets::SpriteEnum::*;


pub struct AnimPlugin;

impl Plugin for AnimPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Animations::default())
        .add_system(animator_sys);
    }
}

#[derive(Clone)]
pub struct Animation {
    pub sprites: Vec<SpriteEnum>,
    pub framerate: f32,
    pub one_shot: bool
}

#[derive(Eq,PartialEq, Hash)]
pub enum AnimEnum {
    MouseyWalk,
    MouseyIdle,
    BugWalk,
    TrunkWalk,
}

#[derive(Resource)]
pub struct Animations {
    pub map: HashMap<AnimEnum, Animation>
}

impl Animations {
    pub fn get(&self, anim_enum: AnimEnum) -> Animation {
        self.map.get(&anim_enum).unwrap().clone()
    }
}

impl Default for Animations {
    fn default() -> Self {
        let mut map: HashMap<AnimEnum, Animation> = HashMap::new();
        map.insert(AnimEnum::MouseyIdle, Animation { 
            sprites: vec![MouseyIdle1, MouseyIdle2, MouseyIdle3], 
            framerate: 6., 
            one_shot: false 
        });
        map.insert(AnimEnum::MouseyWalk, Animation { 
            sprites: vec![MouseyWalk1, MouseyWalk2, MouseyWalk3, MouseyWalk4, MouseyWalk5], 
            framerate: 8., 
            one_shot: false 
        });
        map.insert(AnimEnum::BugWalk, Animation { 
            sprites: vec![Bug1, Bug2, Bug3, Bug4],
             framerate: 8.,
              one_shot: false 
            });
        map.insert(AnimEnum::TrunkWalk, Animation { 
            sprites: vec![TrunkWalk1, TrunkWalk2, TrunkWalk3, TrunkWalk4, TrunkWalk5, TrunkWalk6],
                framerate: 8.,
                one_shot: false 
            });

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
            current_frame: 0
        }
    }

    pub fn play_anim(&mut self, anim: Animation) {
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
            sprites: vec![sprite],
            framerate: 60.,
            one_shot: true
        };
        self.time = 0.;
        self.current_frame = 0;
    }
}

pub fn animator_sys(
    mut animators: Query<(&mut Animator, &mut Handle<Image>)>,
    assets: Res<GameAssets>,
    time: Res<Time>
) {
    for (mut animator, mut handle) in animators.iter_mut() {
        if !animator.playing {
            return;
        }
        let framerate = animator.current_anim.framerate;
        let anim_length = animator.current_anim.sprites.len();
        let time_per_frame = anim_length as f32 / framerate;
        
        if time_per_frame * (animator.current_frame as f32 + 1.) < animator.time {
            animator.current_frame += 1;
            animator.current_frame %= anim_length;
            *handle = assets.get(animator.current_anim.sprites[animator.current_frame]);
        }
        
        animator.time += time.delta_seconds();
    }
}