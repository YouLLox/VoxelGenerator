use std::io;

pub use noise::{NoiseFn, Perlin, Seedable};

use super::bruit::setup_noise_texture;
use super::bruit::MapSeed;
use crate::proc_gen::bruit::generate_height_map;

pub fn noise_main() {

    println!("=== GÉNÉRATEUR DE NOISEMAP===");
    println!("Veuillez entrer une seed (un nombre entier, ex: 42, 1234, 99) :");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture");

    let seed: u32 = input.trim().parse().unwrap_or_else(|_| {
        println!("Entrée invalide. Utilisation de la seed par défaut : 1");
        1
    });

    println!("Veuillez entrer une largeur(un nombre entier) :");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture");

    let width: usize = input.trim().parse().unwrap_or_else(|_| {
        println!("Entrée invalide. Utilisation de la largeur par défaut : 856");
        856
    });
    
    println!("Veuillez entrer une largeur(un nombre entier) :");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture");

    let height: usize = input.trim().parse().unwrap_or_else(|_| {
        println!("Entrée invalide. Utilisation de la largeur par défaut : 856");
        856
    });

    println!("Génération de la noise_map en cours avec 
    la seed : {} 
    les dimensions : {} et {}... ", seed,width,height);
    let scale = 27.6;
    let octaves = 4;
    let persistance = 0.5;
    let lacunarity = 2.0;
    let offset=(100,100);
/* 
    // sécurités quand les valeurs ne seront plus hardcodée
    if octaves<0
    {
        let octaves=0
    }

    persistance.clamp(0.0,1.0);
    if lacunarity<1 
    {
        let lacunarity=1
    }
*/

    let map_seed=MapSeed{
        seed,
        width,
        height,
        scale,
        octaves,
        persistance,
        lacunarity,
        offset,
    };
    setup_noise_texture(&map_seed);
/*
   let height_map=generate_height_map(&map_seed,128);
    dbg!(height_map);
*/
}
