use crate::game_object::game_object::GameObject;
use bevy::{
    ecs::{component::Component, system::Query},
    math::Vec2,
    prelude::*,
    transform,
    window::PrimaryWindow,
};

#[derive(Component)]
pub struct MouseComponent {
    clicked: bool,
    hover: bool,
    upside_triggered: bool,
    downside_triggered: bool,
}
impl Default for MouseComponent {
    fn default() -> Self {
        Self {
            clicked: false,
            hover: false,
            upside_triggered: false,
            downside_triggered: false,
        }
    }
}
fn update_click(
    buttons: &Res<Input<MouseButton>>,
    query: &mut Query<(&Transform, &Sprite, &mut MouseComponent)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        query.iter_mut().for_each(|(_, _, mut mouse_component)| {
            mouse_component.clicked = mouse_component.hover;
            mouse_component.upside_triggered = mouse_component.hover;
        });
    } else {
        query.iter_mut().for_each(|(_, _, mut mouse_component)| {
            mouse_component.upside_triggered = false;
        });
    }
    if buttons.just_released(MouseButton::Left) {
        query.iter_mut().for_each(|(_, _, mut mouse_component)| {
            mouse_component.downside_triggered = mouse_component.hover && mouse_component.clicked;
            mouse_component.clicked = false;
        });
    } else {
        query.iter_mut().for_each(|(_, _, mut mouse_component)| {
            mouse_component.downside_triggered = false;
        });
    }
}

fn update_hover(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    query: &mut Query<(&Transform, &Sprite, &mut MouseComponent)>,
) {
    let cursor = q_windows.single().cursor_position();
    if cursor.is_none() {
        return;
    }
    let mouse_pos = mouse_to_world(cursor.unwrap(), &q_windows);
    query
        .iter_mut()
        .for_each(|(transform, sprite, mut mouse_component)| {
            let position = transform.translation;
            let size = sprite.custom_size.unwrap();
            let hover = mouse_pos.x < position.x + size.x / 2.
                && mouse_pos.x > position.x - size.x / 2.
                && mouse_pos.y < position.y + size.y / 2.
                && mouse_pos.y > position.y - size.y / 2.;
            mouse_component.hover = hover;
        });
}

fn mouse_to_world(position: Vec2, q_windows: &Query<&Window, With<PrimaryWindow>>) -> Vec3 {
    return Vec3::new(
        position.x - q_windows.single().width() / 2.,
        (position.y - q_windows.single().height() / 2.) * -1.,
        0.,
    );
}
