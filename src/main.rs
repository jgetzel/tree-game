use crate::assets::AssetLoaderPlugin;
use crate::environment_init::EnvironmentInitPlugin;
use crate::keyboard_input::KeyboardInputPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use crate::player::{auto_sort_on_y, move_player, reinsert_colliders, update_size_on_y};

mod assets;
mod camera;
mod environment_init;
mod keyboard_input;
mod player;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(DebugPlugins)
        .add_plugin(AssetLoaderPlugin)
        .add_plugin(EnvironmentInitPlugin)
        .add_plugin(KeyboardInputPlugin);

    app.add_system(move_player)
        .add_system(update_size_on_y)
        .add_system(auto_sort_on_y)
        .add_system(reinsert_colliders);
    
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
