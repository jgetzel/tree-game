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
    HouseFrontHouse,
    HouseFrontBackground,
    HouseFrontTree1,
    HouseFrontTree2,
    HouseInside,
    LadyIdle,
    MouseyIdle1,
    MouseyIdle2,
    MouseyIdle3,
    MouseyWalk1,
    MouseyWalk2,
    MouseyWalk3,
    MouseyWalk4,
    MouseyWalk5,
    Bug1,
    Bug2,
    Bug3,
    Bug4,
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
        SpriteEnum::HouseFrontBackground,
        asset_server.load("sprites/house_front_background.png"),
    );
    assets.map.insert(
        SpriteEnum::HouseFrontHouse,
        asset_server.load("sprites/house_front_house.png")
    );
    assets.map.insert(
        SpriteEnum::HouseFrontTree1,
        asset_server.load("sprites/house_front_tree_1.png")
    );
    assets.map.insert(
        SpriteEnum::HouseFrontTree2,
        asset_server.load("sprites/house_front_tree_2.png")
    );
    assets.map.insert(
        SpriteEnum::HouseInside,
        asset_server.load("sprites/house_inside.png")
    );
    assets.map.insert(
        SpriteEnum::LadyIdle,
        asset_server.load("sprites/lady_idle.png")
    );
    assets.map.insert(
        SpriteEnum::MouseyWalk1,
        asset_server.load("sprites/mousey/mouseywalk1.png")
    );
    assets.map.insert(
        SpriteEnum::MouseyWalk2,
        asset_server.load("sprites/mousey/mouseywalk2.png")
    );
    assets.map.insert(
        SpriteEnum::MouseyWalk3,
        asset_server.load("sprites/mousey/mouseywalk3.png")
    );
    assets.map.insert(
        SpriteEnum::MouseyWalk4,
        asset_server.load("sprites/mousey/mouseywalk4.png")
    );
    assets.map.insert(
        SpriteEnum::MouseyWalk5,
        asset_server.load("sprites/mousey/mouseywalk5.png")
    );
    assets.map.insert(
        SpriteEnum::MouseyIdle1,
        asset_server.load("sprites/mousey/mousey_idle1.png")
    );
    assets.map.insert(
        SpriteEnum::MouseyIdle2,
        asset_server.load("sprites/mousey/mousey_idle2.png")
    );
    assets.map.insert(
        SpriteEnum::MouseyIdle3,
        asset_server.load("sprites/mousey/mousey_idle3.png")
    );
    assets.map.insert(
        SpriteEnum::Bug1,
        asset_server.load("sprites/bug/buggyboo1.png")
    );
    assets.map.insert(
        SpriteEnum::Bug2,
        asset_server.load("sprites/bug/buggyboo2.png")
    );
    assets.map.insert(
        SpriteEnum::Bug3,
        asset_server.load("sprites/bug/buggyboo3.png")
    );
    assets.map.insert(
        SpriteEnum::Bug4,
        asset_server.load("sprites/bug/buggyboo4.png")
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
