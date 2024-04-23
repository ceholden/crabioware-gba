// Build Tiled map export JSON into Rust modules we can use
const LEVELS: &[&str] = &[
    "assets/maps/level-1.json",
];


fn main() {
    // let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable must be specified");

    // tiled_export::export_tilemap(&out_dir, "assets/tilemap.json").expect("Failed to export tilemap");
    // for &level in LEVELS {
    //     tiled_export::export_level(&out_dir, level).expect("Failed to export level");
    // }
}


mod tiled_export {
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write};

    pub fn export_tilemap(tilemap: &str, out_dir: &str) -> std::io::Result<()> {
        println!("cargo:rerun-if-changed={tilemap}");

        let file = File::open(tilemap)?;
        let reader = BufReader::new(file);

        let tilemap: TiledTilemap = serde_json::from_reader(reader)?;

        let output = File::create(format!("{out_dir}/tilemap.rs"))?;
        let mut writer = BufWriter::new(output);

        // let tile_data = HashMap<_, _> = tilemap
        //     .tiles
        //     .iter()
        //         .map(|tile| {

        //     })
        Ok(())
    }

    pub fn export_level(level: &str, out_dir: &str) -> std::io::Result<()> {
        Ok(())
    }

    #[derive(Deserialize)]
    struct TiledLevel {
        layers: Vec<TiledLayer>,
        width: i32,
        height: i32,
    }

    #[derive(Deserialize)]
    struct TiledLayer {
        name: String,
        #[serde(rename = "data")]
        tiles: Option<Vec<i32>>,
        objects: Option<Vec<TiledObject>>,
    }

    #[derive(Deserialize)]
    struct TiledObject {
        #[serde(rename = "type")]
        object_type: String,
        x: i32,
        y: i32,
    }

    #[derive(Deserialize)]
    struct TiledTilemap {
        tiles: Vec<TiledTile>,
        tilecount: i32,
    }

    #[derive(Deserialize)]
    struct TiledTile {
        id: i32,
        #[serde(rename = "type")]
        tile_type: String,
    }

}
