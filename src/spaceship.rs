use macroquad::prelude::*;
use crate::stellarobject::StellarObject;

/// Représente un vaisseau spatial avec une position, une vitesse,
/// une orientation et un niveau de bouclier.
pub struct Spaceship {
    screen_size : (f32,f32),
    position: Vec2,
    speed: Vec2,
    orientation: f32,
    shield: i32,
}

impl Spaceship {
    /// Def des constantes 
    pub const MAX_SPEED: f32 = 50.0;
    pub const MIN_SPEED: f32 = 0.05;
    pub const ACCELERATION: f32 = 10.0;
    pub const ROTATION_SPEED: f32 = 0.05;
    pub const FRICTION: f32 = 0.99;
    pub const INITIAL_SHIELD: i32 = 3;

    /// Crée un nouveau vaisseau spatial au centre de l'écran avec des paramètres par défaut.
    /// 
    /// # Arguments
    /// 
    /// * `screen_size` - taille de la fenetre de jeu.
    pub fn new(screen_size:(f32,f32)) -> Self {
        Self {
            screen_size,
            position: vec2(screen_size.0 / 2.0, screen_size.1 / 2.0),
            speed: vec2(0.0, 0.0),
            orientation: 0.0,
            shield: Self::INITIAL_SHIELD,
        }
    }

    /// Active la poussée pour accélérer le vaisseau dans la direction actuelle de son orientation.
    pub fn activate_thrust(&mut self) {
        let thrust = vec2(
            self.orientation.sin() * Self::ACCELERATION*0.0168,
            -self.orientation.cos() * Self::ACCELERATION*0.0168 ,
        );
        self.speed += thrust;

        // Limite la vitesse maximale
        if self.speed.length() > Self::MAX_SPEED {
            self.speed = self.speed.normalize() * Self::MAX_SPEED;
        }
    }

    /// Applique une rétro-poussée pour ralentir le vaisseau
    pub fn back_thrust(&mut self) {
        self.speed *= Self::FRICTION;
    }

    /// Tourne le vaisseau vers la gauche.
    pub fn left_rotation(&mut self) {
        self.orientation -= Self::ROTATION_SPEED;
    }

    /// Tourne le vaisseau vers la droite.
    pub fn right_rotation(&mut self) {
        self.orientation += Self::ROTATION_SPEED;
    }

    /// Retourne l'orientation actuelle du vaisseau.
    /// 
    /// # Returns
    /// 
    /// l'orientation du vaisseau.
    pub fn get_orientation(&self) -> f32 {
        self.orientation
    }

    ///Retourne le niveau bouclier
    /// 
    /// # Returns
    /// 
    /// Le niveau du bouclier
    pub fn get_shield(&self) ->i32 {
        self.shield
    }

    /// Diminue le bouclier du vaisseau de 1.
    pub fn decrease_shield(&mut self) {
        self.shield -= 1;
    }

    /// Vérifie si le vaisseau est détruit (bouclier égal ou inférieur à zéro).
    /// 
    /// # Returns
    /// 
    /// boolean (true si le bouclier <=0 , false sinon).
    pub fn is_destroyed(&self) -> bool {
        self.shield <= 0
    }
}

/// Implémentation du trait StellarObject pour la struct Spaceship
impl StellarObject for Spaceship {
    /// Retourne la position actuelle du vaisseau.
    /// 
    /// # Returns
    /// 
    /// Un vecteur représentant la position du vaisseau.
    fn get_position(&self) -> Vec2 {
        self.position
    }


    /// Met à jour la position du vaisseau, applique la friction et 
    /// gère le retour de l’autre côté de l’écran si le vaisseau dépasse les bords.
    
