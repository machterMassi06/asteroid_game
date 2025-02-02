use macroquad::prelude::*;
/// Trait qui encapsule les comportements communs des objets stellaires dans le jeu.
pub trait StellarObject{
    /// Renvoie la position Acutuelle de l'objet
    fn get_position(&self)->Vec2;

    /// Met à jour l'état de l'objet (déplacement)
    fn update(&mut self);

    /// Vérifie si cet objet stellaire est en collision avec un autre.
    /// Retourne `true` si une collision est détectée, `false` sinon.
    /// Par Défaut Retourne False si elle n'est pas surchargé.
    fn check_collision(&mut self, _other:&dyn StellarObject) -> bool{
        false
    }
    
    /// Renvoie la taille de l'object 
    /// Par Défaut Retourne 0.0 si elle n'est pas surchargé.
    fn get_size(&self)->f32{
        0.0
    }
  
}