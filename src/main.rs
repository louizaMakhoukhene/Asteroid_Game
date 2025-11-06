//! # Astéroïdes
//!
//! Ce projet est un jeu inspiré du classique "Asteroids". Le joueur pilote un vaisseau spatial
//! dans un environnement rempli d'astéroïdes. L'objectif est de détruire ces astéroïdes tout en
//! évitant les collisions, afin d'obtenir le meilleur score possible.
//!
//! ## Fonctionnalités principales
//!
//! - Contrôle fluide d'un vaisseau spatial avec gestion de l'orientation et de la vitesse.
//! - Astéroïdes destructibles, qui se divisent en fragments plus petits.
//! - Gestion des collisions avec effets visuels et audio.
//! - Système de score avec sauvegarde du meilleur score dans un fichier texte.
//! - Interface intuitive : affichage des scores, vies restantes et menu principal.
//!
//! ## Commandes du jeu
//!
//! - **Orientation du vaisseau** :  
//!   - **Flèche droite** : Tourner à droite.  
//!   - **Flèche gauche** : Tourner à gauche.
//! - **Contrôle de la vitesse** :  
//!   - **Flèche haut** : Augmenter la vitesse.  
//!   - **Flèche bas** : Reculer.
//! - **Tirer des missiles** : Appuyer sur la barre d’espace.
//! - **Pause** : Appuyer sur `P` pour mettre le jeu en pause ou reprendre.
//! - **Quitter le jeu** : Appuyer sur `Échap` pour quitter.
//!
//! ## Dépendances
//!
//! - **`macroquad`** : Gère les graphismes et l'audio.
//! - Modules internes :
//!   - `asteroid` : Gestion des astéroïdes et leurs comportements.
//!   - `missile` : Gestion des missiles tirés par le vaisseau.
//!   - `spaceship` : Gestion du vaisseau spatial, incluant orientation et vitesse.
//!   - `explosions` : Animation et gestion des explosions.
//!   - `stellarobject` : Interface générique pour les objets stellaires.
//!
//! ## Gestion des scores
//!
//! Le score est enregistré dans un fichier texte nommé [`Best_score.txt`]. Si ce fichier est absent,
//! il est créé automatiquement lors de l'exécution du jeu.

use asteroid::Asteroid;
use explosions::Explosion;
use macroquad::audio::load_sound;
use macroquad::prelude::*;
use missile::Missile;
use spaceship::Spaceship;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

mod stellarobject;
use crate::stellarobject::{ObjectType, StellarObject};

mod asteroid;
mod explosions;
mod missile;
mod spaceship;

//use macroquad::audio::{play_sound_once, Sound};

const SCORE_FILE: &str = "Best_score.txt";

/// Lit le meilleur score sauvegardé dans le fichier [`SCORE_FILE`].
///
/// # Retour
/// Retourne le meilleur score sous forme d'un entier non signé, ou 0 si le fichier est vide ou invalide.
fn read_best_score() -> io::Result<u32> {
    let mut file = OpenOptions::new().read(true).open(SCORE_FILE)?;
    let mut score_str = String::new();
    file.read_to_string(&mut score_str)?;
    Ok(score_str.trim().parse().unwrap_or(0))
}

/// Sauvegarde le meilleur score dans le fichier [`SCORE_FILE`].
///
/// # Paramètres
/// - `score`: Le score à sauvegarder.
///
/// # Retour
/// Retourne une [`io::Result`] pour indiquer si l'opération a réussi.
fn save_best_score(score: u32) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(SCORE_FILE)?;
    write!(file, "{}", score)?;
    Ok(())
}

/// Met à jour le meilleur score si le score actuel est supérieur au meilleur score précédent.
///
/// # Paramètres
/// - `score`: Le score actuel à comparer.
fn update_best_score(score: u32) -> io::Result<()> {
    let best_score = read_best_score().unwrap_or(0);
    if score > best_score {
        save_best_score(score)?;
    }
    Ok(())
}

