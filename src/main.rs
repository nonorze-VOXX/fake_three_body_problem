use bevy::{prelude::*, render::render_resource::resource_macros, window::close_on_esc};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, add_Task)
        .add_systems(Update, show_task)
        .add_systems(Update, close_on_esc)
        .run();
}

fn show_task(mut contexts: EguiContexts, query: Query<&Task>) {
    for task in &query {
        egui::Window::new(task.0.to_string()).show(contexts.ctx_mut(), |ui| {
            ui.label(task.0.to_string());
        });
    }
}
#[derive(Component)]
struct Task(String);
fn add_Task(mut commands: Commands) {
    commands.spawn((Task("Elaina Proctor".to_string())));
    commands.spawn((Task("Renzo Hume".to_string())));
    commands.spawn((Task("Zayna Nieves".to_string())));
}
