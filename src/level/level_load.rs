use crate::parser::parse_level::WADLevel;

use super::level_elements::{Vertex, Sector, ExtSector};
use super::level_texture::MissingTextureTracker;

/* 
 * for behavior: first loadbehavior, then load default modules , then load the ACS lump
 * if strife also load the dialogues
 */


/* This function will load a single level (i.e. e1m1) and for normal doom map may 
 * need a translator to make it a udmf like map?
 * also needs to load the scripts */
pub fn load_level(level: &WADLevel) {
    
}

pub fn load_vertexes(level: &WADLevel) -> Vec<Vertex> {
    let mut vertexes: Vec<Vertex> = Vec::with_capacity(level.vertexes.len());

    for v in &level.vertexes {
        let vertex: Vertex = Vertex::new(v.x, v.y);
        vertexes.push(vertex);
    }
    vertexes
}

pub fn load_sectors(level: &WADLevel, missing_textures: &MissingTextureTracker) -> Vec<Sector> {
    let mut sectors: Vec<Sector> = Vec::with_capacity(level.sectors.len());
    let mut ext_sectors: Vec<ExtSector> = Vec::with_capacity(level.sectors.len());

    for i in 0..level.sectors.len() {
        ext_sectors.push(ExtSector::new());
        sectors.push(Sector::new(i as i32));
    }

    sectors
}