    fn update(&mut self) {
        // Applique la friction pour réduire progressivement la vitesse du vaisseau
        self.speed *= Self::FRICTION;

        // Empêche la vitesse de descendre en dessous de la vitesse minimale
        if self.speed.length() < Self::MIN_SPEED && self.speed.length() > 0.0 {
            self.speed = self.speed.normalize() * Self::MIN_SPEED;
        }

        // Met à jour la position du vaisseau en fonction de la vitesse
        self.position += self.speed;

        // Gestion des bords de l'écran : si le vaisseau sort d'un côté, il réapparaît de l'autre
        if self.position.x < 0.0 {
            self.position.x = self.screen_size.0;
        } else if self.position.x > self.screen_size.0 {
            self.position.x = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = self.screen_size.1;
        } else if self.position.y > self.screen_size.1 {
            self.position.y = 0.0;
        }
    }

    /// Vérifie la collision avec un astéroïde et gère la logique de bouclier.
    /// En cas de collision, le bouclier est diminué de 1.
    /// # Arguments
    /// * `_other` - Une réference a un dyn StelarObject.
    ///
    /// # Returns
    /// 
    /// * `true` si une collision est détectée, `false` sinon.
    fn check_collision(&mut self, _other:&dyn StellarObject) -> bool {
        let distance = self.position.distance(_other.get_position());
        let radius = 15.0;
        if distance< radius+_other.get_size() {
            self.position = vec2(self.screen_size.0/ 2.0, self.screen_size.1/ 2.0);
            self.speed = vec2(0.0, 0.0);
            self.orientation = 0.0;
            self.decrease_shield();
            return true;
        }
        false
    }
    
}

/// Tests unitaires 
/// 
#[test]
fn test_spaceship_new() {
    let screen_size = (800.0,600.0);
    let spaceship = Spaceship::new(screen_size);

    assert_eq!(spaceship.position,vec2(400.0,300.0));
    assert_eq!(spaceship.speed,vec2(0.0, 0.0));
    assert_eq!(spaceship.orientation,0.0);
    assert_eq!(spaceship.shield,Spaceship::INITIAL_SHIELD);
    assert_eq!(spaceship.screen_size,screen_size);
}

#[test]
fn test_update() {
    let screen_size = (800.0,600.0);
    let mut spaceship = Spaceship::new(screen_size);

    spaceship.update();
    spaceship.position = vec2(-10.0, 300.0); // Hors de l'écran à gauche
    spaceship.update();
    assert_eq!(spaceship.position.x, 800.0);

}

#[test]
fn test_spaceship_thrust() {
    let screen_size = (800.0, 600.0);
    let mut spaceship = Spaceship::new(screen_size);

    spaceship.activate_thrust();
    assert!(spaceship.speed.length() > 0.0);

    let initial_speed = spaceship.speed.length();
    spaceship.activate_thrust();
    assert!(spaceship.speed.length() > initial_speed);
}

#[test]
fn test_spaceship_rotation() {
    let screen_size = (800.0, 600.0);
    let mut spaceship = Spaceship::new(screen_size);

    spaceship.right_rotation();
    assert!(spaceship.orientation > 0.0);

    let orientation_after_right = spaceship.orientation;
    spaceship.left_rotation();
    assert!(spaceship.orientation < orientation_after_right);
}
#[test]
fn test_spaceship_collision() {
    let screen_size = (800.0, 600.0);
    let mut spaceship = Spaceship::new(screen_size);

    let mut asteroid=crate::Asteroid::new(20.0, screen_size);
    asteroid.set_position(vec2(400.0, 300.0));// Position identique au vaisseau

    let initial_shield = spaceship.shield;
    let collision = spaceship.check_collision(&asteroid);

    assert!(collision);
    assert_eq!(spaceship.shield, initial_shield - 1);
    assert_eq!(spaceship.position, vec2(400.0, 300.0)); // Réinitialisation au centre
    assert_eq!(spaceship.speed, vec2(0.0, 0.0)); // Vitesse réinitialisée
}

#[test]
fn test_spaceship_destruction() {
    let screen_size = (800.0, 600.0);
    let mut spaceship = Spaceship::new(screen_size);

    spaceship.shield = 1;
    spaceship.decrease_shield();
    assert!(spaceship.is_destroyed());

    spaceship.shield = 0;
    assert!(spaceship.is_destroyed());
}