/// Affiche le meilleur score sur l'écran.
///
/// # Paramètres
/// - `best_score`: Le meilleur score à afficher.
fn draw_best_score(best_score: u32) {
    draw_text(
        &format!("Meilleur score : {}", best_score),
        screen_width() - 280.0,
        70.0,
        30.0,
        WHITE,
    );
}

/// Configure la fenêtre du jeu.
///
/// # Retour
/// Retourne une configuration [`Conf`] pour la fenêtre.
fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroides".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

/// Dessine l'arrière-plan en utilisant une texture.
///
/// # Paramètres
/// - `texture`: La texture de l'arrière-plan.
fn draw_background(texture: &Texture2D) {
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );
}

/*async fn load_sounds() -> Sound {
    // Charge le son de fond.
    load_sound("assets/sons/background_music.wav")
        .await
        .unwrap()
}*/

/// Affiche le nombre de vies restantes.
///
/// # Paramètres
/// - `lives`: Le nombre de vies à afficher.
fn draw_lives(lives: u8) {
    draw_text(&format!("Vies : {}", lives), 20.0, 30.0, 30.0, WHITE);
}

/// Affiche le score actuel.
///
/// # Paramètres
/// - `score`: Le score à afficher.
fn draw_score(score: u32) {
    draw_text(
        &format!("Score : {}", score),
        screen_width() - 150.0,
        30.0,
        30.0,
        WHITE,
    );
}

/// Affiche un menu avec des options pour jouer ou quitter.
///
/// # Paramètres
/// - `background`: Texture de l'arrière-plan.
/// - `message`: Message affiché au centre du menu.
/// - `best_score`: Meilleur score à afficher.
async fn draw_menu(background: &Texture2D, message: &str, best_score: u32) {
    loop {
        draw_background(background);

        let title_text = message;
        let play_text = "JOUER (APPUYER SUR ENTRER)";
        let quit_text = "QUITTER (APPUYER SUR ESC)";
        let pause_text = "PAUSE (APPUYEZ SUR LA TOUCHE P";
        let best_score_text = &format!("MEILLEUR SCORE : {}", best_score);
        //let background_music = load_sounds().await;

        draw_text(
            title_text,
            (screen_width() - measure_text(title_text, None, 80, 1.0).width) / 2.0,
            screen_height() / 4.0,
            80.0,
            WHITE,
        );

        draw_text(
            play_text,
            (screen_width() - measure_text(play_text, None, 40, 1.0).width) / 2.0,
            screen_height() / 2.0,
            40.0,
            WHITE,
        );

        draw_text(
            quit_text,
            (screen_width() - measure_text(quit_text, None, 40, 1.0).width) / 2.0,
            screen_height() / 2.0 + 100.0,
            40.0,
            WHITE,
        );

        draw_text(
            best_score_text,
            (screen_width() - measure_text(best_score_text, None, 40, 1.0).width) / 2.0,
            screen_height() / 2.0 + 180.0,
            40.0,
            WHITE,
        );

        draw_text(
            pause_text,
            (screen_width() - measure_text(pause_text, None, 40, 1.0).width) / 2.0,
            screen_height() / 2.0 + 50.0,
            40.0,
            WHITE,
        );

        //play_sound_once(&background_music);

        if is_key_pressed(KeyCode::Enter) {
            break;
        }
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        next_frame().await;
    }
}

/// Met à jour les objets stellaires dans la scène.
///
/// # Paramètres
/// - `objects`: Une liste mutable d'objets stellaires à mettre à jour.
///
/// Cette fonction appelle la méthode `update` de chaque objet et
/// ajoute les nouveaux objets générés à la liste existante.
fn update_model(objects: &mut Vec<Box<dyn StellarObject>>) {
    let mut new_objects: Vec<Box<dyn StellarObject>> = vec![];
    for obj in objects.iter_mut() {
        obj.update(&mut new_objects);
    }
    objects.extend(new_objects);
}

