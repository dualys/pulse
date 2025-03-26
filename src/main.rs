use random_number::rand::random;
use std::{thread::sleep, time::Duration};

#[derive(Debug, Clone)]
pub enum Event {
    Wakeup,
    Inspire,
    Contract,
    Expire,
    Rest,
    Awake,
}

#[derive(Debug, Clone)]
pub enum Etat {
    Inconscient,
    Eveiller,
    Conscient,
    Veille,
    Calme,
    Endormit,
    Running,
}

#[derive(Debug, Clone)]
pub enum Emotion {
    Fatigue,
    Eveil,
    Pression,
    Joie,
    Surcharge,
    Vide,
    Soulagement,
}

impl Emotion {
    pub fn pause_range(self) -> (u64, u64) {
        match self {
            Emotion::Fatigue => (20_000_000, 60_000_000),
            Emotion::Eveil => (5_000_000, 15_000_000),
            Emotion::Pression => (2_000_000, 6_000_000),
            Emotion::Joie => (3_000_000, 12_000_000),
            Emotion::Surcharge => (1_000_000, 3_000_000),
            Emotion::Vide => (30_000_000, 80_000_000),
            Emotion::Soulagement => (10_000_000, 25_000_000),
        }
    }
}
pub struct PulseMemory {
    events: Vec<Event>,   // derniers événements perçus
    signals: Vec<String>, // signaux ou intentions
    tempo: Vec<u32>,      // historique du rythme
}

pub struct SignalBus {} // canal de communication
pub struct PlanShell {} // interface d’un plan
pub struct CompilerCell {} // cellules de compilation
pub struct PulseEnergy {} // énergie globale
pub struct EventMatrix {} // déclencheurs/réactions

pub struct Pulse {
    bpm: u32,
    memory: PulseMemory,
}

impl Pulse {
    pub fn wakeup() -> Self {
        Self {
            bpm: 60,
            memory: PulseMemory {
                events: vec![],
                signals: vec![],
                tempo: vec![],
            },
        }
    }

    pub fn awake(&mut self) -> Emotion {
        self.record(Event::Awake);
        Emotion::Fatigue
    }

    pub fn inspire(&mut self) -> Emotion {
        println!("inspire");
        self.record(Event::Inspire);
        Emotion::Eveil
    }

    pub fn contract(&mut self) -> Emotion {
        println!("contract");
        self.record(Event::Contract);
        Emotion::Pression
    }

    pub fn emit(&mut self, message: &str) {
        println!("emit");
        self.memory.events.push(Event::Contract);
        self.memory.signals.push(message.to_string());
    }

    pub fn expire(&mut self) -> Emotion {
        println!("expire");
        self.record(Event::Expire);
        Emotion::Soulagement
    }

    pub fn rest(&mut self) -> Emotion {
        println!("rest");
        Emotion::Vide
    }

    fn record(&mut self, e: Event) {
        self.memory.events.push(e.clone());
        self.memory.tempo.push(self.bpm);
    }

    fn pause(&self, emotion: Emotion) {
        let (min, max) = emotion.pause_range();
        let range = max - min;
        let rand_offset: u64 = random::<u64>() % range;
        let duration = min + rand_offset;
        sleep(Duration::from_nanos(duration));
    }

    pub fn beat(&mut self) {
        let e1: Emotion = self.inspire();
        self.pause(e1);

        let e2: Emotion = self.contract();
        self.pause(e2);

        self.emit("Signal vers plan dev");

        let e3: Emotion = self.expire();
        self.pause(e3);

        let e4: Emotion = self.rest();
        self.pause(e4);
    }
}

fn main() {
    let mut pulse: Pulse = Pulse::wakeup();
    loop {
        pulse.awake();
        pulse.beat();
    }
}
