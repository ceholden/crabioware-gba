use std::path::Path;

// Remap GID for these layers
const REMAPPED_LAYERS: &[&str] = &["Path", "Dots"];

// Tile size in pixels
const TILE_SIZE_PIXELS: i32 = 8;

// Build Tiled map export JSON into Rust modules we can use
const LEVELS: &[&str] = &["assets/maps/level-1.json"];

fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable must be specified");

    let (tile_types, tile_type_names) = tiled_export::export_tilemap(&out_dir, "assets/tilemap.json")
        .expect("Failed to export tilemap");
    for &level in LEVELS {
        tiled_export::export_level(&out_dir, Path::new(level), &tile_types, &tile_type_names)
            .expect("Failed to export level");
    }
}

// TODO: We can use tiled crate to better handle tmx files..
//          In particular this would be useful for handling flipped tiles
mod tiled_export {
    use crate::{REMAPPED_LAYERS, TILE_SIZE_PIXELS};

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write};
    use std::path::Path;

    use itertools::Itertools;
    use serde::Deserialize;

    /// Export mappings: (tile_id -> type_index, type_name -> type_index)
    pub fn export_tilemap(out_dir: &str, tilemap: &str) -> std::io::Result<(HashMap<i32, usize>, HashMap<String, usize>)> {
        // println!("cargo:rerun-if-changed={tilemap}");
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

        let tile_types_to_ids: HashMap<&str, usize> = tile_types
            .iter()
            .enumerate()
            .map(|(idx, &tile_type)| (tile_type, idx))
            .collect();
        let name_map: HashMap<String, usize> = tile_types_to_ids
            .iter()
            .map(|(&name, &idx)| (name.to_string(), idx))
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
            // tile indices
            let tile_type_id = tile_types_to_ids.get(tile_type).unwrap();
            writeln!(
                &mut writer,
                "pub const {}: i32 = {tile_type_id};",
                tile_type.to_uppercase()
            )?;

            // tile GIDs for visual rendering
            if let Some((&gid, _)) = tile_ids_to_types.iter().find(|(_, &ty)| ty == *tile_type) {
                writeln!(
                    &mut writer,
                    "pub const {}_GID: usize = {gid};",
                    tile_type.to_uppercase()
                )?;
            }
        }

        // FIXME: dead code?
        writeln!(&mut writer, "pub const TILE_DATA: &[u32] = &[{tile_info}];")?;

        let type_map = tile_ids_to_types
            .iter()
            .map(|(&id, ty)| (id, *tile_types_to_ids.get(ty).unwrap()))
            .collect();
        Ok((type_map, name_map))
    }

    pub fn export_level(
        out_dir: &str,
        level_file: &Path,
        tile_types: &HashMap<i32, usize>,
        tile_type_names: &HashMap<String, usize>,
    ) -> std::io::Result<()> {
        // println!("cargo:rerun-if-changed={level_file:?}");
        let file = File::open(level_file).expect("Cannot read level file {level_file}");
        let reader = BufReader::new(file);

        let level: TiledLevel = serde_json::from_reader(reader)?;

        let filename = level_file.file_name().unwrap().to_str().unwrap();
        let output_file = File::create(format!("{out_dir}/{filename}.rs"))?;
        let mut writer = BufWriter::new(output_file);

        let dot_type_idx = tile_type_names.get("Dot").copied().unwrap_or(usize::MAX);
        let pellet_type_idx = tile_type_names.get("Pellet").copied().unwrap_or(usize::MAX);

        let tile_layers: HashMap<_, _> = level
            .layers
            .iter()
            .filter(|layer| layer.data.is_some())
            .sorted_by(|a, b| Ord::cmp(&a.id, &b.id))
            .map(|layer| {
                (
                    layer.name.clone(),
                    layer
                        .data
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|&gid| {
                            if REMAPPED_LAYERS.contains(&layer.name.as_str()) {
                                // Remap GIDs to tile type indices so runtime check is `== PATH`
                                if gid == 0 {
                                    0
                                } else {
                                    *tile_types.get(&(gid - 1)).unwrap_or(&0)
                                }
                            } else {
                                gid as usize
                            }
                        })
                        .map(|t| t.to_string())
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
                        .map(|obj| {
                            (
                                &obj.object_name,
                                (
                                    (obj.x / TILE_SIZE_PIXELS).to_string(),
                                    (obj.y / TILE_SIZE_PIXELS).to_string(),
                                ),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect();

        writeln!(&mut writer, "// AUTO-GENERATED")?;
        writeln!(&mut writer, "// Level data for {filename}")?;
        writeln!(&mut writer, "const WIDTH: u32 = {};", level.width)?;
        writeln!(&mut writer, "const HEIGHT: u32 = {};", level.height)?;
        writeln!(
            &mut writer,
            "const TILE_SIZE_PIXELS: u8 = {};",
            TILE_SIZE_PIXELS
        )?;

        let total_dots = tile_layers
            .get("Dots")
            .map(|data| {
                data.split(", ")
                    .filter_map(|s| s.parse::<usize>().ok())
                    .filter(|&t| t == dot_type_idx || t == pellet_type_idx)
                    .count()
            })
            .unwrap_or(0);
        writeln!(&mut writer, "const TOTAL_DOTS: usize = {total_dots};")?;

        writeln!(&mut writer, "// Tilemap layers")?;
        for (name, data) in tile_layers.iter() {
            let varname = name.to_uppercase();
            writeln!(&mut writer, "const {varname}: &[u8] = &[{data}];")?;
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
                    "const {}_{}: &[(u8, u8)] = &[{xys}];",
                    layer_name.to_uppercase(),
                    obj_name.to_uppercase()
                )?;
            }
        }

        // PacCrab specific part...
        writeln!(
            &mut writer,
            r#"
use agb::fixnum::Vector2D;
use crate::levels::Level;

pub const fn get_level() -> Level {{
    Level {{
        walls: BACKGROUND,
        path: PATH,
        dots: DOTS,
        dimensions: Vector2D {{ x: WIDTH, y: HEIGHT }},
        tile_size: TILE_SIZE_PIXELS,
        total_dots: TOTAL_DOTS,
        spawn: &POINTS_SPAWN[0],
        ghosts: POINTS_GHOST,
        berries: POINTS_BERRY,
        doors: POINTS_DOOR,
        warps: POINTS_WARP,
    }}
}}
"#
        )?;

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
        #[serde(rename = "type")]
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
