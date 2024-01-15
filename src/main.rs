use bevy::{
    transform::{self, commands},
    window::PrimaryWindow,
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, create_world)
        .add_systems(Startup, create_a_button)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, move_card)
        // .add_systems(Update, )
        .add_systems(Update, update_kanban_button)
        .add_systems(Update, handle_click_card)
        .add_systems(Update, clear_trigger)
        .run();
}
fn update_kanban_button(
    mut query: Query<(&Transform, &Sprite, &mut KanbanButton)>,
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    update_button_press(&buttons, q_windows, &mut query);
    update_button_released(&buttons, query);
}
fn clear_trigger(mut query: Query<&mut KanbanButton>, mut commands: Commands) {
    query.iter_mut().for_each(|mut card| {
        if card.trigger {
            create_a_card(&mut commands);
            println!("triggered");
            card.trigger = false;
        }
    })
}
fn update_button_released(
    buttons: &Res<Input<MouseButton>>,
    mut query: Query<(&Transform, &Sprite, &mut KanbanButton)>,
) {
    if !buttons.just_released(MouseButton::Left) {
        return;
    }
    query.iter_mut().for_each(|(_, _, mut card)| {
        if card.clicked {
            card.trigger = true;
            card.clicked = false;
        }
    })
}

fn update_button_press(
    buttons: &Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    query: &mut Query<(&Transform, &Sprite, &mut KanbanButton)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let cursor = q_windows.single().cursor_position();
    if cursor.is_none() {
        return;
    }
    let mouse_pos = mouse_to_world(cursor.unwrap(), &q_windows);
    query
        .iter_mut()
        .for_each(|(transform, sprite, mut button)| {
            let position = transform.translation;
            let size = sprite.custom_size.unwrap();
            if mouse_pos.x < position.x + size.x / 2.
                && mouse_pos.x > position.x - size.x / 2.
                && mouse_pos.y < position.y + size.y / 2.
                && mouse_pos.y > position.y - size.y / 2.
            {
                button.clicked = true;
            }
        });
}
fn create_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    create_a_card(&mut commands);
}
fn create_a_button(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
            ..default()
        },
        KanbanButton { ..default() },
    ));
}

fn create_a_card(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.55, 0.25, 0.25),
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
    mut query: Query<(&mut Transform, &mut Sprite, &Card)>,
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    if q_windows.single().cursor_position().is_none() {
        return;
    }

    let position = q_windows.single().cursor_position().unwrap();

    let clicked_card = query.iter_mut().find(|(_, _, card)| card.clicked);
    if clicked_card.is_none() {
        return;
    }

    let (mut transform, _, _) = clicked_card.unwrap();

    let mouse_pos = mouse_to_world(position, &q_windows);
    transform.translation = mouse_pos;
}
#[derive(Component)]
struct KanbanButton {
    clicked: bool,
    trigger: bool,
}
impl Default for KanbanButton {
    fn default() -> Self {
        KanbanButton {
            clicked: false,
            trigger: false,
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
