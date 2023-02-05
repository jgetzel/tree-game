use bevy::prelude::{Plugin, Component};
use bevy::{utils::HashMap, prelude::Resource};

use crate::assets::SpriteEnum;
use crate::assets::SpriteEnum::*;


pub struct AnimPlugin;

impl Plugin for AnimPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Animations::default());
    }
}

#[derive(Copy, Clone)]
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
}

#[derive(Resource)]
pub struct Animations {
    pub map: HashMap<AnimEnum, Animation>
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

        Self { map }
    }
}

#[derive(Component)]
pub struct Animator {
    time: f32,
    current_anim: Animation,
    playing: bool,
    current_frame: SpriteEnum,
}

impl Animator {
    pub fn play_anim(&mut self, anim: Animation) {
        self.time = 0.;
        self.current_anim = anim;
        self.playing = true;
        self.current_frame = anim.sprites[0];
    }
}