use std::{borrow::Borrow, fs::copy};

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, create_world)
        .add_systems(Startup, create_a_card)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, move_card)
        .add_systems(Update, cursor_position)
        .add_systems(Update, handle_click_card)
        .run();
}
fn create_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn create_a_card(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        },
        Card {
            name: "test".to_string(),
            ..default()
        },
    ));
}

fn handle_click_card(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Sprite, &mut Card)>,
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (mut transform, sprite, mut card) in query.iter_mut() {
            if let Some(position) = q_windows.single().cursor_position() {
                let mouse_pos = mouse_to_world(position, &q_windows);
                let position = transform.translation;
                let size = sprite.custom_size.unwrap();
                if mouse_pos.x < position.x + size.x / 2.
                    && mouse_pos.x > position.x - size.x / 2.
                    && mouse_pos.y < position.y + size.y / 2.
                    && mouse_pos.y > position.y - size.y / 2.
                {
                    card.clicked = true;
                    break;
                }
            }
        }
    }
    if buttons.just_released(MouseButton::Left) {
        for (mut transform, sprite, mut card) in query.iter_mut() {
            if card.clicked {
                card.clicked = false;
                break;
            }
        }
    }
}
fn mouse_to_world(position: Vec2, q_windows: &Query<&Window, With<PrimaryWindow>>) -> Vec3 {
    return Vec3::new(
        position.x - q_windows.single().width() / 2.,
        (position.y - q_windows.single().height() / 2.) * -1.,
        0.,
    );
}
fn move_card(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Sprite, &Card)>,
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.pressed(MouseButton::Left) {
        for (mut transform, sprite, card) in query.iter_mut() {
            if card.clicked {
                if let Some(position) = q_windows.single().cursor_position() {
                    let mouse_pos = mouse_to_world(position, &q_windows);
                    transform.translation = mouse_pos;
                    break;
                }
            }
        }
    }
}
#[derive(Component)]
struct Card {
    name: String,
    clicked: bool,
}
impl Default for Card {
    fn default() -> Self {
        Card {
            name: "default".to_string(),
            clicked: false,
        }
    }
}

use bevy::window::PrimaryWindow;

fn cursor_position(q_windows: Query<&Window, With<PrimaryWindow>>) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        //println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        //println!("Cursor is not in the game window.");
    }
}
