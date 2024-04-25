mod tilemaps {

   pub mod tilemap {
        include!(concat!(env!("OUT_DIR"), "/tilemap.rs"));
   }

   pub mod level_1 {
        include!(concat!(env!("OUT_DIR"), "/level-1.json.rs"));
   }

}
