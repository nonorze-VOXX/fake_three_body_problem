use crate::game_object::game_object::GameObjectBundle;
use bevy::{
    ecs::{component::Component, entity, system::Query},
    input::mouse,
    math::Vec2,
    prelude::*,
    transform,
    window::PrimaryWindow,
};
pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MouseClickEvent>()
            .add_event::<MouseUpsideClickEvent>()
            .add_event::<MouseDownsideClickEvent>()
            .add_systems(Update, update_hover)
            .add_systems(Update, update_up_down)
            .add_systems(Update, update_touch)
            .add_systems(Update, debug_click)
            .add_systems(Update, click_detect);
    }
}

#[derive(Event)]
struct MouseUpsideClickEvent(Entity);
#[derive(Event)]
struct MouseDownsideClickEvent(Entity);
#[derive(Event)]
struct MouseClickEvent(Entity);
#[derive(Component)]
pub struct MouseComponent {
    touched: bool,
    hover: bool,
    upside_triggered: bool,
    downside_triggered: bool,
    trigger: bool,
}
impl Default for MouseComponent {
    fn default() -> Self {
        Self {
            touched: false,
            hover: false,
            upside_triggered: false,
            downside_triggered: false,
            trigger: false,
        }
    }
}

fn debug_click(mut query: Query<(&mut Sprite, &mut MouseComponent)>) {
    query.iter_mut().for_each(|(mut sprite, ouseComponent)| {
        //println!("find sprite and mouse component");
        if (ouseComponent.upside_triggered) {
            println!("upside triggered");
            sprite.color = Color::RED;
        } else if (ouseComponent.downside_triggered) {
            println!("downside triggered");
            sprite.color = Color::BLUE;
        } else if (ouseComponent.touched) {
            sprite.color = Color::GREEN;
        } else {
            sprite.color = Color::WHITE;
        }
    })
}
fn click_detect(
    mut ev_up_click: EventReader<MouseUpsideClickEvent>,
    mut ev_down_click: EventReader<MouseDownsideClickEvent>,
    mut ev_click: EventWriter<MouseClickEvent>,
    mut query: Query<(Entity, &mut MouseComponent)>,
) {
    ev_up_click.read().for_each(|up_event| {
        if let Ok(mut mouse) = query.get_component_mut::<MouseComponent>(up_event.0) {
            mouse.trigger = true;
        }
    });
    ev_down_click.read().for_each(|down_event| {
        if let Ok(mut mouse) = query.get_component_mut::<MouseComponent>(down_event.0) {
            if (mouse.trigger) {
                ev_click.send(MouseClickEvent(down_event.0));
                println!("clicked")
            }
            mouse.trigger = false;
        }
    });
}
fn update_up_down(
    mut ev_down_click: EventWriter<MouseDownsideClickEvent>,
    mut ev_up_click: EventWriter<MouseUpsideClickEvent>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &mut MouseComponent)>,
) {
    query.iter_mut().for_each(|(entity, mut mouse_component)| {
        let upside = buttons.just_pressed(MouseButton::Left) && mouse_component.hover;
        if (upside) {
            ev_up_click.send(MouseUpsideClickEvent(entity));
        }
        mouse_component.upside_triggered = upside;
    });
    query.iter_mut().for_each(|(entity, mut mouse_component)| {
        let downside = buttons.just_released(MouseButton::Left) && mouse_component.hover;
        mouse_component.downside_triggered = downside;
        if (downside) {
            ev_down_click.send(MouseDownsideClickEvent(entity));
        }
    });
}
fn update_touch(buttons: Res<Input<MouseButton>>, mut query: Query<(&mut MouseComponent)>) {
    query.iter_mut().for_each(|(mut mouse_component)| {
        mouse_component.touched = buttons.pressed(MouseButton::Left) && mouse_component.hover;
    });
}

fn update_hover(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Transform, &Sprite, &mut MouseComponent)>,
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
