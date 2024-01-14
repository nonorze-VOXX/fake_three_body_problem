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
        .run();
}
fn create_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn create_a_card(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut transform, card) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * 100.0;
        println!("Card name: {}", card.name);
    }
}
#[derive(Component)]
struct Card {
    name: String,
}
