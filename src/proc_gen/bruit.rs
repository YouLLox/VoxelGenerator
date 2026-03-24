pub use noise::{NoiseFn, Perlin, Seedable};

use image::{save_buffer, ColorType};

pub struct MapSeed
{
        pub value:u32,
}
pub fn generate_noise(map_height: usize , 
    map_width: usize , mut scale: f64,seed:u32)
    ->Vec<Vec<f64>>
{
    let mut noise_map=vec![vec![0.0;map_width];map_height];

    if scale<=0.00
    {
        scale =0.0001
    }

        // Si j'ai bien compris ici on va créer un objet perlin qui va permettre
        // de récupérer une valeure de perlin aléatoire en fonction des nombres 
        // donnés donc on va faire un calcul à chaque fois pour pouvoir avoir 
        // une valeure différente pour chaque élément de la noise map
    let perlinm = Perlin::new(seed);
    for x in 0..map_height 
    {
        for y in 0..map_width
        {
            let supplex= (x as f64)/scale;
            let suppley= (y as f64)/scale;
            

            noise_map[x][y]=perlinm.get([supplex,suppley]);
            
        }
    }       

    return noise_map;
}

pub fn setup_noise_texture( map_seed: MapSeed) 
{
    let width = 256;
    let height = 256;
    let scale = 20.0;


    let noise_map = generate_noise(height, width, scale,map_seed.value);


    let mut pixels: Vec<u8> = Vec::with_capacity(width * height * 4);

    for y in 0..height {
        for x in 0..width {
           

            //ici on normalise les valeurs aléatoires obtenues afin 
            //que ça corresponde à une nuance de gris
            let normalized = (noise_map[x][y] + 1.0) / 2.0;
            
            let color_val = (normalized * 255.0).clamp(0.0, 255.0) as u8;

            pixels.push(color_val); // R
            pixels.push(color_val); // G
            pixels.push(color_val); // B
            pixels.push(255);       // A
        }
    }

    //On va pouvoir sauvegarder la texture afin de la réutiliser 
    //plus tard ça sera peut être plus pertinent 
    //de se servir du buffer directement
    
    let filepath = (format!("noiseMap_seed_{}.png",map_seed.value)).to_string();
    match save_buffer(&filepath, &pixels, width as u32, 
        height as u32, ColorType::Rgba8) 
    {
        Ok(_) => println!("Image sauvegardée avec succès sous le nom : {}", 
            filepath),
        Err(e) => eprintln!("Erreur lors de la sauvegarde de l'image : {}", e),
    }

   
}
