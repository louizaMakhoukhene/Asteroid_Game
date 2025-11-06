use macroquad::prelude::*;
use std::any::Any;

pub trait StellarObject: Any {
    fn render(&self);
    fn update(&mut self, nouveaux_objets: &mut Vec<Box<dyn StellarObject>>);
    fn get_position(&self) -> Vec2;
    fn get_redius(&self) -> f32;
    fn get_type(&self) -> ObjectType;
    fn is_active(&self) -> bool;
    fn set_inactive(&mut self);

    fn check_collision(&self, other: &dyn StellarObject) -> bool {
        let distance = self.get_position().distance(other.get_position());
        distance < self.get_redius() + other.get_redius()
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(PartialEq, Debug)]
pub enum ObjectType {
    Asteroid,
    Missile,
    Spaceship,
}
