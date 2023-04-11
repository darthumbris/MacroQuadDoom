use std::array::TryFromSliceError;
use std::fs;
use std::collections::HashMap;

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
    Name
}

fn read_short(wad_data: &Vec<u8>, offset: &mut usize) -> Result<i16, TryFromSliceError> {
    let tmp_ = i16::from_le_bytes(wad_data[*offset..*offset+2].try_into()?);
    *offset += 2;
    Ok(tmp_)
}

fn read_ushort(wad_data: &Vec<u8>, offset: &mut usize) -> Result<u16, TryFromSliceError> {
    let tmp_ = u16::from_le_bytes(wad_data[*offset..*offset+2].try_into()?);
    *offset += 2;
    Ok(tmp_)
}

fn read_int(wad_data: &Vec<u8>, offset: &mut usize) -> Result<i32, TryFromSliceError> {
    let tmp_ = i32::from_le_bytes(wad_data[*offset..*offset+4].try_into()?);
    *offset += 4;
    Ok(tmp_)
}

fn read_uint(wad_data: &Vec<u8>, offset: &mut usize) -> Result<u32, TryFromSliceError> {
    let tmp_ = u32::from_le_bytes(wad_data[*offset..*offset+4].try_into()?);
    *offset += 4;
    Ok(tmp_)
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
	wad_header.lump_count = read_uint(wad_data, offset).unwrap();
	wad_header.directory_offset = read_uint(wad_data, offset).unwrap();
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
        wad_entry.offset = read_uint(wad_data, offset).unwrap();
        wad_entry.size = read_uint(wad_data, offset).unwrap();
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

fn headerCheck(data: &Vec<u8>, header: &str, offset: usize) ->bool {
    for (i, c) in header.bytes().enumerate() {
        if c != data[offset + i] {return false;}
    }
    return true;
}

fn detect_lump_type(wad_parsed: &WADData, index: usize, data: &Vec<u8>) -> LumpTypes{
    let map_lumps: [String; 13] = ["THINGS".to_owned(),"LINEDEFS".to_owned(),"SIDEDEFS".to_owned(),"VERTEXES".to_owned(),"SEGS".to_owned(),"TEXTMAP".to_owned(),
                "SSECTORS".to_owned(),"NODES".to_owned(),"SECTORS".to_owned(),"REJECT".to_owned(),"BLOCKMAP".to_owned(),"BEHAVIOR".to_owned(),"ZNODES".to_owned()];
    let text_lumps: [String; 17] = [ "DEHACKED".to_owned(), "MAPINFO".to_owned(), "ZMAPINFO".to_owned(), "EMAPINFO".to_owned(), 
                "DMXGUS".to_owned(), "DMXGUSC".to_owned(), "WADINFO".to_owned(), "EMENUS".to_owned(), "MUSINFO".to_owned(),
                "SNDINFO".to_owned(), "GLDEFS".to_owned(), "KEYCONF".to_owned(), "SCRIPTS".to_owned(), "LANGUAGE".to_owned(),
                "DECORATE".to_owned(), "SBARINFO".to_owned(), "MENUDEF".to_owned() ];
    let data_lumps = [ "PLAYPAL".to_owned(), "COLORMAP".to_owned(), "TEXTURE1".to_owned(), "TEXTURE2".to_owned(), "PNAMES".to_owned(),
                  "ENDOOM".to_owned()];

    //Data based lump detection
    if wad_parsed.directory[index].size != 0 {
        let offset = wad_parsed.directory[index].offset as usize;
        if headerCheck(data, "MThd", offset) {
            return LumpTypes::MIDI;
        }    
        if headerCheck(data, "ID3", offset) {
            return LumpTypes::MP3;
        }
        if headerCheck(data, "MUS", offset) {
            return LumpTypes::MUS;
        }
        if headerCheck(data, "PNG", offset) {
            return LumpTypes::PNG;
        }
    }

    //Name based detection
    let name = wad_parsed.directory[index].name.clone();
    if text_lumps.contains(&name) {return LumpTypes::Text;}
    if map_lumps.contains(&name) {return LumpTypes::MapData;}
    if data_lumps.contains(&name) {return LumpTypes::Name;}
    // if /^MAP\d\d/.test(name) {return LumpTypes::Map;}
    // if /^E\dM\d/.test(name) {return LumpTypes::Map;}
    // if /_START$/.test(name) {return LumpTypes::Marker;}
    // if /_END$/.test(name) {return LumpTypes::Marker;}

    if wad_parsed.directory[index].size == 0 {return LumpTypes::Marker;}

    return LumpTypes::Graphic
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