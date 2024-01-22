use crate::game_object::game_object::GameObjectBundle;
use bevy::{
    ecs::{component::Component, entity, system::Query},
    input::mouse,
    math::Vec3,
    prelude::*,
    transform,
    window::PrimaryWindow,
};
pub struct RigibodyPlugin;
impl Plugin for RigibodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rigibody);
    }
}
#[derive(Component)]
pub struct Rigibody {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
    // gravity: f32,
    // friction: f32,
}
impl Default for Rigibody {
    fn default() -> Self {
        Self {
            velocity: Vec3::new(0., 0., 0.),
            acceleration: Vec3::new(0., 0., 0.),
            mass: 1.,
            // gravity: 0.,
            // friction: 0.,
        }
    }
}
impl Rigibody {
    pub fn add_force(&mut self, force: Vec3) {
        self.acceleration += force / self.mass;
    }
}
fn update_rigibody(time: Res<Time>, mut query: Query<(&mut Transform, &mut Rigibody)>) {
    query.iter_mut().for_each(|(mut transform, mut rigibody)| {
        let delta_velocity = rigibody.acceleration * time.delta_seconds();
        rigibody.velocity += delta_velocity;
        transform.translation += rigibody.velocity * time.delta_seconds();
        rigibody.acceleration = Vec3::new(0., 0., 0.);
    });
}
