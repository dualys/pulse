use nalgebra::Vector3;
use uuid::Uuid;

pub struct Wave {
    pub id: Uuid,
    pub amplitude: f64,
    pub frequency: f64,
    pub position: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Wave {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            amplitude: 1.0,
            frequency: 1.0,
            position: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(1.0, 0.0, 0.0),
        }
    }

    pub fn propagate(&mut self, time_step: f64) {
        // La nouvelle position est l'ancienne plus la direction multipliée par le temps écoulé.
        self.position += self.direction * time_step;
    }
    pub fn conjugate(&self) -> Self {
        println!("--- [CONJugaISON DE PHASE] : L'onde {} est clonée ! ---", self.id);
        Self {
            id: Uuid::new_v4(), // Le clone a son propre ID unique.
            amplitude: self.amplitude,
            frequency: self.frequency,
            position: self.position, // Le clone apparaît à la position exacte de son parent.
            direction: -self.direction, // La direction est inversée. C'est le cœur du concept.
        }
    }
}
