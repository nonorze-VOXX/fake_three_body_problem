use crate::game_object::game_object::GameObjectBundle;
use bevy::{
    ecs::{component::Component, entity, system::Query},
    input::mouse,
    math::{self, Vec3},
    prelude::*,
    transform,
    window::PrimaryWindow,
};
pub struct RigibodyPlugin;
impl Plugin for RigibodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rigibody);
        app.add_systems(Update, update_collider);
    }
}
#[derive(Component)]
pub struct Rigibody {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
    pub size: Vec3,
    // gravity: f32,
    // friction: f32,
}
impl Default for Rigibody {
    fn default() -> Self {
        Self {
            velocity: Vec3::new(0., 0., 0.),
            acceleration: Vec3::new(0., 0., 0.),
            mass: 1.,
            size: Vec3::new(50., 50., 50.),
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

fn update_collider(time: Res<Time>, mut query: Query<(&mut Transform, &mut Rigibody)>) {
    let mut c = query.iter_combinations_mut();
    while let Some([mut a, mut b]) = c.fetch_next() {
        let delta = a.0.translation - b.0.translation;

        let x = (a.1.size.x / 2.0 + b.1.size.x / 2.0);
        let y = (a.1.size.y / 2.0 + b.1.size.y / 2.0);
        if delta.x.abs() < (a.1.size.x / 2.0 + b.1.size.x / 2.0)
            && delta.y.abs() < (a.1.size.y / 2.0 + b.1.size.y / 2.0)
        {
            if (a.1.size.x / 2.0 + b.1.size.x / 2.0) - delta.x.abs()
                < (a.1.size.y / 2.0 + b.1.size.y / 2.0) - delta.y.abs()
            {
                let past_v1 = a.1.velocity.x;
                let past_v2 = b.1.velocity.x;
                let total_mass = a.1.mass + b.1.mass;
                a.1.velocity.x =
                    ((a.1.mass - b.1.mass) * past_v1 + (2.0 * b.1.mass) * past_v2) / (total_mass);
                b.1.velocity.x =
                    ((2.0 * a.1.mass) * past_v1 + (b.1.mass - a.1.mass) * past_v2) / (total_mass);
            } else {
                let past_v1 = a.1.velocity.y;
                let past_v2 = b.1.velocity.y;
                let total_mass = a.1.mass + b.1.mass;
                a.1.velocity.y =
                    ((a.1.mass - b.1.mass) * past_v1 + (2.0 * b.1.mass) * past_v2) / (total_mass);
                b.1.velocity.y =
                    ((2.0 * a.1.mass) * past_v1 + (b.1.mass - a.1.mass) * past_v2) / (total_mass);
            }
        }
    }
}
