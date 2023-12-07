use std::collections::VecDeque;

use bevy::{prelude::*, reflect::List};
use bevy_egui::{
    egui::{self, Pos2},
    EguiContexts, EguiPlugin,
};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
// const BOX_OLOR: Color = Color::rgb(0.3, 0.3, 0.7);
fn main() {
    println!("Hello, world!");

    let _run = App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window { ..default() }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),))
        .add_plugins(EguiPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .insert_resource(Task::new())
        .add_systems(Update, displayByUi)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Resource)]
pub struct Task {
    id: i32,
    name: VecDeque<String>,
    op: Option<String>,
}
impl Task {
    pub fn new() -> Self {
        Self {
            id: 10,
            name: VecDeque::new(),
            op: Some(String::from("s")),
        }
    }
    pub fn change_id(&mut self, new_id: i32) {
        self.id = new_id;
    }
    pub fn add(&mut self, text: String) {
        self.name.push_back(text);
    }
}
fn displayByUi(mut contexts: EguiContexts, mut task: ResMut<Task>) {
    const WIN_WIDTH: f32 = 300.0;
    const WIN_HEIGHT: f32 = 100.0;
    if (task.name.is_empty()) {
        task.add(String::from("amogus"));
    }
    if (task.op.is_none()) {
        task.op = task.name.pop_front();
    }
    egui::Window::new("test")
        .default_pos(Pos2 { x: 0.0, y: 0.0 })
        .resizable(false)
        .title_bar(true)
        .constrain(false)
        .movable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width(WIN_WIDTH);
            ui.set_max_height(WIN_HEIGHT);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                let text = egui::Label::new(task.op.as_ref().unwrap()).wrap(true);
                let height = ui.add(text).rect.size().y;
                ui.set_max_height(height);
            });
        });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // commands.spawn();
}
