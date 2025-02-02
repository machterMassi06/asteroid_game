/// Import des modules nécessaires.
use asteroid::Asteroid;
use miniquad::window::screen_size;
use spaceship::Spaceship;
use missile::Missile;
use crate::stellarobject::StellarObject;
use macroquad::prelude::*; 
use macroquad::audio::{play_sound, load_sound, Sound, PlaySoundParams};
use ::rand::{thread_rng, Rng};
use std::time::{Instant,Duration};
mod asteroid; 
mod spaceship;
mod missile ; 
mod stellarobject;

/// Dessin à l'écran .
/// # Arguments
///
/// * `asteroids` - Une référence à un vecteur des astreroids à dessiner.
/// * `spaceship` - Une référence au visseau à dessiner.
/// * `missiles` - Une référence à un vecteur de missiles à dessiner.
/// * `background_texture` - Une référence à Texture2D qui represent l'image de fond d'écran.
/// * `asteroid_texture` - Une référence à Texture2D qui represent l'image des asteroid.
///  
async fn draw(asteroids: &Vec<Asteroid>,spaceship:&Spaceship,missiles:&Vec<Missile>,background_texture:&Texture2D,asteroid_texture:&Texture2D) {
    draw_background(background_texture); // Dessine l'arrière-plan.
    draw_asteroids(asteroids,asteroid_texture); // Dessine les astéroïdes.
    draw_spaceship(spaceship,WHITE); // Dessine le vaisseau.
    draw_shield_level(spaceship);
    draw_missiles(missiles);
}

///Remplit l'arrière-plan en affichant l'image Fond d'écran .
/// 
/// # Arguments
///
/// * `texture` - Une référence à Texture2D.
fn draw_background(texture:&Texture2D) {
    let (screen_width, screen_height) = (screen_width(), screen_height());
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width, screen_height)),
            ..Default::default()
        },
    );
}

/// Affiche le game over et Restart 
/// 
/// 
/// # Arguments
///
/// * `background_texture` - Une référence à Texture2D.
/// * `win` - Un boolean indiquant le cas d'une victoire.
/// 
async fn game_state(background_texture:&Texture2D,win:bool){

    draw_background(background_texture);
    // Affichage de l'écran de l'etat de jeu et Restart 
    match win {
        false=> {
            draw_text(
                "GAME OVER",
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0 - 50.0,
                70.0,
                WHITE,);
        } ,
        true => {
            draw_text(
            "YOU WIN ",
            screen_width() / 2.0 - 100.0,
            screen_height() / 2.0 - 50.0,
            70.0,
            WHITE,);
        },
    }
   
    draw_text(
        " Press 'R' to Restart ",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0,
        30.0,
        WHITE,);

}
/// Affiche le niveau de visseau .
/// 
/// # Arguments
///
/// * `spaceship` - Une référence à Spaceship (le Visseau).
/// 
fn draw_shield_level(spaceship:&Spaceship){
    let text = format!("Sheild_level : {}", spaceship.get_shield());
    draw_text(
        &text,                  
        10.0,  
        20.0,        
        30.0,                       
        WHITE,                         
    );
}
/// Affiche le temps ecoulé dans le jeu 
/// 
/// # Arguments
///
/// * `t` - time::Duration represente le temps écoulé.
/// 
fn show_elapsed_time(t:Duration){
    // Calculer les minutes et secondes
    let minutes = t.as_secs() / 60;
    let seconds = t.as_secs() % 60;
    // Formater en mm:ss
    let text = format!("Time: {:02}:{:02}", minutes, seconds);
    
    draw_text(
        &text,                  
        10.0,  
        40.0,        
        30.0,                       
        WHITE,                         
    );
}
/// Dessine chaque messile tiré sur l'écran.
///
/// # Arguments
///
/// * `missiles` - Une référence à un vecteur de missiles à dessiner.
/// 
fn draw_missiles(missiles:&Vec<Missile>){
    for missile in missiles{
        if missile.is_active(){
            draw_circle(missile.get_position().x, missile.get_position().y, 4.0, WHITE);
        }
    }
}
/// Dessine chaque astéroïde sur l'écran.
///
/// # Arguments
///
/// * `asteroids` - Une référence à un vecteur d'astéroïdes à dessiner.
/// * `asteroids_texture` - Une référence à Texture2D.
/// 
fn draw_asteroids(asteroids: &Vec<Asteroid>,asteroid_texture:&Texture2D) {
    for asteroid in asteroids {
        let size = asteroid.get_size();

        // Dessiner l'image de l'astéroïde en redimensionnant selon sa taille
        draw_texture_ex(
            asteroid_texture,                       
            asteroid.get_position().x - size*1.8,                 
            asteroid.get_position().y - size*2.0,                 
            WHITE,                                   
            DrawTextureParams {
                dest_size: Some(vec2(size*4.0, size*4.0)),  // Redimensionner l'image en fonction de la taille de l'astéroïde
                ..Default::default()
            },
        );
        
    }
}
/// Dessine Le visseau sur l'écran.
///
/// # Arguments
///
/// * `spaceship` - Une référence à Spaceship.
/// * `c` - Une couleur de type 'Color' qui définit la couleur du vaisseau.
/// 
fn draw_spaceship(spaceship : &Spaceship,c:Color) {
    let rotation_angle = spaceship.get_orientation();
    let shape_points = [
        vec2(0.0, -15.0),
        vec2(10.0, 15.0),
        vec2(-10.0, 15.0),
    ];
    let rotated_points:Vec<Vec2> = shape_points.iter()
    .map(|&point| Vec2::from_angle(point.to_angle()+rotation_angle)*point.length()).collect();
    draw_triangle_lines(
        rotated_points[0]+spaceship.get_position(),
        rotated_points[1]+spaceship.get_position(),
        rotated_points[2]+spaceship.get_position(),
        3.0,
        c,);
}

