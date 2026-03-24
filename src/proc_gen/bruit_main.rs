
use std::io;


pub use noise::{NoiseFn, Perlin, Seedable};

use super::bruit::setup_noise_texture;
use super::bruit::MapSeed;


pub fn noise_main() {

    println!("=== GÉNÉRATEUR DE NOISEMAP===");
    println!("Veuillez entrer une seed (un nombre entier, ex: 42, 1234, 99) :");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture");

    let seed: u32 = input.trim().parse().unwrap_or_else(|_| {
        println!("Entrée invalide. Utilisation de la seed par défaut : 1");
        1
    });

    println!("Génération en cours avec la seed : {}...", seed);
    
    setup_noise_texture(MapSeed{value:seed});
}
