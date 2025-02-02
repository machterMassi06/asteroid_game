use macroquad::prelude::*;
use crate::stellarobject::StellarObject;


/// Représente un missile tiré par le vaisseau dans le jeu.
pub struct Missile {
    position: Vec2,  // Position actuelle du missile.
    speed: Vec2,     // Direction et vitesse du missile.
    is_active: bool,// Indique si le missile est actif ou non.
    screen_size:(f32,f32) ,// Taille de la fenetre (width,height)
}

impl Missile {
    /// Vitesse constante des missiles en unités par seconde.
    pub const SPEED: f32 = 80.0;

    /// Crée un nouveau missile avec la position initiale et l'orientation donnée.
    ///
    /// # Arguments
    /// * `position` - Position de départ du missile.
    /// * `orientation` - Angle en radians qui détermine la direction du missile.
    /// * `screen_size` - taille de la fenetre de jeu.
    pub fn new(position: Vec2, orientation: f32,screen_size:(f32,f32)) -> Self {
        // Calcule la vitesse en fonction de l'angle d'orientation.
        let speed = vec2(
            orientation.sin() * Self::SPEED,
            -orientation.cos() * Self::SPEED,
            
        );
        Self {
            position,
            speed,
            is_active: true,
            screen_size,
        }
    }

    /// Vérifie si le missile est toujours actif (c'est-à-dire à l'écran).
    ///
    /// # Returns
    /// * `true` si le missile est actif, `false` sinon.
    pub fn is_active(&self) -> bool {
        self.is_active
    }

}

/// Implémentation du trait StellarObject pour la struct Missile
impl StellarObject for Missile {
    /// Renvoie la position actuelle du missile.
    ///
    /// # Returns
    /// * La position sous forme de `Vec2`.
    fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Met à jour la position du missile en tenant compte de sa vitesse.
    ///
    /// La méthode désactive le missile s'il sort des limites de l'écran.
    fn update(&mut self) {
        if self.is_active {
            // Mise à jour de la position .
            
            self.position += self.speed * 0.04;

            // Désactivation si le missile quitte l'écran.
            if self.position.x < 0.0
                || self.position.x > self.screen_size.0
                || self.position.y < 0.0
                || self.position.y > self.screen_size.1
            {
                self.is_active = false;
            }
        }
    }

    /// Vérifie si le missile a heurté un astéroïde.
    ///
    /// # Arguments
    /// * `_other` - Une réference a un dyn StelarObject.
    ///
    /// # Returns
    /// * `true` si une collision est détectée, `false` sinon.
    fn check_collision(&mut self, _other:&dyn StellarObject) -> bool {
        if self.is_active {
            let distance = self.position.distance(_other.get_position());
            if distance < _other.get_size(){
                self.is_active=false;
                return true; 
            }else{
                return false;
            }
        }
        false
    }
}

/// Testes unitaires 
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_missile_update() {
        let position = vec2(100.0, 100.0);
        let orientation = 0.0; 
        let screen_size = (800.0, 600.0);
        let mut missile = Missile::new(position, orientation, screen_size);

        // Mise à jour de la position
        missile.update();
        
        // Vérifie que la position a changé
        assert_ne!(missile.get_position(), position);
    }
    #[test]
    fn test_missile_out_of_bounds() {
        let position = vec2(100.0, 100.0);
        let orientation = 0.0;
        let screen_size = (800.0, 600.0);
        let mut missile = Missile::new(position, orientation, screen_size);

        // Simule plusieurs mises à jour(position) pour faire sortir le missile de l'écran
        for _ in 0..100 {
            missile.update();
        }

        // Le missile devrait être désactivé
        assert_eq!(missile.is_active(), false);
    }
     
     #[test]
    fn test_missile_collision() {
         let position = vec2(100.0, 100.0);
         let orientation = 0.0; 
         let screen_size = (800.0, 600.0);
         let mut missile = Missile::new(position, orientation, screen_size);
 
         let mut asteroid=crate::Asteroid::new(10.0, screen_size);
         asteroid.set_position(vec2(105.0, 105.0));

 
         // Vérifie que la collision est détectée
         assert_eq!(missile.check_collision(&asteroid), true);
         // le missile est désactivé après la collision
         assert_eq!(missile.is_active(), false);
     }
     #[test]
     fn test_no_collision() {
        let position = vec2(100.0, 100.0);
        let orientation = 0.0; // Missile dirigé vers le haut
        let screen_size = (800.0, 600.0);
        let mut missile = Missile::new(position, orientation, screen_size);

        let mut asteroid=crate::Asteroid::new(10.0, screen_size);
        asteroid.set_position(vec2(120.0, 120.0));

        // Vérifie qu'il n'y a pas de collision
        assert_eq!(missile.check_collision(&asteroid), false);
        // Vérifie que le missile est toujours actif
        assert_eq!(missile.is_active(), true);
    }

}
