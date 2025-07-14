// In your file: src/main.rs

use std::{thread, time};

mod computation_result;
mod physics;
mod substrat;
mod wave;

use physics::PhysicsEngine;
use substrat::Substrat;
use crate::wave::Wave;

fn main() {
    // ... (initialization code is the same)
    println!("--- [PULSE CORE] : Initialisation du Substrat ---");
    let _univers = Substrat::new(100, 100, 100);

    let mut engine = PhysicsEngine::new(1.0);

    let mut plan_a = Wave::new();
    plan_a.position.x = -10.0;
    plan_a.frequency = 440.0; // Give it a property

    let mut plan_b = plan_a.conjugate();
    plan_b.position.x = 10.0;
    plan_b.frequency = 880.0; // Give it a different property

    engine.add_wave(plan_a);
    engine.add_wave(plan_b);

    println!("\n--- [PULSE CORE] : Démarrage de la boucle de simulation ---");
    let mut tick_count = 0;
    loop {
        tick_count += 1;
        println!("\n--- [TICK {}] ---", tick_count);

        if engine.waves.is_empty() {
            println!("--- [SILENCE] : Univers vide. ---");
            break;
        }

        // --- NOUVELLE LOGIQUE : CAPTURER LE RÉSULTAT ---
        if let Some(result) = engine.step() {
            println!("--- [RÉSULTAT DE CALCUL] : Un résultat a été généré ! ---");
            println!("  ID      : {}", result.id);
            println!("  Valeur  : '{}'", result.value);
            println!("  Énergie : {}", result.energy_released);
        }

        // ... (display logic for waves is the same)
        for wave in &engine.waves {
            println!(
                "Onde {} @ ({:.1}, {:.1}, {:.1})",
                wave.id, wave.position.x, wave.position.y, wave.position.z
            );
        }

        thread::sleep(time::Duration::from_millis(500));
    }
}
