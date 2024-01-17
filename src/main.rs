use bevy::{
    transform::{self, commands},
    window::PrimaryWindow,
};
use game_object::{
    game_object::GameObjectBundle,
    mouse_controller::{MouseComponent, MousePlugin},
};
use std::{borrow::Borrow, fs::copy, string};

use bevy::{
    core_pipeline::core_2d::graph::input,
    ecs::query,
    input::mouse,
    math::vec2,
    prelude::*,
    render::{mesh, render_resource::resource_macros},
    sprite,
    window::close_on_esc,
};
mod game_object;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MousePlugin)
        .add_systems(Startup, create_world)
        .add_systems(Startup, create_a_button)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, update)
        .run();
}
fn update(mut query: Query<(&mut Sprite, &mut MouseComponent)>) {
    // query.iter_mut().for_each(|(mut sprite, ouseComponent)| {
    //     println!("find sprite and mouse component");
    // })
}

fn create_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn create_a_button(mut commands: Commands) {
    commands.spawn(GameObjectBundle::default());
}
