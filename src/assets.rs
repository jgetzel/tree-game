use bevy::asset::LoadState;
use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    InGame,
}

#[derive(Default, Resource)]
pub struct GameAssets {
    pub map: HashMap<SpriteEnum, Handle<Image>>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Loading)
            .insert_resource(GameAssets::default())
            .insert_resource(AssetsLoading::default())
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_assets))
            .add_system_set(
                SystemSet::on_update(AppState::Loading).with_system(check_assets_loaded),
            );
    }
}

#[derive(Default, Resource)]
pub struct AssetsLoading(Vec<HandleUntyped>);

#[derive(PartialEq, Eq, Hash)]
pub enum SpriteEnum {
    TrunkJr,
    HouseFront,
    DebugCircle
}

pub fn load_assets(
    mut assets: ResMut<GameAssets>,
    mut loading: ResMut<AssetsLoading>,
    asset_server: Res<AssetServer>,
) {
    assets.map.insert(
        SpriteEnum::TrunkJr,
        asset_server.load("sprites/trunk-jr.png"),
    );
    assets.map.insert(
        SpriteEnum::HouseFront,
        asset_server.load("sprites/house_front.png"),
    );
    assets.map.insert(
        SpriteEnum::DebugCircle,
        asset_server.load("sprites/debug_circle.png")
    );

    for (_, asset) in assets.map.iter() {
        loading.0.push(asset.clone_untyped());
    }
}

pub fn check_assets_loaded(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    match server.get_group_load_state(loading.0.iter().map(|handle| handle.id)) {
        LoadState::Failed => {}
        LoadState::Loaded => {
            commands.remove_resource::<AssetsLoading>();
            state.set(AppState::InGame).unwrap();
        }
        _ => {}
    }
}
