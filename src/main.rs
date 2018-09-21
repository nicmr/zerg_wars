mod traits;
mod gameobject;
mod gamestate;
mod player;
mod constants;

extern crate ggez;
extern crate reqwest;

use ggez::event;
use ggez::{Context};
use ggez::conf;

use std::{env, path};
use std::fs::File;
use std::io::{Read, Write};


use gamestate::GameState;


fn download_assets(files: Vec<(&str, &str)>){ //consuming
    for (filename, url) in files{
        if !path::Path::new(&filename).exists(){
            let mut file = File::create(format!("assets/{}", &filename)).unwrap();
            let mut bytes = Vec::with_capacity(1000);
            let mut response = reqwest::get(url).unwrap();
            response.read_to_end(&mut bytes).expect("error when decoding response");

            file.write_all(&bytes).expect("error when trying to write to file");
        }
    }
    
}


fn main() {

    //move this to a  json file and loaded at launch instead
    let required_assets = vec![("ggez_bane_left.png", "https://i.imgur.com/SlJSqCG.png"),
    ("ggez_bane_left.png", "https://i.imgur.com/SlJSqCG.png"),
    ("ggez_bane_right.png", "https://i.imgur.com/8gcy6x3.png"),
    ("ggez_hydra_left.png", "https://i.imgur.com/IjeOQC5.png"),
    ("ggez_hydra_right.png", "https://i.imgur.com/6pvKuci.png"),
    ("ggez_zergling_left.png", "https://i.imgur.com/pF4RqP7.png"),
    ("ggez_zergling_right.png", "https://i.imgur.com/K0SpT6E.png"),
    ("hatchery.png", "https://i.imgur.com/DUJDut5.png"),
    ];

    download_assets(required_assets);


    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    //mount the assets folder into the ggez filesystem
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR"){
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        println!("{:?}", path);
        ctx.filesystem.mount(&path, true)
    }

    //create gamestate
    let state = &mut GameState::new(ctx).unwrap();

    //run gamestate
    if let Err(e) = event::run(ctx, state){
        println!("Error encountered: {}", e);
    }else{
        println!("Game exited cleanly.");
    }
}


