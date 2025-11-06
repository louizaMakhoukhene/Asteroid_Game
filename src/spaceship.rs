//! # Module `Spaceship`
//!
//! Ce module définit le vaisseau spatial contrôlé par le joueur. Le vaisseau peut se déplacer,
//! tirer des missiles et tourner. Il est également capable de passer d'un bord de l'écran à l'autre
//! en cas de sortie des limites.

use crate::stellarobject::{ObjectType, StellarObject};
use crate::Missile;
use macroquad::prelude::*;
use std::any::Any;

/// Structure représentant le vaisseau spatial.
///
/// Le vaisseau est défini par sa position, son rayon (taille physique), sa couleur, son angle de direction,
/// sa vitesse, et un état actif ou inactif.
#[derive(Clone)]
pub struct Spaceship {
    /// Position actuelle du vaisseau.
    pub pos: Vec2,

    /// Rayon du vaisseau (taille visible).
    pub radius: f32,

    /// Couleur du vaisseau.
    pub color: Color,

    /// Angle de direction du vaisseau (en degrés).
    pub angle: f32,

    /// Vitesse actuelle du vaisseau.
    pub speed: f32,

    /// Indique si le vaisseau est actif ou non.
    active: bool,
}

impl Spaceship {
    /// Crée un nouveau vaisseau spatial avec une position, un rayon, une couleur et une vitesse initiale donnés.
    ///
    /// # Paramètres
    /// - `pos`: Position initiale du vaisseau.
    /// - `radius`: Rayon du vaisseau.
    /// - `color`: Couleur du vaisseau.
    /// - `speed`: Vitesse initiale du vaisseau.
    ///
    /// # Retour
    /// Retourne une instance initialisée de `Spaceship`.
    pub fn new(pos: Vec2, radius: f32, color: Color, speed: f32) -> Self {
        Self {
            pos,
            radius,
            color,
            angle: 0.0,
            speed,
            active: true,
        }
    }
}

impl StellarObject for Spaceship {
    /// Rend le vaisseau à l'écran.
    ///
    /// Le vaisseau est représenté par un triangle coloré avec un canon rouge à son extrémité.
    fn render(&self) {
        draw_poly(
            self.pos.x,
            self.pos.y,
            3,
            self.radius,
            self.angle + 210.0,
            self.color,
        );
        draw_poly_lines(
            self.pos.x,
            self.pos.y,
            3,
            self.radius,
            self.angle + 210.0,
            1.5,
            WHITE,
        );

        let cannon_angle = self.angle + 210.0;
        let cannon_pos = self.pos + Vec2::from_angle(cannon_angle.to_radians()) * self.radius;
        draw_circle(cannon_pos.x, cannon_pos.y, 5.0, RED);
    }

    /// Met à jour l'état du vaisseau en fonction des entrées utilisateur et de sa vitesse.
    ///
    /// - Les touches gauche et droite tournent le vaisseau.
    /// - Les touches haut et bas ajustent la vitesse.
    /// - La barre d'espace permet de tirer un missile.
    /// - Le vaisseau revient de l'autre côté de l'écran en cas de sortie des limites.
    ///
    /// # Paramètres
    /// - `nouveaux_objets`: Liste mutable où les nouveaux objets (comme les missiles) sont ajoutés.
    fn update(&mut self, nouveaux_objets: &mut Vec<Box<dyn StellarObject>>) {
        const MIN_SPEED: f32 = -5.0;
        const MAX_SPEED: f32 = 5.0;

        // Rotation du vaisseau
        if is_key_down(KeyCode::Left) {
            self.angle -= 1.0;
            if self.angle < 0.0 {
                self.angle += 360.0;
            }
        }
        if is_key_down(KeyCode::Right) {
            self.angle += 1.0;
            if self.angle >= 360.0 {
                self.angle -= 360.0;
            }
        }

        // Modification de la vitesse
        if is_key_pressed(KeyCode::Up) {
            self.speed = (self.speed + 0.1).clamp(MIN_SPEED, MAX_SPEED);
        }
        if is_key_pressed(KeyCode::Down) {
            self.speed = (self.speed - 0.3).clamp(MIN_SPEED, MAX_SPEED);
        }

        // Déplacement du vaisseau
        let cannon_angle = self.angle + 210.0;
        let direction = Vec2::from_angle(cannon_angle.to_radians());
        self.pos += direction * self.speed;

        // Gestion des bordures d'écran
        if self.pos.x > screen_width() {
            self.pos.x = 0.0;
        } else if self.pos.x < 0.0 {
            self.pos.x = screen_width();
        }

        if self.pos.y > screen_height() {
            self.pos.y = 0.0;
        } else if self.pos.y < 0.0 {
            self.pos.y = screen_height();
        }

        // Tir d'un missile
        if is_key_pressed(KeyCode::Space) {
            let missile_pos = self.pos + direction * self.radius;
            nouveaux_objets.push(Box::new(Missile::new(missile_pos, cannon_angle, 10.0)));
        }
    }

    /// Retourne la position actuelle du vaisseau.
    fn get_position(&self) -> Vec2 {
        self.pos
    }

    /// Retourne le rayon du vaisseau.
    fn get_redius(&self) -> f32 {
        self.radius
    }

    /// Retourne le type de l'objet, ici `ObjectType::Spaceship`.
    fn get_type(&self) -> ObjectType {
        ObjectType::Spaceship
    }

    /// Vérifie si le vaisseau est actif.
    ///
    /// # Retour
    /// Retourne `true` si le vaisseau est actif, sinon `false`.
    fn is_active(&self) -> bool {
        self.active
    }

    /// Définit le vaisseau comme inactif.
    fn set_inactive(&mut self) {
        self.active = false;
    }

    /// Fournit une référence au type `Any` de l'objet (utilisé pour les conversions dynamiques).
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Fournit une référence mutable au type `Any` de l'objet (utilisé pour les conversions dynamiques).
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    //use macroquad::prelude::*;

    #[test]
    fn test_spaceship_creation() {
        // Crée un vaisseau 
        let spaceship = Spaceship::new(Vec2::new(100.0, 100.0), 20.0, BLUE, 2.0);

        assert_eq!(spaceship.get_position(), Vec2::new(100.0, 100.0));
        assert_eq!(spaceship.get_redius(), 20.0);
        assert_eq!(spaceship.is_active(), true);
    }


    
    #[test]
    fn test_spaceship_turning() {
        let mut spaceship = Spaceship::new(Vec2::new(100.0, 100.0), 20.0, BLUE, 2.0);
        
        // Test de la rotation (rotation de 1 degré vers la gauche et la droite)
        let initial_angle = spaceship.angle;
        spaceship.angle -= 1.0; // Tourne à gauche
        assert_eq!(spaceship.angle, initial_angle - 1.0);
        spaceship.angle += 2.0; // Tourne à droite
        assert_eq!(spaceship.angle, initial_angle + 1.0);
    }

    
}