
mod parse_graphics;
mod parse_level;

use parse_graphics::*;
use std::array::TryFromSliceError;
use std::fs;
use std::collections::HashMap;
use parse_level::*;


#[derive(Debug, PartialEq)]
enum LumpTypes {
    Text,
    Map,
    MapData,
    Music,
    MIDI,
    MP3,
    PNG,
    MUS,
    Graphic,
    Flat,
    Marker,
    PlayPal,
    ColorMap,
    EndDoom,
    PNames,
    Texture1,
    Texture2,
    ERROR
}

pub fn read_short(wad_data: &Vec<u8>, offset: &mut usize) -> Result<i16, TryFromSliceError> {
    let tmp_ = i16::from_le_bytes(wad_data[*offset..*offset+2].try_into()?);
    *offset += 2;
    Ok(tmp_)
}

pub fn read_ushort(wad_data: &Vec<u8>, offset: &mut usize) -> Result<u16, TryFromSliceError> {
    let tmp_ = u16::from_le_bytes(wad_data[*offset..*offset+2].try_into()?);
    *offset += 2;
    Ok(tmp_)
}

pub fn read_int(wad_data: &Vec<u8>, offset: &mut usize) -> Result<i32, TryFromSliceError> {
    let tmp_ = i32::from_le_bytes(wad_data[*offset..*offset+4].try_into()?);
    *offset += 4;
    Ok(tmp_)
}

pub fn read_uint(wad_data: &Vec<u8>, offset: &mut usize) -> Result<u32, TryFromSliceError> {
    let tmp_ = u32::from_le_bytes(wad_data[*offset..*offset+4].try_into()?);
    *offset += 4;
    Ok(tmp_)
}

pub fn read_u8(wad_data: &Vec<u8>, offset: &mut usize) -> Result<u8, TryFromSliceError> {
    let tmp_ = u8::from_le_bytes(wad_data[*offset..*offset + 1].try_into()?);
    *offset += 1;
    Ok(tmp_)
}

pub fn copy_and_capitalize_buffer(
	r_dst: &mut String,
	wad_data: &Vec<u8>, offset: &mut usize,
	src_length: u32)
{
    let mut owned_string: String = String::from("");
    let mut i: usize = 0;
	while i < src_length as usize && wad_data[*offset + i] != 0 {
        owned_string.push((wad_data[*offset + i]).to_ascii_uppercase() as char);
        i += 1;
    }

    *r_dst = owned_string;
    *offset += src_length as usize;
}

struct WADHeader {
    map_type: String,
    lump_count: u32,
    directory_offset: u32
}

#[derive(Clone)]
pub struct WADEntry {
    offset: u32, //offset to start of lump
    size: u32, // size of the lump
    name: String // name of lump 8 char long
}

pub struct WADData {
    directory: Vec<WADEntry>,
    lump_map: HashMap<String, u32>,
    wad_header: WADHeader,
    palletes: Vec<Vec<WADPaletteColor>>,
    color_maps: Vec<Vec<u8>>,
    sprites: Vec<(String, WADSprite)>,
    flats: Vec<u8>,
    levels: Vec<WADLevel>
}

fn read_header(wad_data: &Vec<u8>, offset: &mut usize) -> WADHeader {
    let mut map_type = String::new();
    copy_and_capitalize_buffer(&mut map_type, wad_data, offset, 4);
	let lump_count = read_uint(wad_data, offset).unwrap();
	let directory_offset = read_uint(wad_data, offset).unwrap();
    WADHeader{map_type, lump_count, directory_offset}
}

fn read_directory(wad_data: &Vec<u8>, offset_: &mut usize, wad_parsed: &mut WADData) {
    for i in 0 .. wad_parsed.wad_header.lump_count {
        let mut name: String = String::new();
        let offset = read_uint(wad_data, offset_).unwrap();
        let size = read_uint(wad_data, offset_).unwrap();
        copy_and_capitalize_buffer(&mut name, wad_data, offset_, 8);
        wad_parsed.lump_map.insert(name.clone(), i);
        wad_parsed.directory.push(WADEntry { offset, size, name });
    }
}

fn header_check(data: &Vec<u8>, header: &str, offset: usize) ->bool {
    for (i, c) in header.bytes().enumerate() {
        if c != data[offset + i] {return false}
    }
    true
}

fn data_lump(name: &str) -> LumpTypes {
    match name {
        "PLAYPAL"   => return LumpTypes::PlayPal,
        "COLORMAP"  => return LumpTypes::ColorMap,
        "TEXTURE1"  => return  LumpTypes::Texture1,
        "TEXTURE2"  => return  LumpTypes::Texture2,
        "PNAMES"    => return LumpTypes::PNames,
        "ENDOOM"    => return LumpTypes::EndDoom,
        _           => return LumpTypes::ERROR
    }
}

