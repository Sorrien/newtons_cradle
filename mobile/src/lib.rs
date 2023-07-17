use bevy::prelude::*;
use newtons_cradle::GamePlugin;

#[bevy_main]
fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run()
}
