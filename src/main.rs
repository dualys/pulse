// Dans votre fichier : src/main.rs

use std::{thread, time};
use nalgebra::Vector3;
use rand::Rng; // Importez le trait Rng de la bibliothèque rand

// ... (les autres `mod` et `use` ne changent pas)
mod computation_result;
mod physics;
mod substrat;
mod wave;

use physics::PhysicsEngine;
use substrat::Substrat;
use wave::Wave;

fn main() {
    // ... (l'initialisation ne change pas)
    println!("--- [PULSE CORE] : Initialisation du Substrat ---");
    let _univers = Substrat::new(100, 100, 100);
    let mut engine = PhysicsEngine::new(1.0);

    let mut plan_a = Wave::new();
    plan_a.position.x = -10.0;
    plan_a.frequency = 440.0;

    let mut plan_b = plan_a.conjugate();
    plan_b.position.x = 10.0;
    plan_b.frequency = 880.0;

    engine.add_wave(plan_a);
    engine.add_wave(plan_b);

    println!("\n--- [PULSE CORE] : Démarrage de la boucle de simulation ---");

    // Initialisez le générateur de nombres aléatoires
    let mut rng = rand::thread_rng();
    let mut tick_count = 0;

    loop {
        tick_count += 1;
        println!("\n--- [TICK {}] ---", tick_count);

        if engine.waves.is_empty() {
            println!("--- [SILENCE] : Univers vide. Le cycle est terminé. ---");
            break;
        }

        if let Some(result) = engine.step() {
            println!("--- [RÉSULTAT DE CALCUL] : Un résultat a été généré ! ---");
            println!("  Valeur  : '{}'", result.value);

            println!("--- [RÉMANENCE] : Le résultat engendre une nouvelle onde ! ---");
            let mut new_wave = Wave::new();
            new_wave.frequency = result.energy_released;

            // --- LOGIQUE MISE À JOUR : DIRECTION ALÉATOIRE ---
            // On génère des composantes x, y, z aléatoires entre -1.0 et 1.0
            let x = rng.gen_range(-1.0..=1.0);
            let y = rng.gen_range(-1.0..=1.0);
            let z = rng.gen_range(-1.0..=1.0);

            // On normalise le vecteur pour que sa longueur soit 1 (direction pure)
            new_wave.direction = Vector3::new(x, y, z).normalize();

            engine.add_wave(new_wave);
        }

        // ... (l'affichage de l'état ne change pas)
        for wave in &engine.waves {
            println!(
                "Onde {} @ ({:.1}, {:.1}, {:.1}) | Freq: {:.1}",
                wave.id, wave.position.x, wave.position.y, wave.position.z, wave.frequency
            );
        }

        thread::sleep(time::Duration::from_millis(500));
    }
}