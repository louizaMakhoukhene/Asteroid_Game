//! # Module `Missile`
//!
//! Ce module gère les missiles dans le jeu. Les missiles sont des projectiles tirés par le joueur,
//! qui se déplacent dans la direction définie par leur angle et leur vitesse. Lorsqu'ils sortent de l'écran,
//! ils deviennent inactifs.

use macroquad::prelude::*;

use crate::stellarobject::ObjectType;
use crate::stellarobject::StellarObject;

use std::any::Any;

/// Structure représentant un missile dans le jeu.
///
/// Un missile est défini par sa position, sa vitesse, son angle de déplacement,
/// son rayon (taille physique), et son état actif ou inactif.
pub struct Missile {
    /// Position actuelle du missile.
    pub pos: Vec2,

    /// Vitesse du missile.
    pub speed: f32,

    /// Angle de déplacement du missile (en degrés).
    pub angle: f32,

    /// Rayon du missile (taille visible).
    pub radius: f32,

    /// Indique si le missile est actif ou non.
    active: bool,
}

impl Missile {
    /// Crée un nouveau missile avec une position, un angle et une vitesse donnés.
    ///
    /// # Paramètres
    /// - `pos`: Position initiale du missile.
    /// - `angle`: Angle de déplacement du missile (en degrés).
    /// - `speed`: Vitesse du missile.
    ///
    /// # Retour
    /// Retourne une instance initialisée de `Missile`.
    pub fn new(pos: Vec2, angle: f32, speed: f32) -> Self {
        Self {
            pos,
            speed,
            angle,
            radius: 5.0,
            active: true,
        }
    }
}

impl StellarObject for Missile {
    /// Retourne la position actuelle du missile.
    fn get_position(&self) -> Vec2 {
        self.pos
    }

    /// Dessine le missile à l'écran.
    ///
    /// Le missile est représenté par un cercle jaune. Si le missile est inactif,
    /// il n'est pas dessiné.
    fn render(&self) {
        if !self.active {
            return;
        }
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }

    /// Met à jour la position du missile en fonction de sa vitesse et de son angle.
    ///
    /// Si le missile sort des limites de l'écran, il devient inactif.
    ///
    /// # Paramètres
    /// - `_objet_de_lespace`: Non utilisé ici, mais nécessaire pour implémenter `StellarObject`.
    fn update(&mut self, _objet_de_lespace: &mut Vec<Box<dyn StellarObject>>) {
        if !self.active {
            return;
        }

        let direction = Vec2::from_angle(self.angle.to_radians());
        self.pos += direction * self.speed;

        // Définir comme inactif si le missile sort de l'écran.
        if self.pos.x < 0.0
            || self.pos.x > screen_width()
            || self.pos.y < 0.0
            || self.pos.y > screen_height()
        {
            self.active = false;
        }
    }

    /// Retourne le rayon du missile.
    fn get_redius(&self) -> f32 {
        self.radius
    }

    /// Retourne le type de l'objet, ici `ObjectType::Missile`.
    fn get_type(&self) -> ObjectType {
        ObjectType::Missile
    }

    /// Vérifie si le missile est actif.
    ///
    /// # Retour
    /// Retourne `true` si le missile est actif, sinon `false`.
    fn is_active(&self) -> bool {
        self.active
    }

    /// Définit le missile comme inactif.
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

    // Fonction de création d'un missile avec des paramètres de base
    fn create_missile() -> Missile {
        Missile::new(Vec2::new(100.0, 100.0), 45.0, 5.0)
    }

    #[test]
    fn test_missile_creation() {
        
        let missile = create_missile();
        assert_eq!(missile.pos, Vec2::new(100.0, 100.0));
        assert_eq!(missile.angle, 45.0);
        assert_eq!(missile.speed, 5.0);
        assert!(missile.is_active());
        assert_eq!(missile.get_redius(), 5.0);
    }


    #[test]
    fn test_missile_update() {

        let mut missile = create_missile();
       
        let initial_pos = missile.pos;
        missile.update(&mut Vec::new());
        assert_ne!(missile.pos, initial_pos);
        let direction = Vec2::from_angle(missile.angle.to_radians());
        let expected_pos = initial_pos + direction * missile.speed;
        assert_eq!(missile.pos, expected_pos);
    }


}