/// Gère l'entrée de l'utilisateur.
/// # Arguments
///
/// * `spaceship` - Une référence au visseau.
/// * `missiles` - Une référence mutable à un vecteur de missiles tiré.
/// * `missile_sound` - Une référence à Sound , represente le son du tire.
/// 
/// # Returns
///
/// * `bool` - Retourne vrai si la touche Échap (Escape) est enfoncée.
fn handle_input(spaceship:&mut Spaceship,missiles:&mut Vec<Missile>,missile_sound: &Sound) -> bool {
    if is_key_down(KeyCode::Escape) {
        return true; 
    }
    // Gestion de la poussée (touche flèche haut)
    if is_key_down(KeyCode::Up){
        spaceship.activate_thrust();
        // Changer la couleur du vaisseau afin de montrer l'effet de la poussée
        draw_spaceship(spaceship, ORANGE);
    }
    // Gestion de la rétro-poussée (touche flèche bas)
    if is_key_down(KeyCode::Down){
        spaceship.back_thrust();
    }
    // Gestion de la rotation à gauche (touche flèche gauche)
    if is_key_down(KeyCode::Left){
        spaceship.left_rotation();
    }
    // Gestion de la rotation à droite (touche flèche droite)
    if is_key_down(KeyCode::Right){
        spaceship.right_rotation();
    }
    // Gestion des tirs des missiles(touche Espace)
    // son 
    if is_key_pressed(KeyCode::Space){
        missiles.push(
            Missile::new(
                spaceship.get_position(),
                spaceship.get_orientation(),
                screen_size(),
            )
        );
        play_sounds(missile_sound);
    }
    false
}

/// Lance le son des differents évenements .
///
/// # Arguments
///
/// * `s` - Une référence à Sound , le fichier audio .
fn play_sounds (s:&Sound) {
    play_sound(
        s,
        PlaySoundParams {
            looped: false,
            volume: 1.0,
        },
    );
}
/// Met à jour de tous les composants du modele de jeu.
///
/// # Arguments
///
/// * `asteroids` - Une référence mutable à un vecteur d'astéroïdes.
/// * `spaceship` - Une référence mutable au visseau . 
/// * `missiles` - Une référence mutable à un vecteur de missiles tiré.
/// * `collision_missile_sound` - Une référence a Sound.
/// * `collision_ship_sound` - Une référence a Sound.
fn update_model(asteroids: &mut Vec<Asteroid>,spaceship:&mut Spaceship,missiles:&mut Vec<Missile>,
    collision_missile_sound: &Sound,collision_ship_sound: &Sound,) {
    let mut new_asteroids = Vec::new();
    for asteroid in asteroids.iter_mut() {
        asteroid.update(); // Déplace chaque astéroïde.
        // Vérifier la collision entre le vaisseau et l'astéroïde
        let mut collision_ship = false;
        if spaceship.check_collision(asteroid) {
            collision_ship = true;
            play_sounds(collision_ship_sound);// son lié a la colission entre visseau et un asteroid
            if let Some((a1, a2)) = asteroid.split_asteroid() {

                // Diviser l asteroid en deux si il y a une collision entre le vaisseau et l asteroid
                new_asteroids.push(a1);
                new_asteroids.push(a2);
            }
            // Si le vaisseau est détruit suite à la collision, on quitte la boucle
            if spaceship.is_destroyed() {
                break;
            }
        }
        // Parcourir les missiles pour vérifier les collisions
        let mut missile_hit = false;
        for missile in missiles.iter_mut() {
            missile.update(); // Mettre à jour la position du missile
        
            // Vérifier la collision entre le missile et l'astéroïde
            if missile.check_collision(asteroid) {
                missile_hit = true; // Indiquer que le missile a touché l'astéroïde
                //et mettre du son 
                play_sounds(collision_missile_sound);
                break;
            }
        }
        
        // Si un missile a touché l'astéroïde, le diviser et le supprimer
        if missile_hit {
            if let Some((a1, a2)) = asteroid.split_asteroid() {
                // Ajouter les nouveaux astéroïdes s'ils sont divisés
                new_asteroids.push(a1);
                new_asteroids.push(a2);
            }
        } else if !collision_ship {
            // Si l'astéroïde n'a pas été touché et pas de collision avec le vaisseau, le conserver
            new_asteroids.push(asteroid.clone());
        }
    }
    // Mettre à jour la position du missile
    for missile in missiles.iter_mut() {
        missile.update(); 
    }
    missiles.retain(|m| m.is_active());

    // Mettre à jour la liste des astéroïdes  et le visseau 
    *asteroids = new_asteroids;
    spaceship.update();
    
}

