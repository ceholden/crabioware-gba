use std::path::Path;

// Build Tiled map export JSON into Rust modules we can use
const LEVELS: &[&str] = &["assets/maps/level-1.json"];

fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable must be specified");

    tiled_export::export_tilemap(&out_dir, "assets/tilemap.json")
        .expect("Failed to export tilemap");
    for &level in LEVELS {
        tiled_export::export_level(&out_dir, Path::new(level)).expect("Failed to export level");
    }
}

mod tiled_export {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write};
    use std::path::Path;

    use itertools::Itertools;
    use serde::Deserialize;

    pub fn export_tilemap(out_dir: &str, tilemap: &str) -> std::io::Result<()> {
        println!("cargo:rerun-if-changed={tilemap}");
        let file = File::open(tilemap)?;
        let reader = BufReader::new(file);

        let tilemap: TiledTilemap = serde_json::from_reader(reader)?;

        let output = File::create(format!("{out_dir}/tilemap.rs"))?;
        let mut writer = BufWriter::new(output);

        let mut tile_types = Vec::<&str>::new();
        let mut tile_ids_to_types = HashMap::<i32, &str>::new();
        for tile in tilemap.tiles.iter() {
            let tile_type = tile.tile_type.as_str();
            if !tile_types.contains(&tile_type) {
                tile_types.push(tile_type);
            }
            tile_ids_to_types.insert(tile.id, tile_type);
        }

        let tile_types_to_ids: HashMap<_, _> = tile_types
            .iter()
            .enumerate()
            .map(|(idx, tile_type)| (tile_type, idx))
            .collect();

        let tile_info = (0..tilemap.tilecount)
            .map(|tile_id| {
                tile_types_to_ids
                    .get(tile_ids_to_types.get(&tile_id).unwrap())
                    .unwrap_or(&0)
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join(", ");

        writeln!(&mut writer, "// AUTO-GENERATED")?;
        writeln!(&mut writer, "// Tilemap data")?;
        for tile_type in tile_types.iter() {
            let tile_type_id = tile_types_to_ids.get(tile_type).unwrap();
            let tile_type_upper = tile_type.to_uppercase();
            writeln!(
                &mut writer,
                "pub const {tile_type_upper}: i32 = {tile_type_id};"
            )?;
        }
        writeln!(&mut writer, "pub const TILE_DATA: &[u32] = &[{tile_info}];")?;

        Ok(())
    }

    pub fn export_level(out_dir: &str, level_file: &Path) -> std::io::Result<()> {
        println!("cargo:rerun-if-changed={level_file:?}");
        let file = File::open(level_file).expect("Cannot read level file {level_file}");
        let reader = BufReader::new(file);

        let level: TiledLevel = serde_json::from_reader(reader)?;

        let filename = level_file.file_name().unwrap().to_str().unwrap();
        let output_file = File::create(format!("{out_dir}/{filename}.rs"))?;
        let mut writer = BufWriter::new(output_file);

        let tile_layers: HashMap<_, _> = level
            .layers
            .iter()
            .filter(|layer| layer.data.is_some())
            .sorted_by(|a, b| Ord::cmp(&a.id, &b.id))
            .map(|layer| {
                (
                    &layer.name,
                    layer
                        .data
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                )
            })
            .collect();

        let object_layers: HashMap<_, _> = level
            .layers
            .iter()
            .filter(|layer| layer.objects.is_some())
            .sorted_by(|a, b| Ord::cmp(&a.id, &b.id))
            .map(|layer| {
                (
                    &layer.name,
                    layer
                        .objects
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|obj| (&obj.object_name, (obj.x.to_string(), obj.y.to_string())))
                        .collect::<Vec<_>>(),
                )
            })
            .collect();

        writeln!(&mut writer, "// AUTO-GENERATED")?;
        writeln!(&mut writer, "// Level data for {filename}")?;
        writeln!(&mut writer, "const WIDTH: u32 = {};", level.width)?;
        writeln!(&mut writer, "const HEIGHT: u32 = {};", level.height)?;

        writeln!(&mut writer, "// Tilemap layers")?;
        for (name, data) in tile_layers.iter() {
            let varname = name.to_uppercase();
            writeln!(&mut writer, "const {varname}: &[u16] = &[{data}];")?;
        }

        writeln!(&mut writer, "// Object layers")?;
        for (layer_name, objects) in object_layers.iter() {
            let objects_by_type: HashMap<_, _> = objects
                .iter()
                .group_by(|(obj_name, _)| obj_name)
                .into_iter()
                .map(|(obj_name, group)| {
                    (
                        obj_name,
                        group
                            .map(|(_, (x, y))| format!("({x}, {y})"))
                            .collect::<Vec<String>>()
                            .join(", "),
                    )
                })
                .collect();
            for (obj_name, xys) in objects_by_type.iter() {
                writeln!(
                    &mut writer,
                    "const {}_{}: &[(i32, i32)] = &[{xys}];",
                    layer_name.to_uppercase(),
                    obj_name.to_uppercase()
                )?;
            }
        }

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
        id: i32,
        name: String,
        data: Option<Vec<i32>>,
        objects: Option<Vec<TiledObject>>,
    }

    #[derive(Deserialize)]
    struct TiledObject {
        #[serde(rename = "name")]
        object_name: String,
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
