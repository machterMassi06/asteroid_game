use std::f32::consts::PI;

use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};
use crate::stellarobject::StellarObject;

/// Représente un astéroïde dans le jeu.
#[derive(Clone)]
pub struct Asteroid {
    position: Vec2,  // Position actuelle de l'astéroïde
    speed: Vec2,     // Vitesse de déplacement de l'astéroïde
    size: f32,       // Taille de l'astéroïde
    screen_size : (f32,f32), // Taille de la fenetre de jeu 
}

impl Asteroid {
    /// La taille initiale d'un astéroïde.
    pub const ASTEROID_INIT_SIZE: f32 = 50.0;
    /// La taille d'un astéroïde moyen.
    pub const ASTEROID_MEDIUM_SIZE: f32 = 25.0;
    /// La taille d'un astéroïde petit.
    pub const ASTEROID_SMALL_SIZE: f32 = 12.5;


    /// Crée une nouvelle instance d'Asteroid avec une taille spécifiée.
    /// 
    /// # Arguments
    /// 
    /// * `size` - La taille de l'astéroïde à créer.
    /// * `screen_size` - La taille de la fenetre de jeu.
    /// 
    /// # Returns
    /// 
    /// Une nouvelle instance d'Asteroid.
    pub fn new(size: f32,screen_size:(f32,f32)) -> Self {
        Self {
            position: Self::new_alea_pos(screen_size),
            speed: Self::new_alea_speed(),
            size, // Taille initiale de l'astéroïde
            screen_size,
        }
    }

    /// Génère une position aléatoire près d'un bord de l'écran.
    /// # Arguments
    /// 
    /// * `screen_size` - La taille de la fenetre de jeu.
    /// 
    /// # Returns
    /// 
    /// Un vecteur représentant la position aléatoire générée.
    fn new_alea_pos(screen_size:(f32,f32)) -> Vec2 {
        let mut rng = thread_rng();
        let nearpos: f32 = rng.gen_range(Self::ASTEROID_INIT_SIZE / 2.0..=Self::ASTEROID_INIT_SIZE);
        let nearside = rng.gen_range(1..=4); // 1 = haut, 2 = droite, 3 = bas, 4 = gauche
        
        let xpos: f32 = match nearside {
            2 => screen_size.0 - nearpos,
            4 => nearpos,                   
            _ => rng.gen_range(0.0..=screen_size.0),
        };

        let ypos: f32 = match nearside {
            1 => nearpos,                   
            3 => screen_size.1 - nearpos,
            _ => rng.gen_range(0.0..=screen_size.1),
        };

        vec2(xpos, ypos)
    }

    /// Génère et retourne un vecteur de vitesse aléatoire pour l'astéroïde.
    /// 
    /// # Returns
    /// 
    /// Un vecteur représentant la vitesse aléatoire générée.
    fn new_alea_speed() -> Vec2 {
        let mut rng = thread_rng();
        let angle: f32 = rng.gen_range(0.0..=(2.0 * PI)); 
        Vec2::from_angle(angle) 
    }

    /// Vérifie et ajuste la position de l'astéroïde pour s'assurer qu'elle reste dans les limites de l'écran.
    /// 
    /// # Arguments
    /// 
    /// * `pos` - La position à ajuster.
    /// * `screen_size` - La taille de la fenetre de jeu.
    /// # Returns
    /// 
    /// La position ajustée si elle sort des limites de l'écran.
    fn bound_pos(mut pos: Vec2,screen_size:(f32,f32)) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_size.0);
        pos.y = Self::bound_to(pos.y, screen_size.1);
        pos
    }

    /// Ajuste une coordonnée pour s'assurer qu'elle reste dans les limites spécifiées.
    /// 
    /// # Arguments
    /// 
    /// * `coord` - La coordonnée à ajuster.
    /// * `max` - La valeur maximale de la coordonnée.
    /// 
    /// # Returns
    /// 
    /// La coordonnée ajustée.
    fn bound_to(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max - coord 
        } else if coord > max {
            coord - max 
        } else {
            coord 
        }
    }
    /// Modif la position de l'asteroid 
    /// # Arguments
    /// 
    /// * `new_pos` - Un nouveau Vec2 representant la nouvelle position.
    /// 
    pub fn set_position(&mut self,new_pos:Vec2){
        self.position=new_pos;
    }
    /// Divise un astéroïde en deux nouveaux astéroïdes plus petits.
    /// 
    /// # Returns
    /// 
    /// Une option contenant les deux nouveaux astéroïdes ou `None` si l'astéroïde ne peut pas être divisé.
    pub fn split_asteroid(&self) -> Option<(Self, Self)> {
        let position=self.get_position();
        
        let new_size=match self.size {
            Self::ASTEROID_INIT_SIZE => {
                Some(Self::ASTEROID_MEDIUM_SIZE)
            },
            Self::ASTEROID_MEDIUM_SIZE => {
                Some(Self::ASTEROID_SMALL_SIZE)
            },
            _ => None, // Pas de division possible pour les petits astéroïdes
        };
        match new_size{
            Some(s)=>{
                let mut a1= Asteroid::new(s,self.screen_size);
                let mut a2 =Asteroid::new(s,self.screen_size);
                a1.set_position(position+vec2(50.0,50.0));
                a2.set_position(position-vec2(50.0,-50.0));
                Some((a1,a2))
            },
            None=> None , 
        }

    }
}

/// Implementation du trait StellarObject pour la structure Asteroid
/// 
impl StellarObject for Asteroid{
    /// Retourne la position actuelle de l'astéroïde.
    /// 
    /// # Returns
    /// 
    /// Un vecteur représentant la position de l'astéroïde.
    fn get_position(&self) -> Vec2 {
        self.position
    }
    /// Met à jour la position de l'astéroïde en fonction de sa vitesse.
    /// 
    fn update(&mut self) {
        self.position += self.speed; // Mise à jour de la position
        self.position = Self::bound_pos(self.position,self.screen_size); // Vérification des limites
        
    }
    
    /// Retourne la taille actuelle de l'astéroïde.
    /// 
    /// # Returns
    /// 
    /// La taille de l'astéroïde.
    fn get_size(&self) -> f32 {
        self.size
    }

}



