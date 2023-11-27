use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
// const BOX_OLOR: Color = Color::rgb(0.3, 0.3, 0.7);
fn main() {
    println!("Hello, world!");

    let _run = App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct GroupBox;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn();
}
