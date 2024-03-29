use bevy::{
    transform::{self, commands},
    ui::widget,
    utils::tracing::instrument::WithSubscriber,
    window::PrimaryWindow,
};
use game_object::{
    game_object::GameObjectBundle,
    mouse_controller::{MouseComponent, MousePlugin},
    rigibody::{self, Rigibody, RigibodyPlugin},
};

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
        .add_plugins(RigibodyPlugin)
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
    const width: f32 = 20.0;

    // const wall_width: f32 = 20.0;
    // commands.spawn((GameObjectBundle {
    //     sprite_bundle: SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0., 0., 0.),
    //             custom_size: Some(Vec2::new(10., 10.)),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
    //         ..default()
    //     },
    //     mouse_component: MouseComponent::default(),
    //     rigibody: Rigibody {
    //         velocity: Vec3::new(0., 0., 0.),
    //         size: Vec3::new(2., 10., 0.),
    //         ..default()
    //     },
    // },));
    commands.spawn((
        GameObjectBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 0., 0.),
                    custom_size: Some(Vec2::new(width, width)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 50., 0.)),
                ..default()
            },
            mouse_component: MouseComponent::default(),
            rigibody: Rigibody {
                velocity: Vec3::new(0., 0., 0.),
                size: Vec3::new(width, width, width),
                ..default()
            },
        },
        Enemy {},
        GravityReceiveEntity {},
        GravityEffectEntity {},
    ));
    commands.spawn((
        GameObjectBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 0., 0.),
                    custom_size: Some(Vec2::new(width, width)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            mouse_component: MouseComponent::default(),
            rigibody: Rigibody {
                velocity: Vec3::new(0., 0., 0.),
                size: Vec3::new(width, width, width),
                ..default()
            },
        },
        Enemy {},
        GravityReceiveEntity {},
        GravityEffectEntity {},
    ));
    commands.spawn((
        GameObjectBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 0., 0.),
                    custom_size: Some(Vec2::new(width, width)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
                ..default()
            },
            mouse_component: MouseComponent::default(),
            rigibody: Rigibody {
                velocity: Vec3::new(0., 0., 0.),
                size: Vec3::new(width, width, width),
                mass: 1000000.,
                ..default()
            },
        },
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
        &mut Rigibody,
    )>,
) {
    let mut c = query.iter_combinations_mut();
    while let Some([mut a1, mut a2]) = c.fetch_next() {
        if a1.2.is_some() {
            let distance = a2.0.translation - a1.0.translation;

            if distance.length() > a2.3.size.length() {
                let forceValue = 50000.0 / (distance.length());
                // let forceValue = 1000000.0 / (distance.length() * distance.length());
                let force = distance.normalize() * forceValue;
                a1.3.add_force(force);
                a2.3.add_force(-force);
            }
        }
    }
}
fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Rigibody, &mut Player)>,
) {
    if keys.pressed(KeyCode::W) {
        query
            .iter_mut()
            .for_each(|(mut transform, mut rigibody, _player)| {
                rigibody.velocity = Vec3::new(0., 100., 0.);
            });
    } else if keys.pressed(KeyCode::A) {
        query
            .iter_mut()
            .for_each(|(mut transform, mut rigibody, _player)| {
                rigibody.velocity = Vec3::new(-100., 0., 0.);
            });
    } else if keys.pressed(KeyCode::S) {
        query
            .iter_mut()
            .for_each(|(mut transform, mut rigibody, _player)| {
                rigibody.velocity = Vec3::new(0., -100., 0.);
            });
    } else if keys.pressed(KeyCode::D) {
        query
            .iter_mut()
            .for_each(|(mut transform, mut rigibody, _player)| {
                rigibody.velocity = Vec3::new(100., 0., 0.);
            });
    } else {
        query
            .iter_mut()
            .for_each(|(mut transform, mut rigibody, _player)| {
                rigibody.velocity = Vec3::new(0., 0., 0.);
            });
    }
}
