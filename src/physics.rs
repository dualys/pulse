
use crate::wave::Wave;
use crate::computation_result::ComputationResult; // Import our new struct

pub struct PhysicsEngine {
    pub waves: Vec<Wave>,
    pub time_step: f64,
}

impl PhysicsEngine {
    // ... (existing code)
    pub fn new(time_step: f64) -> Self {
        Self {
            waves: Vec::new(),
            time_step,
        }
    }

    pub fn add_wave(&mut self, wave: Wave) {
        self.waves.push(wave);
    }

    // The function now returns an optional result
    pub fn step(&mut self) -> Option<ComputationResult> {
        // ... (propagation logic is the same)
        for wave in self.waves.iter_mut() {
            wave.propagate(self.time_step);
        }

        let mut to_remove = vec![];
        let mut computation_result: Option<ComputationResult> = None;

        for i in 0..self.waves.len() {
            for j in (i + 1)..self.waves.len() {
                let wave_a = &self.waves[i];
                let wave_b = &self.waves[j];

                if (wave_a.position - wave_b.position).norm() < 0.1 {
                    println!("\n--- [NŒUD DE NÉANT] : Interférence constructive maximale ! ---");

                    // --- NOUVELLE LOGIQUE : GÉNÉRATION DU RÉSULTAT ---
                    // The result is a combination of the two waves' information.
                    // For now, we'll just use their frequencies.
                    let energy = wave_a.amplitude + wave_b.amplitude;
                    let result_value = format!("Fusion of frequencies: {} & {}", wave_a.frequency, wave_b.frequency);

                    computation_result = Some(ComputationResult::new(result_value, energy));

                    to_remove.push(i);
                    to_remove.push(j);

                    // We only handle one collision per step for simplicity
                    break;
                }
            }
            if computation_result.is_some() {
                break;
            }
        }

        // ... (removal logic is the same)
        if !to_remove.is_empty() {
            to_remove.sort_unstable();
            to_remove.reverse();
            to_remove.dedup();
            for index in to_remove {
                self.waves.remove(index);
            }
        }

        computation_result
    }
}