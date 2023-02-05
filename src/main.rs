use crate::assets::AssetLoaderPlugin;
use crate::init_systems::{EnvironmentInitPlugin, YOffset};
use crate::keyboard_input::KeyboardInputPlugin;
use animations::AnimPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use utils::{auto_sort_on_y, reinsert_colliders, update_size_on_y};
use crate::camera::camera_follow;
use crate::player::{flip_flippables, move_player};

mod assets;
mod camera;
mod init_systems;
mod keyboard_input;
mod player;
mod utils;
mod animations;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins
        .set(WindowPlugin {
            window: WindowDescriptor {
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            },
            ..default()
        })
        .build().add_before::<AssetPlugin, _>(EmbeddedAssetPlugin))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(DebugPlugins)
        .add_plugin(AssetLoaderPlugin)
        .add_plugin(AnimPlugin)
        .add_plugin(EnvironmentInitPlugin)
        .add_plugin(KeyboardInputPlugin);

    app.add_system(move_player)
        .add_system(camera_follow)
        .add_system(flip_flippables)
        .add_system(update_size_on_y)
        .add_system(auto_sort_on_y)
        .add_system(reinsert_colliders);

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