fn detect_lump_type(wad_parsed: &WADData, index: usize, data: &Vec<u8>) -> LumpTypes{
    let map_lumps: Vec<&str> = vec!["THINGS","LINEDEFS","SIDEDEFS","VERTEXES","SEGS","TEXTMAP",
                "SSECTORS","NODES","SECTORS","REJECT","BLOCKMAP","BEHAVIOR","ZNODES"];
    let text_lumps: Vec<&str> = vec![ "DEHACKED", "MAPINFO", "ZMAPINFO", "EMAPINFO", 
                "DMXGUS", "DMXGUSC", "WADINFO", "EMENUS", "MUSINFO",
                "SNDINFO", "GLDEFS", "KEYCONF", "SCRIPTS", "LANGUAGE",
                "DECORATE", "SBARINFO", "MENUDEF" ];
    let data_lumps = vec![ "PLAYPAL", "COLORMAP", "TEXTURE1", "TEXTURE2", "PNAMES",
                  "ENDOOM"];
    let graphic_markers = vec!["P_","PP_","P1_","P2_","P3_","S_","S2_","S3_","SS_"];
    let flat_markers = vec!["F_","FF_","F1_","F2_","F3_"];
    
    //Data based lump detection
    if wad_parsed.directory[index].size != 0 {
        let offset = wad_parsed.directory[index].offset as usize;
        if header_check(data, "MThd", offset) {return LumpTypes::MIDI;}
        if header_check(data, "ID3", offset) {return LumpTypes::MP3;}
        if header_check(data, "MUS", offset) {return LumpTypes::MUS;}
        if header_check(data, "PNG", offset) {return LumpTypes::PNG;}
    }
    
    //Name based detection
    let name = wad_parsed.directory[index].name.clone();
    if text_lumps.contains(&name.as_str()) {return LumpTypes::Text}
    if map_lumps.contains(&name.as_str()) {return LumpTypes::MapData}
    if data_lumps.contains(&name.as_str()) {return data_lump(&name.as_str())}
    if name.starts_with("MAP") && name.len() > 3 && name.chars().nth(3).unwrap() >= '0' 
        && name.chars().nth(3).unwrap() <= '9' {return LumpTypes::Map}
    if name.starts_with("E") && name.len() > 3 && name.chars().nth(1).unwrap() >= '0' 
        && name.chars().nth(1).unwrap() <= '9' && name.chars().nth(2).unwrap() == 'M' &&
        name.chars().nth(3).unwrap() >= '0' && name.chars().nth(3).unwrap() <= '9' {return LumpTypes::Map}
    if name.ends_with("_START") {return LumpTypes::Marker}
    if name.ends_with("_END") {return LumpTypes::Marker}

    if wad_parsed.directory[index].size == 0 {return LumpTypes::Marker}

    //between markers
    for i in (0..index).rev() {
        if wad_parsed.directory[i].name.ends_with("_END") {break}
        if wad_parsed.directory[i].name.ends_with("_START") {
            let pre = wad_parsed.directory[i].name.trim_end_matches("START");
            if graphic_markers.contains(&pre) {return LumpTypes::Graphic}
            if flat_markers.contains(&pre) {return LumpTypes::Flat}
        }
    }

    //shitty name-based detection
    if name.starts_with("D_") {return LumpTypes::Music;}

    if is_doom_gfx(data, wad_parsed.directory[index].clone(), wad_parsed.directory[index].offset as usize) {
        return LumpTypes::Graphic
    }
    println!("lump: {} failed to get a lumptype", name);
    LumpTypes::ERROR
}

fn read_data_lumps(wad_data: &Vec<u8>, wad_parsed: &mut WADData) {
    for i in 0..wad_parsed.directory.len() {
        match detect_lump_type(wad_parsed, i, wad_data) {
            LumpTypes::Graphic => {read_sprites(wad_data, wad_parsed, i);}
            LumpTypes::Flat => {read_flats(wad_data, wad_parsed, i);}
            LumpTypes::Map => {read_levels(wad_data, wad_parsed, i);}
            _o => {/* println!("not implemented {:?} yet", o)*/}
        }
    }
}


pub fn parse_map(path: &str) {
    println!("parsing wad file");
    let map = fs::read(path).unwrap();

    let mut offset: usize = 0;
    let wad_header: WADHeader = read_header(&map, &mut offset);
    offset = wad_header.directory_offset as usize;
    let mut wad_parsed = WADData {
        directory: vec![],
        lump_map: HashMap::new(),
        wad_header: wad_header,
        palletes: vec![],
        color_maps: vec![],
        sprites: vec![],
        flats: vec![],
        levels: vec![]
    };

    read_directory(&map, &mut offset, &mut wad_parsed);
    read_pallete(&map, &mut offset, &mut wad_parsed);
    read_colormap(&map, &mut offset, &mut wad_parsed);
    read_data_lumps(&map, &mut wad_parsed);
    println!("\ndone parsing!");
}