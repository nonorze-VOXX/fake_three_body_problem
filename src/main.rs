use bevy::{
    core_pipeline::core_2d::graph::input,
    ecs::query,
    prelude::*,
    render::{mesh, render_resource::resource_macros},
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
        },
    ));
}
fn move_card(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Card)>,
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.pressed(MouseButton::Left) {
        for (mut transform, card) in query.iter_mut() {
            if let Some(position) = q_windows.single().cursor_position() {
                println!("Cursor is inside the primary window, at {:?}", position);
                transform.translation = Vec3::new(
                    position.x - q_windows.single().width() / 2.,
                    (position.y - q_windows.single().height() / 2.) * -1.,
                    0.,
                );
            } else {
                println!("Cursor is not in the game window.");
            }
            //println!("Card name: {}", card.name);
        }
    }
}
#[derive(Component)]
struct Card {
    name: String,
}

use bevy::window::PrimaryWindow;

fn cursor_position(q_windows: Query<&Window, With<PrimaryWindow>>) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}
