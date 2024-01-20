use bevy::{
    transform::{self, commands},
    utils::tracing::instrument::WithSubscriber,
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
        .add_systems(Startup, create_something)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, update)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, update_gravity_receive)
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

#[derive(Component)]
struct Enemy {}
#[derive(Component)]
struct Player {}
#[derive(Component)]
struct GravityEffectEntity {}
#[derive(Component)]
struct GravityReceiveEntity {}
fn create_something(mut commands: Commands) {
    commands.spawn((
        GameObjectBundle::default(),
        Enemy {},
        GravityReceiveEntity {},
        GravityEffectEntity {},
    ));
    commands.spawn((
        GameObjectBundle::default(),
        Enemy {},
        GravityReceiveEntity {},
        GravityEffectEntity {},
    ));
    commands.spawn((
        GameObjectBundle::default(),
        Player {},
        GravityEffectEntity {},
    ));
}

fn update_gravity_receive(
    time: Res<Time>,
    // mut query1: Query<(&mut Transform), With<GravityEffectEntity>>,
    mut query: Query<(
        &mut Transform,
        &mut GravityEffectEntity,
        Option<&mut GravityReceiveEntity>,
    )>,
) {
    let mut c = query.iter_combinations_mut();
    while let Some([mut a1, mut a2]) = c.fetch_next() {
        if a1.2.is_some() {
            let distance = a2.0.translation - a1.0.translation;

            if distance.length() > 0.0f32 {
                let force = 1.0 / distance.length() * distance.length();
                println!("force: {}", force);
                // let a = force / 1.0;
                // let delta = a * time.delta_seconds();
                let g = 300.0;
                a1.0.translation += distance.normalize() * force * g * time.delta_seconds();
            }
        }
    }
}
fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &mut Player)>) {
    if keys.pressed(KeyCode::W) {
        query.iter_mut().for_each(|(mut transform, _player)| {
            transform.translation.y += 5.0;
        });
    }
    if keys.pressed(KeyCode::S) {
        query.iter_mut().for_each(|(mut transform, _player)| {
            transform.translation.y -= 5.0;
        });
    }
    if keys.pressed(KeyCode::A) {
        query.iter_mut().for_each(|(mut transform, _player)| {
            transform.translation.x -= 5.0;
        });
    }
    if keys.pressed(KeyCode::D) {
        query.iter_mut().for_each(|(mut transform, _player)| {
            transform.translation.x += 5.0;
        });
    }
}
