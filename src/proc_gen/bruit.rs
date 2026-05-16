use rand::prelude::*;
use chacha20::ChaCha8Rng;

pub use noise::{NoiseFn, Perlin, Seedable};

use image::{save_buffer, ColorType};

pub struct MapSeed
{
        pub seed:u32,
        pub width:usize,
        pub height:usize,
        pub scale:f64,
        pub octaves:u32,
        pub persistance:f64,
        pub lacunarity:f64,
        pub offset:(i32,i32),
}

fn inv_lerp(a:f64,b:f64,x:f64)->f64
{
    (a-b)/(x-b)
}

fn lerp(a:f64,b:f64,x:f64)->f64
{
    (1.0- a)*b + a*x
}
pub fn generate_noise(map_height: usize , 
    map_width: usize , mut scale: f64,seed:u32,octaves:u32,persistance:f64,
    lacunarity:f64,offset:(u32,u32))
    ->Vec<Vec<f64>>
{
    let mut noise_map=vec![vec![0.0;map_width];map_height];
    let mut octave_offsets=vec![(octaves,octaves);octaves as usize];
    
    let mut prng:ChaCha8Rng=ChaCha8Rng::seed_from_u64(seed as u64);

    for i in 0..(octaves as usize) 
    {
        let offsetx= prng.next_u32()+offset.0;
        let offsety= prng.next_u32()+offset.1;
        octave_offsets[i]=(offsetx,offsety);
    }           

    if scale<=0.00
    {
        scale =0.0001
    }

        // Si j'ai bien compris ici on va créer un objet perlin qui va permettre
        // de récupérer une valeure de perlin aléatoire en fonction des nombres 
        // donnés donc on va faire un calcul à chaque fois pour pouvoir avoir 
        // une valeure différente pour chaque élément de la noise map
    

    let half_width=(map_width as f64)/2.0;
    let half_height=(map_height as f64)/2.0;
    let perlinm = Perlin::new(seed);

    let mut maxNoiseHeight=f64::MIN;

    let mut minNoiseHeight=f64::MAX;

    for x in 0..map_height 
    {
        for y in 0..map_width
        {
            let mut amplitude:f64=1.0;
            let mut frequency:f64=1.0;
            let mut noiseHeight:f64=0.0;
            
            for i in 0..(octaves as usize)
            {
                let supplex= ((x as f64)-half_height)/scale*frequency + 
                    (octave_offsets[i].0 as f64);
                let suppley= ((y as f64)-half_width)/scale*frequency + 
                    (octave_offsets[i].1 as f64);
            
                let perlinValue=perlinm.get([supplex,suppley]);
                noiseHeight+=perlinValue*amplitude;

                amplitude*=persistance;
                frequency*=lacunarity;
            
            }
            if noiseHeight>maxNoiseHeight
            {
                maxNoiseHeight=noiseHeight;
            }
            else if noiseHeight<minNoiseHeight
            {
                minNoiseHeight=noiseHeight;
            }
            noise_map[x][y]=noiseHeight;
        }
    }       
    /*
    for x in 0..map_height  
    {
        for y in 0..map_width
        {
            //ici on normalise la valeur entre minNoiseHeight et maxNoiseHeight 
            //avec la fonction inv_lerp défini précédemment 
            
            noise_map[x][y]=inv_lerp(minNoiseHeight,maxNoiseHeight,
                noise_map[x][y]);
        }
    }
*/
    return noise_map;
}

pub fn setup_noise_texture( map_seed: &MapSeed) 
{
    let width = map_seed.width;
    let height = map_seed.height;
    let seed = map_seed.seed;
    let scale = map_seed.scale;
    let octaves = map_seed.octaves;
    let persistance = map_seed.persistance;
    let lacunarity = map_seed.lacunarity;
    let offset =map_seed.offset;
    let noise_map = generate_noise(height, width, scale,seed,
        octaves,persistance,lacunarity,offset);


    let mut pixels: Vec<u8> = Vec::with_capacity(width * height * 4);

    for x in 0..height {
        for y in 0..width {
           

            //ici on normalise les valeurs aléatoires obtenues afin 
            //que ça corresponde à une nuance de gris
            /*
            let normalized = (noise_map[x][y] + 1.0) / 2.0;
            
            let color_val = (normalized * 255.0).clamp(0.0, 255.0) as u8;
            */

            let normalized = (noise_map[x][y]+1.0)/2.0;

            let color_val = (normalized * 255.0).clamp(0.0, 255.0) as u8;
            //let color_val=lerp(0.0,255.0,noise_map[x][y]) as u8;
            pixels.push(color_val); // R
            pixels.push(color_val); // G
            pixels.push(color_val); // B
            pixels.push(255);       // A
        }
    }

    //On va pouvoir sauvegarder la texture afin de la réutiliser 
    //plus tard ça sera peut être plus pertinent 
    //de se servir du buffer directement
    
    let filepath = (format!("nMap_seed_{}_dim_{}x{}_o{}x{}.png",
        seed,width,height,offset.0,offset.1)).to_string();
    match save_buffer(&filepath, &pixels, width as u32, 
        height as u32, ColorType::Rgba8) 
    {
        Ok(_) => println!("Image sauvegardée avec succès sous le nom : {}", 
            filepath),
        Err(e) => eprintln!("Erreur lors de la sauvegarde de l'image : {}", e),
    }

   
}


pub fn generate_height_map(map_seed: &MapSeed, max_height:usize)
    ->Vec<Vec<u32>>
{
    let width = map_seed.width;
    let depth = map_seed.height;

    let noise_map = generate_noise(
        depth, 
        width, 
        map_seed.scale,
        map_seed.seed,
        map_seed.octaves,
        map_seed.persistance,
        map_seed.lacunarity,
        map_seed.offset,
    );

    let mut height_map = vec![vec![0; width]; depth];

    for z in 0..depth {
        for x in 0..width {
            let normalized = (noise_map[z][x] + 1.0) / 2.0;
            
            let clamped = normalized.clamp(0.0, 1.0);

            let voxel_height = (clamped * (max_height as f64)) as u32;

            height_map[z][x] = voxel_height;
        }
    }

    height_map
}
