use crate::assets::AssetLoaderPlugin;
use crate::init_systems::{EnvironmentInitPlugin, YOffset};
use crate::keyboard_input::KeyboardInputPlugin;
use animations::{AnimPlugin, Animator};
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use crate::camera::camera_follow;
use crate::player::{interact_col_event_sys, flip_flippables, flip_interactor, move_player, InteractEvent, interact_events_pt2};
use player::player_anim_controller;
use utils::{auto_sort_on_y, reinsert_colliders, update_size_on_y};
use crate::utils::{attack_system, door_interact, mouse_door_anim_player, mouse_idle_anim, mouse_trash_animator, mouse_walk_anim, mousey_interact};

mod animations;
mod assets;
mod camera;
mod init_systems;
mod keyboard_input;
mod player;
mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                },
                ..default()
            })
            .build()
            .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
    )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(DebugPlugins)
        .add_plugin(AssetLoaderPlugin)
        .add_plugin(AnimPlugin)
        .add_plugin(EnvironmentInitPlugin)
        .add_plugin(KeyboardInputPlugin);

    app.add_system(move_player)
        .add_system(player_anim_controller)
        .add_system(attack_system)
        .add_system(mouse_walk_anim)
        .add_system(mouse_idle_anim)
        .add_system(camera_follow)
        .add_system(mousey_interact)
        .add_system(door_interact)
        .add_system(mouse_trash_animator)
        .add_system(mouse_door_anim_player)
        .add_system(flip_flippables)
        .add_system(flip_interactor)
        .add_system(interact_col_event_sys)
        .add_system(interact_events_pt2)
        .add_system(update_size_on_y)
        .add_system(auto_sort_on_y)
        .add_system(reinsert_colliders);

    app.add_event::<InteractEvent>();

    app.register_type::<YOffset>();

    app.run();
}

struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[cfg(debug_assertions)]
        return PluginGroupBuilder::start::<Self>()
            .add(WorldInspectorPlugin)
            .add(RapierDebugRenderPlugin::default())
            .build();

        #[cfg(not(debug_assertions))]
        return PluginGroupBuilder::start::<Self>();
    }
}
