use uuid::Uuid;

// Represents the output of a "Nœud de Néant" event.
pub struct ComputationResult {
    pub id: Uuid,
    pub value: String, // The actual result of the computation
    pub energy_released: f64,
}

impl ComputationResult {
    pub fn new(value: String, energy: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            value,
            energy_released: energy,
        }
    }
}