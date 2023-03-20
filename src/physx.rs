use raylib::prelude::*;

#[derive(Debug, Default)]
pub struct PhysxObject {
    pub pos: Vector2,
    pub vel: Vector2,
    acc: Vector2,
    pub radius: f32,
    pub dampening_factor: f32,
}

impl PhysxObject {
    pub fn new(pos: Vector2, vel: Vector2, acc: Vector2, radius: f32, dampening_factor: f32) -> Self 
    { 
        Self { pos, vel, acc, radius, dampening_factor } 
    }

    pub fn set_pos(&mut self, pos: Vector2) {
        self.pos = pos
    }

    pub fn set_accel(&mut self, acc: Vector2) {
        self.acc = acc
    }

    pub fn set_vel(&mut self, vel: Vector2) {
        self.vel = vel
    }

    pub fn update_vel(&mut self, dt: f32) {
        self.vel.x += self.acc.x/dt;
        self.vel.y += self.acc.y/dt;
    }

    pub fn update_pos(&mut self, dt: f32) {
        self.pos.x += self.vel.x/dt;
        self.pos.y += self.vel.y/dt;
    }
}