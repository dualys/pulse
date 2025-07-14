// Dans votre fichier : src/main.rs

use std::{thread, time};

mod wave;
mod substrat;

use wave::Wave;
use substrat::Substrat;

fn main() {
    println!("--- [PULSE CORE] : Initialisation du Substrat ---");
    let _univers = Substrat::new(100, 100, 100);
    println!("--- [SUBSTRAT] : Espace-temps créé.");

    let mut plan_a = Wave::new();
    println!("--- [GENÈSE] : Vague A créée. ID: {}", plan_a.id);

    // Nous devons pouvoir stocker la Vague B quand elle sera créée.
    let mut plan_b: Option<Wave> = None;

    println!("\n--- [PULSE CORE] : Démarrage de la boucle de simulation temporelle ---");

    let time_step = 1.0;
    let mut tick_count = 0;

    loop {
        // --- MOTEUR PHYSIQUE ---
        // Fait avancer la Vague A
        plan_a.propagate(time_step);

        // Fait avancer la Vague B si elle existe
        if let Some(plan) = &mut plan_b {
            plan.propagate(time_step);
        }

        tick_count += 1;

        // --- AFFICHAGE DE L'ÉTAT ---
        println!("[Tick: {}] Vague A @ ({:.1}, {:.1}, {:.1})",
                 tick_count,
                 plan_a.position.x,
                 plan_a.position.y,
                 plan_a.position.z
        );

        if let Some(plan) = &plan_b {
            println!("[Tick: {}] Vague B @ ({:.1}, {:.1}, {:.1})",
                     tick_count,
                     plan.position.x,
                     plan.position.y,
                     plan.position.z
            );
        }

        // --- ÉVÉNEMENT DE CLONAGE ---
        // Au 5ème tick, on déclenche le clonage.
        if tick_count == 5 && plan_b.is_none() {
            plan_b = Some(plan_a.conjugate());
        }

        thread::sleep(time::Duration::from_millis(500));
        println!("--------------------------------------------------");
    }
}