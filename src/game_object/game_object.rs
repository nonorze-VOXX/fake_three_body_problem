use crate::game_object::mouse_controller::MouseComponent;
use bevy::{
    ecs::{component::Component, system::Query},
    math::Vec2,
    prelude::*,
    window::PrimaryWindow,
};
#[derive(Component)]
pub struct GameObject {
    sprite_bundle: SpriteBundle,
    mouse_component: MouseComponent,
}