/// Gère l'entrée de l'utilisateur.
///
/// # Retour
/// Retourne `true` si l'utilisateur a appuyé sur la touche d'échappement pour quitter le jeu.
fn handle_input() -> bool {
    is_key_down(KeyCode::Escape)
}

/// Nombre initial d'astéroïdes dans le jeu.
const INITIAL_ASTEROID_COUNT: usize = 10;

/// Affiche le nombre d'astéroïdes restants.
///
/// # Paramètres
/// - `count`: Le nombre d'astéroïdes restants à afficher.
fn draw_asteroids_left(count: usize) {
    draw_text(
        &format!("Astéroïdes restants : {}", count),
        screen_width() - 350.0,
        100.0,
        30.0,
        WHITE,
    );
}

/// Point d'entrée principal du jeu.
///
/// Cette fonction configure la fenêtre, charge les ressources, initialise les objets du jeu
/// et gère la boucle principale du jeu.
#[macroquad::main(window_conf)]
async fn main() {
    // Charge les textures et sons nécessaires.
    let background_image = load_texture("assets/images/espace.png").await.unwrap();
    let explosion_texture = load_texture("assets/images/explosion.png").await.unwrap();
    let explosion_sound = load_sound("assets/sons/son.wav").await.unwrap();

    //let background_music = load_sounds().await;

    // Lit le meilleur score depuis le fichier.
    let mut best_score = read_best_score().unwrap_or(0);

    // Affiche le menu principal.
    draw_menu(&background_image, "ASTEROIDES", best_score).await;

    // Initialise le vaisseau spatial.
    let spaceship = Spaceship::new(
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        20.0,
        BLUE,
        0.5,
    );

    // Crée une liste d'objets stellaires contenant le vaisseau et des astéroïdes.
    let mut objects: Vec<Box<dyn StellarObject>> = vec![Box::new(spaceship.clone())];
    objects.extend(
        (0..INITIAL_ASTEROID_COUNT).map(|_| Box::new(Asteroid::new()) as Box<dyn StellarObject>),
    );

    let mut explosions: Vec<Explosion> = Vec::new();
    let mut lives = 3;
    let mut score = 0;
    let mut pause = false;

    // Boucle principale du jeu.
    loop {
        // Gère la pause.
        if is_key_pressed(KeyCode::P) {
            pause = !pause;
        }

        if pause {
            clear_background(PURPLE);
            draw_text(
                "PAUSE",
                screen_width() / 2.0 - 60.0,
                screen_height() / 2.0,
                40.0,
                WHITE,
            );
            next_frame().await;
            continue;
        }

        // Quitte le jeu si l'utilisateur appuie sur Échap.
        if handle_input() {
            update_best_score(score).unwrap(); // Sauvegarde du score avant de quitter.
            break;
        }

        // Dessine l'arrière-plan.
        draw_background(&background_image);

        // Met à jour les objets stellaires.
        update_model(&mut objects);

        // Rend chaque objet sur l'écran.
        for obj in &objects {
            obj.render();
        }

        // Met à jour et dessine les explosions.
        for explosion in explosions.iter_mut() {
            explosion.update(1.0);
            explosion.draw_explosion();
        }

        // Retient uniquement les explosions encore actives.
        explosions.retain(|e| !e.exprire());

        // Affiche les vies, le score, le meilleur score et le nombre d'astéroïdes restants.
        draw_lives(lives);
        draw_score(score);
        draw_best_score(best_score);
        let asteroid_count = objects
            .iter()
            .filter(|o| o.get_type() == ObjectType::Asteroid)
            .count();
        draw_asteroids_left(asteroid_count);

        // Vérifie si tous les astéroïdes ont été détruits.
        if asteroid_count == 0 {
            // Met à jour le meilleur score.
            update_best_score(score).unwrap();
            best_score = read_best_score().unwrap_or(0);

            // Affiche un message de victoire.
            draw_menu(&background_image, "VICTOIRE !", best_score).await;

            // Réinitialise ou quitte le jeu.
            score = 0;
            lives = 3;
            objects.clear();
            objects.push(Box::new(Spaceship::new(
                Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
                20.0,
                BLUE,
                0.5,
            )));
            objects.extend(
                (0..INITIAL_ASTEROID_COUNT)
                    .map(|_| Box::new(Asteroid::new()) as Box<dyn StellarObject>),
            );

            continue; // Redémarre la boucle de jeu après une victoire.
        }

        // Gère les collisions et les mises à jour des objets.
        let mut new_objects: Vec<Box<dyn StellarObject>> = Vec::new();
        for i in 0..objects.len() {
            for j in (i + 1)..objects.len() {
                if objects[i].check_collision(objects[j].as_ref()) {
                    match (objects[i].get_type(), objects[j].get_type()) {
                        (ObjectType::Spaceship, ObjectType::Asteroid) => {
                            lives -= 1;

                            if let Some(spaceship) =
                                objects[i].as_any_mut().downcast_mut::<Spaceship>()
                            {
                                spaceship.pos =
                                    Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
                            }

                            objects[j].set_inactive();

                            if lives == 0 {
                                // Gère la fin de partie.
                                update_best_score(score).unwrap();
                                best_score = read_best_score().unwrap_or(0);
                                draw_menu(&background_image, "GAME OVER", best_score).await;

                                // Réinitialise le jeu.
                                lives = 3;
                                score = 0;
                                objects.clear();
                                objects.push(Box::new(Spaceship::new(
                                    Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
                                    20.0,
                                    BLUE,
                                    0.5,
                                )));
                                objects.extend(
                                    (0..INITIAL_ASTEROID_COUNT).map(|_| {
                                        Box::new(Asteroid::new()) as Box<dyn StellarObject>
                                    }),
                                );
                            }
                        }
                        (ObjectType::Asteroid, ObjectType::Missile) => {
                            // Gère les collisions entre astéroïdes et missiles.
                            let fragments = if let Some(asteroid) =
                                objects[i].as_any_mut().downcast_mut::<Asteroid>()
                            {
                                asteroid.split()
                            } else {
                                None
                            };

                            if let Some(pos) = objects[i]
                                .as_any()
                                .downcast_ref::<Asteroid>()
                                .map(|a| a.get_position())
                            {
                                explosions.push(Explosion::new(
                                    pos,
                                    explosion_texture.clone(),
                                    explosion_sound.clone(),
                                ));
                            }

                            objects[i].set_inactive();
                            objects[j].set_inactive();

                            if let Some(fragments) = fragments {
                                new_objects.extend(fragments);
                            }

                            score += 1;
                        }
                        (ObjectType::Asteroid, ObjectType::Asteroid) => {
                            // Gère les collisions entre astéroïdes.
                            let mut fragments = vec![];

                            if let Some(asteroid) =
                                objects[i].as_any_mut().downcast_mut::<Asteroid>()
                            {
                                if let Some(new_fragments) = asteroid.split() {
                                    fragments.extend(new_fragments);
                                }
                                objects[i].set_inactive();
                            }

                            if let Some(asteroid) =
                                objects[j].as_any_mut().downcast_mut::<Asteroid>()
                            {
                                if let Some(new_fragments) = asteroid.split() {
                                    fragments.extend(new_fragments);
                                }
                                objects[j].set_inactive();
                            }

                            new_objects.extend(fragments);
                        }
                        _ => {}
                    }
                }
            }
        }

        // Ajoute les nouveaux objets générés et enlève les objets inactifs.
        objects.extend(new_objects);
        objects.retain(|obj| obj.is_active());

        // Passe au prochain cadre.
        next_frame().await;
    }
}
