use std::fs;
use std::collections::HashMap;

fn read_short(wad_data: &Vec<u8>, offset: &mut usize) -> i16 {
    let tmp_: i16 = ((wad_data[*offset + 1] << 8) | wad_data[*offset]) as i16;
    *offset += 2;
	tmp_
}

fn read_ushort(wad_data: &Vec<u8>, offset: &mut usize) -> u16 {
    let tmp_: u16 = ((wad_data[*offset + 1] << 8) | wad_data[*offset]) as u16;
    *offset += 2;
	tmp_
}

fn read_int(wad_data: &Vec<u8>, offset: &mut usize) -> i32 {
    let tmp_ = i32::from(wad_data[*offset + 3]) << 24 | 
        i32::from(wad_data[*offset + 2]) << 16 | 
        i32::from(wad_data[*offset + 1]) << 8 | 
        i32::from(wad_data[*offset]);
    *offset += 4;
    tmp_
}

fn read_uint(wad_data: &Vec<u8>, offset: &mut usize) -> u32 {
    let tmp_ = u32::from(wad_data[*offset + 3]) << 24 | 
        u32::from(wad_data[*offset + 2]) << 16 | 
        u32::from(wad_data[*offset + 1]) << 8 | 
        u32::from(wad_data[*offset]);
    *offset += 4;
    tmp_
}

fn copy_and_capitalize_buffer(
	r_dst: &mut String,
	wad_data: &Vec<u8>, offset: &mut usize,
	src_length: u32)
{
    let mut owned_string: String = "".to_owned();
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

struct WADEntry {
    offset: u32, //offset to start of lump
    size: u32, // size of the lump
    name: String // name of lump 8 char long
}
struct WADPaletteColor
{
    r: u8,
    g: u8,
    b: u8
}

struct WADSprite {
    width: u32,
    height: u32,
    left_offset: u32,
    top_offset: u32,
    posts: Vec<WADSpritePost>
}

struct WADSpritePost
{
    col: u8,
    row: u8,
    size: u8,
    pixels: Vec<u8>
}

struct WADData {
    directory: Vec<WADEntry>,
    lump_map: HashMap<String, u32>,
    wad_header: WADHeader,
    palletes: Vec<Vec<WADPaletteColor>>,
    color_maps: Vec<Vec<u8>>
}


fn read_header(wad_data: &Vec<u8>, offset: &mut usize) -> WADHeader {
    let mut wad_header = WADHeader{
        map_type: "".to_owned(),
        lump_count: 0,
        directory_offset: 0
    };
    copy_and_capitalize_buffer(&mut wad_header.map_type, wad_data, offset, 4);
	wad_header.lump_count = read_uint(wad_data, offset);
	wad_header.directory_offset = read_uint(wad_data, offset);
    println!("type: {}", wad_header.map_type);
    println!("numlumps: {}", wad_header.lump_count);
    println!("infotableofs: {}", wad_header.directory_offset);
    wad_header
}

fn read_directory(wad_data: &Vec<u8>, offset: &mut usize, wad_parsed: &mut WADData) {
    for i in 0 .. wad_parsed.wad_header.lump_count {
        let mut wad_entry = WADEntry{
            name: "".to_owned(),
            size: 0,
            offset: 0
        };
        wad_entry.offset = read_uint(wad_data, offset);
        wad_entry.size = read_uint(wad_data, offset);
        copy_and_capitalize_buffer(&mut wad_entry.name, wad_data, offset, 8);
        println!("offset: {}", wad_entry.offset);
        println!("size: {}", wad_entry.size);
        println!("name: {}", wad_entry.name);
        wad_parsed.lump_map.insert(wad_entry.name.clone(), i);
        wad_parsed.directory.push(wad_entry);
    }
}

fn read_pallete(wad_data: &Vec<u8>, offset: &mut usize, wad_parsed: &mut WADData) {

    let index = wad_parsed.lump_map.get("PLAYPAL").unwrap().clone() as usize;
    let pallete_entry = wad_parsed.directory.get(index).unwrap();

    *offset = pallete_entry.offset as usize;
    while *offset < pallete_entry.offset  as usize + pallete_entry.size as usize {
        let mut palette_: Vec<WADPaletteColor> = Vec::with_capacity(256);

        for i in 0 .. 256 {
            let color_: WADPaletteColor = WADPaletteColor { r: wad_data[*offset], g: wad_data[*offset + 1], b: wad_data[*offset + 2] };
            palette_.push(color_);
            *offset += 3;
        }

        wad_parsed.palletes.push(palette_);
    }
    println!("palletes: {}", wad_parsed.palletes.len());
}

fn read_colormap(wad_data: &Vec<u8>, offset: &mut usize, wad_parsed: &mut WADData) {
    let index = wad_parsed.lump_map.get("COLORMAP").unwrap().clone() as usize;
    let color_map = wad_parsed.directory.get(index).unwrap();

    *offset = color_map.offset as usize;
    while *offset < color_map.offset  as usize + color_map.size as usize {
        let mut colormap_: Vec<u8> = Vec::with_capacity(256);

        for i in 0 .. 256 {
            colormap_.push(wad_data[*offset]);
            *offset += 1;
        }

        wad_parsed.color_maps.push(colormap_);
    }
    println!("color_maps: {}", wad_parsed.color_maps.len());
}

fn read_sprites(wad_data: &Vec<u8>, offset: &mut usize, wad_parsed: &mut WADData) {
    
}

fn detect_lump_type() {
    
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
        color_maps: vec![]
    };

    read_directory(&map, &mut offset, &mut wad_parsed);
    read_pallete(&map, &mut offset, &mut wad_parsed);
    read_colormap(&map, &mut offset, &mut wad_parsed);
    println!("\ndone parsing!");
}