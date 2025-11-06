//! # Module `Explosion`
//!
//! Ce module gère les explosions dans le jeu. Une explosion est représentée par une texture animée,
//! une durée de vie limitée, et un son joué une seule fois lors de son apparition.

use macroquad::audio::{play_sound_once, Sound};
use macroquad::prelude::*;

/// Structure représentant une explosion dans le jeu.
///
/// Une explosion est définie par sa position, sa durée de vie, une texture, sa taille,
/// un son associé, et un indicateur permettant de jouer le son une seule fois.
#[derive(Debug)]
pub struct Explosion {
    /// Position de l'explosion à l'écran.
    pos: Vec2,

    /// Durée de vie restante de l'explosion.
    lifetime: f32,

    /// Texture utilisée pour afficher l'explosion.
    texture: Texture2D,

    /// Taille de l'explosion (diamètre).
    taille: f32,

    /// Son associé à l'explosion.
    son: Sound,

    /// Indicateur pour savoir si le son a déjà été joué.
    play_sound: bool,
}

impl Explosion {
    /// Crée une nouvelle explosion avec une position, une texture et un son donnés.
    ///
    /// # Paramètres
    /// - `pos`: Position de l'explosion à l'écran.
    /// - `texture`: Texture utilisée pour afficher l'explosion.
    /// - `son`: Son associé à l'explosion.
    ///
    /// # Retour
    /// Une instance d'`Explosion` initialisée.
    pub fn new(pos: Vec2, texture: Texture2D, son: Sound) -> Self {
        Self {
            pos,
            lifetime: 15.0,
            texture,
            taille: 80.0,
            son,
            play_sound: false,
        }
    }

    /// Dessine l'explosion à l'écran en utilisant la texture et la position spécifiées.
    ///
    /// Si la durée de vie de l'explosion est écoulée, elle ne sera pas rendue.
    pub fn draw_explosion(&self) {
        if self.lifetime > 0.0 {
            draw_texture_ex(
                &self.texture,
                self.pos.x,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.taille, self.taille)),
                    ..Default::default()
                },
            );
        }
    }

    /// Met à jour l'état de l'explosion, en réduisant sa durée de vie et en jouant le son si nécessaire.
    ///
    /// # Paramètres
    /// - `delta`: Temps écoulé depuis la dernière mise à jour (en secondes).
    pub fn update(&mut self, delta: f32) {
        self.lifetime -= delta * 0.2;

        if !self.play_sound {
            play_sound_once(&self.son);
            self.play_sound = true;
        }
    }

    /// Vérifie si l'explosion a expiré (c'est-à-dire si sa durée de vie est écoulée).
    ///
    /// # Retour
    /// Retourne `true` si la durée de vie est inférieure ou égale à 0, sinon `false`.
    pub fn exprire(&self) -> bool {
        self.lifetime <= 0.0
    }
}



