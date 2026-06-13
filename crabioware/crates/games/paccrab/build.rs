use std::path::Path;

const TILE_SIZE_PIXELS: i32 = 8;
const LEVELS: &[&str] = &["assets/maps/level-1.json"];

fn main() {
    println!("cargo:rerun-if-changed=assets/tileset.json");
    for &level in LEVELS {
        println!("cargo:rerun-if-changed={level}");
    }

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable must be specified");

    let type_name_to_tile_id = tiled_export::export_tileset(&out_dir, "assets/tileset.json")
        .expect("Failed to export tileset");
    for &level in LEVELS {
        tiled_export::export_level(&out_dir, Path::new(level), &type_name_to_tile_id)
            .expect("Failed to export level");
    }
}

mod tiled_export {
    use crate::TILE_SIZE_PIXELS;

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write};
    use std::path::Path;

    use itertools::Itertools;
    use serde::Deserialize;

    /// Returns type_name -> tile_id (0-based tileset index).
    /// Generates _TILE_ID constants into tileset.rs.
    pub fn export_tileset(
        out_dir: &str,
        tileset_path: &str,
    ) -> std::io::Result<HashMap<String, i32>> {
        let file = File::open(tileset_path)?;
        let tileset: TiledTileset = serde_json::from_reader(BufReader::new(file))?;

        let output = File::create(format!("{out_dir}/tileset.rs"))?;
        let mut writer = BufWriter::new(output);

        writeln!(&mut writer, "// AUTO-GENERATED")?;

        // One constant per type (first tile_id seen for that type name)
        let mut type_name_to_tile_id = HashMap::<String, i32>::new();
        for tile in tileset.tiles.iter() {
            let type_name = tile.tile_type.as_str();
            type_name_to_tile_id
                .entry(type_name.to_string())
                .or_insert(tile.id);
        }

        // Write in stable order
        let mut entries: Vec<_> = type_name_to_tile_id.iter().collect();
        entries.sort_by_key(|(_, &tile_id)| tile_id);
        for (type_name, &tile_id) in entries.iter() {
            writeln!(
                &mut writer,
                "pub const {}_TILE_ID: usize = {tile_id};",
                type_name.to_uppercase()
            )?;
        }

        Ok(type_name_to_tile_id)
    }

    /// All layer data is stored as (gid - firstgid), i.e. 0-based local tile_id.
    /// Empty cells (gid == 0) are stored as 0xFF.
    pub fn export_level(
        out_dir: &str,
        level_file: &Path,
        type_name_to_tile_id: &HashMap<String, i32>,
    ) -> std::io::Result<()> {
        let file = File::open(level_file).expect("Cannot read level file");
        let level: TiledLevel = serde_json::from_reader(BufReader::new(file))?;

        // firstgid for each tileset, sorted so we can find owning tileset per gid
        let mut firstgids: Vec<i32> = level.tilesets.iter().map(|ts| ts.firstgid).collect();
        firstgids.sort_unstable();

        let filename = level_file.file_name().unwrap().to_str().unwrap();
        let output_file = File::create(format!("{out_dir}/{filename}.rs"))?;
        let mut writer = BufWriter::new(output_file);

        let dot_tile_id = type_name_to_tile_id.get("Dot").copied().unwrap_or(-1) as u8;
        let pellet_tile_id = type_name_to_tile_id.get("Pellet").copied().unwrap_or(-1) as u8;

        // Convert gid -> local tile_id within its tileset (0-based), empty = 0xFF
        let to_tile_id = |gid: i32| -> u8 {
            if gid == 0 {
                return 0xFF;
            }
            // Find the tileset with the largest firstgid <= gid
            let firstgid = firstgids
                .iter()
                .filter(|&&fg| fg <= gid)
                .last()
                .copied()
                .unwrap_or(1);
            (gid - firstgid) as u8
        };

        let tile_layers: HashMap<String, Vec<u8>> = level
            .layers
            .iter()
            .filter(|layer| layer.data.is_some())
            .map(|layer| {
                let data = layer
                    .data
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|&gid| to_tile_id(gid))
                    .collect();
                (layer.name.clone(), data)
            })
            .collect();

        // Object layers: pixel coords -> tile coords via TILE_SIZE_PIXELS
        let object_layers: HashMap<String, Vec<(&str, (u8, u8))>> = level
            .layers
            .iter()
            .filter(|layer| layer.objects.is_some())
            .map(|layer| {
                let objects = layer
                    .objects
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|obj| {
                        (
                            obj.object_type.as_str(),
                            (
                                (obj.x / TILE_SIZE_PIXELS) as u8,
                                (obj.y / TILE_SIZE_PIXELS) as u8,
                            ),
                        )
                    })
                    .collect();
                (layer.name.clone(), objects)
            })
            .collect();

        let total_dots = tile_layers
            .get("Dots")
            .map(|data| {
                data.iter()
                    .filter(|&&t| t == dot_tile_id || t == pellet_tile_id)
                    .count()
            })
            .unwrap_or(0);

        writeln!(&mut writer, "// AUTO-GENERATED")?;
        writeln!(&mut writer, "// Level data for {filename}")?;
        writeln!(&mut writer, "const WIDTH: u32 = {};", level.width)?;
        writeln!(&mut writer, "const HEIGHT: u32 = {};", level.height)?;
        writeln!(
            &mut writer,
            "const TILE_SIZE_PIXELS: u8 = {};",
            TILE_SIZE_PIXELS
        )?;
        writeln!(&mut writer, "const TOTAL_DOTS: usize = {total_dots};")?;

        writeln!(
            &mut writer,
            "// Tile layers (0-based tile_id; 0xFF = empty)"
        )?;
        for (name, data) in tile_layers.iter() {
            let data_str = data.iter().map(|t| t.to_string()).join(", ");
            writeln!(
                &mut writer,
                "const {}: &[u8] = &[{data_str}];",
                name.to_uppercase()
            )?;
        }

        writeln!(&mut writer, "// Object layers")?;
        for (layer_name, objects) in object_layers.iter() {
            let by_type: HashMap<&str, Vec<(u8, u8)>> =
                objects
                    .iter()
                    .fold(HashMap::new(), |mut acc, &(obj_type, coords)| {
                        acc.entry(obj_type).or_default().push(coords);
                        acc
                    });
            for (obj_type, coords) in by_type.iter() {
                let xys = coords.iter().map(|(x, y)| format!("({x}, {y})")).join(", ");
                writeln!(
                    &mut writer,
                    "const {}_{}: &[(u8, u8)] = &[{xys}];",
                    layer_name.to_uppercase(),
                    obj_type.to_uppercase()
                )?;
            }
        }

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
        tilesets: Vec<TiledTilesetRef>,
    }

    #[derive(Deserialize)]
    struct TiledTilesetRef {
        firstgid: i32,
    }

    #[derive(Deserialize)]
    struct TiledLayer {
        name: String,
        data: Option<Vec<i32>>,
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
    struct TiledTileset {
        tiles: Vec<TiledTile>,
    }

    #[derive(Deserialize)]
    struct TiledTile {
        id: i32,
        #[serde(rename = "type")]
        tile_type: String,
    }
}
