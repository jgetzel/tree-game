use bevy::prelude::*;

mod assets;
mod environment;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.run();
}
