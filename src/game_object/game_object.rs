use crate::game_object::mouse_controller::MouseComponent;
use bevy::{
    ecs::{component::Component, system::Query},
    math::Vec2,
    prelude::*,
    window::PrimaryWindow,
};
#[derive(Bundle)]
pub struct GameObjectBundle {
    pub sprite_bundle: SpriteBundle,
    pub mouse_component: MouseComponent,
}

impl Default for GameObjectBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.55, 0.25, 0.25),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
                ..default()
            },
            mouse_component: MouseComponent::default(),
        }
    }
}
