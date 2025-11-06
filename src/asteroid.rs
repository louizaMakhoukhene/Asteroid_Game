//! # Module `Asteroid`
//!
//! Ce module définit les astéroïdes utilisés dans le jeu. Les astéroïdes peuvent avoir trois tailles différentes :
//! grand, moyen et petit. Ils se déplacent de manière aléatoire à travers l'écran et peuvent être divisés en
//! fragments plus petits lorsqu'ils sont détruits.

use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::any::Any;
use std::f32::consts::PI;

use crate::stellarobject::{ObjectType, StellarObject};

/// Structure représentant un astéroïde dans le jeu.
///
/// Un astéroïde a une position, une vitesse, une taille, un rayon (représentant sa taille physique),
/// et un état actif ou inactif.
#[derive(Clone)]
pub struct Asteroid {
    position: Vec2,
    speed: Vec2,
    size: AsteroidSize,
    redius: f32,
    active: bool,
}

/// Enumération représentant la taille d'un astéroïde.
///
/// Les tailles possibles sont :
/// - `Large`: Grand.
/// - `Medium`: Moyen.
/// - `Small`: Petit.
#[derive(Debug, Copy, Clone)]
pub enum AsteroidSize {
    Large,
    Medium,
    Small,
}

impl AsteroidSize {
    /// Génère une taille d'astéroïde aléatoire.
    ///
    /// # Retour
    /// Une taille parmi les variantes de `AsteroidSize`.
    fn new_size_alea() -> AsteroidSize {
        let mut rng = thread_rng();
        match rng.gen_range(0..=2) {
            0 => AsteroidSize::Large,
            1 => AsteroidSize::Medium,
            _ => AsteroidSize::Small,
        }
    }
}

impl Asteroid {
    /// Constante pour la taille initiale des grands astéroïdes.
    const ASTEROID_INIT_SIZE: f32 = 60.0;

    /// Constante pour la taille des astéroïdes moyens.
    const ASTEROID_MEDIUM: f32 = 30.0;

    /// Constante pour la taille des petits astéroïdes.
    const ASTEROID_SMALL: f32 = 15.0;

    /// Crée un nouvel astéroïde avec une position, une vitesse et une taille aléatoires.
    ///
    /// # Retour
    /// Une instance d'`Asteroid` initialisée.
    pub fn new() -> Self {
        let size = AsteroidSize::new_size_alea();
        let redius = match size {
            AsteroidSize::Large => Self::ASTEROID_INIT_SIZE,
            AsteroidSize::Medium => Self::ASTEROID_MEDIUM,
            AsteroidSize::Small => Self::ASTEROID_SMALL,
        };

        Self {
            position: Self::new_alea_pos(),
            speed: Self::new_alea_speed(),
            size,
            redius,
            active: true,
        }
    }

    /// Génère une position aléatoire dans les limites de l'écran.
    ///
    /// # Retour
    /// Un vecteur 2D représentant la position aléatoire.
    fn new_alea_pos() -> Vec2 {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..screen_width());
        let y = rng.gen_range(0.0..screen_height());
        Vec2::new(x, y)
    }

    /// Génère une vitesse aléatoire pour l'astéroïde.
    ///
    /// # Retour
    /// Un vecteur 2D représentant la vitesse aléatoire.
    fn new_alea_speed() -> Vec2 {
        let mut rng = thread_rng();
        let angle = rng.gen_range(0.0..=2.0 * PI);
        let speed = rng.gen_range(1.0..3.0);
        Vec2::from_angle(angle) * speed
    }

    /// Divise un astéroïde en fragments plus petits lorsqu'il est détruit.
    ///
    /// # Retour
    /// Une option contenant un vecteur d'objets stellaires représentant les fragments,
    /// ou `None` si l'astéroïde est trop petit pour être divisé.
    pub fn split(&mut self) -> Option<Vec<Box<dyn StellarObject>>> {
        match self.size {
            AsteroidSize::Large => Some(vec![
                Box::new(Asteroid {
                    position: self.position + vec2(30.0, 30.0),
                    speed: Asteroid::new_alea_speed(),
                    size: AsteroidSize::Medium,
                    redius: Asteroid::ASTEROID_MEDIUM,
                    active: true,
                }),
                Box::new(Asteroid {
                    position: self.position + vec2(-30.0, -30.0),
                    speed: Asteroid::new_alea_speed(),
                    size: AsteroidSize::Medium,
                    redius: Asteroid::ASTEROID_MEDIUM,
                    active: true,
                }),
            ]),
            AsteroidSize::Medium => Some(vec![
                Box::new(Asteroid {
                    position: self.position + vec2(15.0, 15.0),
                    speed: Asteroid::new_alea_speed(),
                    size: AsteroidSize::Small,
                    redius: Asteroid::ASTEROID_SMALL,
                    active: true,
                }),
                Box::new(Asteroid {
                    position: self.position + vec2(-15.0, -15.0),
                    speed: Asteroid::new_alea_speed(),
                    size: AsteroidSize::Small,
                    redius: Asteroid::ASTEROID_SMALL,
                    active: true,
                }),
            ]),
            AsteroidSize::Small => None,
        }
    }
}

impl StellarObject for Asteroid {
    /// Rend l'astéroïde visible à l'écran en dessinant un cercle.
    fn render(&self) {
        if !self.active {
            return;
        }
        draw_circle(self.position.x, self.position.y, self.redius, ORANGE);
        draw_circle_lines(self.position.x, self.position.y, self.redius, 1.0, BLACK);
    }

    /// Met à jour la position de l'astéroïde en fonction de sa vitesse.
    ///
    /// # Paramètres
    /// - `_new_objects`: Non utilisé ici, mais nécessaire pour implémenter l'interface `StellarObject`.
    fn update(&mut self, _new_objects: &mut Vec<Box<dyn StellarObject>>) {
        if !self.active {
            return;
        }
        self.position += self.speed;

        if self.position.x < 0.0 {
            self.position.x = screen_width();
        } else if self.position.x > screen_width() {
            self.position.x = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = screen_height();
        } else if self.position.y > screen_height() {
            self.position.y = 0.0;
        }
    }

    /// Retourne la position actuelle de l'astéroïde.
    fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Retourne le rayon de l'astéroïde.
    fn get_redius(&self) -> f32 {
        self.redius
    }

    /// Retourne le type de l'objet, ici `ObjectType::Asteroid`.
    fn get_type(&self) -> ObjectType {
        ObjectType::Asteroid
    }

    /// Indique si l'astéroïde est actif.
    fn is_active(&self) -> bool {
        self.active
    }

    /// Rend l'astéroïde inactif.
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

    #[test]
    fn test_asteroid_split_large() {
        let mut asteroid = Asteroid {
            size: AsteroidSize::Large,
            redius: Asteroid::ASTEROID_INIT_SIZE,
            position: Vec2::new(100.0, 100.0),
            speed: Vec2::new(1.0, 1.0),
            active: true,
        };

        let fragments = asteroid.split();

        // Vérifie qu'un grand astéroïde peut être divisé en fragments
        assert!(fragments.is_some());
    }
    
    #[test]
    fn test_asteroid_split_small() {
        let mut asteroid = Asteroid {
            size: AsteroidSize::Small,
            redius: Asteroid::ASTEROID_SMALL,
            position: Vec2::new(100.0, 100.0),
            speed: Vec2::new(1.0, 1.0),
            active: true,
        };

        let fragments = asteroid.split();

        // Vérifie qu'un petit astéroïde ne peut pas être divisé en fragments
        assert!(fragments.is_none());
    }

   
}