/// Initialise les objets de jeu (Modele) 
/// # Arguments
///
/// * `spaceship` - Une référence mutable Spaceship (au visseau) .
/// * `asteroids` - Une référence mutable à un vecteur d'astéroïdes.
/// * `missiles` - Une référence mutable à un vecteur de missiles.
/// 
fn init_game(
    spaceship: &mut Spaceship,
    asteroids: &mut Vec<Asteroid>,
    missiles: &mut Vec<Missile>,
) {
    *spaceship = Spaceship::new(screen_size()); // Réinitialise le vaisseau
    asteroids.clear(); // Supprime tous les astéroïdes
    // creation d'un vecteur de missiles initialement vide 
    *missiles=Vec::new(); 
    // creation de n(alea) astroieds
    let n_asteroid = thread_rng().gen_range(4..=8); 
    let mut vec_asteroids = Vec::new();
    for _ in 0..n_asteroid {
        vec_asteroids.push(asteroid::Asteroid::new(Asteroid::ASTEROID_INIT_SIZE,screen_size()));
    } 
    *asteroids = vec_asteroids; 
}


/// Fonction principale du programme.
///
/// Gère la boucle de jeu.
/// 
#[macroquad::main("ASTEROID GAME")] // Annotation pour exécuter la fonction principale avec macroquad.
async fn main() {
    // Charger les fichiers audio 
    let missile_sound = load_sound("assets/sounds/missile.wav").await.unwrap();
    let collision_missile_sound = load_sound("assets/sounds/collision_missile_asteroid.wav").await.unwrap();
    let collision_ship_sound = load_sound("assets/sounds/collision_ship_asteroid.wav").await.unwrap();
    let game_over_sound = load_sound("assets/sounds/game_over.wav").await.unwrap();

    // init des boolean d'etat de jeu a faux 
    let mut is_game_over = false;
    let mut win =false;
    // Charger les Images
    let background_texture = load_texture("assets/images/background_space.png").await.unwrap();
    let asteroid_texture = load_texture("assets/images/asteroid.png").await.unwrap();
    
    // Création d'un vaisseau
    let mut spaceship = Spaceship::new(screen_size());
    // creation d'un vecteur de missiles initialement vide
    let mut vec_asteroids = Vec::new();
    let mut missiles=Vec::new();

    init_game(&mut spaceship, &mut vec_asteroids, &mut missiles); 
    let mut start_time = Instant::now(); // temps de debut 
    // Boucle principale du jeu.
    loop {
        // gerer le restart apres une defaite ou une victoire
        if is_game_over || win {
            game_state(&background_texture,win).await;
            if is_key_pressed(KeyCode::R) {
                init_game(&mut spaceship, &mut vec_asteroids, &mut missiles);
                is_game_over = false;
                win=false;
                start_time=Instant::now() ;
            }
        }
        else {
            draw(&vec_asteroids,&spaceship,&missiles,&background_texture,&asteroid_texture).await;

            show_elapsed_time(Instant::now()-start_time);

            if handle_input(&mut spaceship,&mut missiles,&missile_sound) { break; } 
    
            update_model(&mut vec_asteroids,&mut spaceship,&mut missiles,&collision_missile_sound,
                &collision_ship_sound,);
    
            if spaceship.is_destroyed() {
                is_game_over = true;
                play_sounds(&game_over_sound);
            }
            if vec_asteroids.is_empty() && !spaceship.is_destroyed() {
                win=true;
            }
        }

        next_frame().await;
    }
}
