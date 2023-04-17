use std::array::TryFromSliceError;
use std::fs;
use std::collections::HashMap;
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
    color_maps: Vec<Vec<u8>>,
    sprites: Vec<(String, WADSprite)>
}


fn read_header(wad_data: &Vec<u8>, offset: &mut usize) -> WADHeader {
    let mut wad_header = WADHeader{
        map_type: String::from(""),
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
            name: String::from(""),
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

        for _i in 0 .. 256 {
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

        for _i in 0 .. 256 {
            colormap_.push(wad_data[*offset]);
            *offset += 1;
        }

        wad_parsed.color_maps.push(colormap_);
    }
    println!("color_maps: {}", wad_parsed.color_maps.len());
}

fn read_sprites(wad_data: &Vec<u8>, wad_parsed: &mut WADData, index: usize) {
    let sprite_lump = &wad_parsed.directory[index];
    let mut offset = sprite_lump.offset as usize;

    let mut sprite: WADSprite = WADSprite { width: 0, height: 0, left_offset: 0, top_offset: 0, posts: vec![] };

    sprite.width = u32::from(read_ushort(wad_data, &mut offset).unwrap());
    sprite.height = u32::from(read_ushort(wad_data, &mut offset).unwrap());
    sprite.left_offset = u32::from(read_ushort(wad_data, &mut offset).unwrap());
    sprite.top_offset = u32::from(read_ushort(wad_data, &mut offset).unwrap());

    let mut col_offsets = vec![];

    for _i in 0..sprite.width {
        col_offsets.push(read_uint(wad_data, &mut offset).unwrap());
    }

    for i in 0..sprite.width {
        offset = sprite_lump.offset as usize + col_offsets[i as usize] as usize;

        while wad_data[offset] != 0xff {
            let mut post: WADSpritePost = WADSpritePost { col: 0, row: 0, size: 0, pixels: vec![] };
            post.col = i as u8;
            post.row = wad_data[offset];
            post.size = wad_data[offset + 1];
            offset += 3; // 2 + 1 to skip the first unused byte

            for _k in 0..post.size {
                post.pixels.push(wad_data[offset]);
                offset += 1;
            }
            offset += 1; // to skip the last unused byte
            sprite.posts.push(post);
        }
    }
    println!("Adding sprite :{}", sprite_lump.name);
    wad_parsed.sprites.push((sprite_lump.name.to_string(), sprite));
    
}

fn header_check(data: &Vec<u8>, header: &str, offset: usize) ->bool {
    for (i, c) in header.bytes().enumerate() {
        if c != data[offset + i] {return false;}
    }
    return true;
}

fn is_doom_gfx(dv: &Vec<u8>,lump: WADEntry, offset: usize) ->bool {
    // first check the dimensions aren't ridiculous
    let mut temp_offset = offset;
    if read_ushort(dv, &mut temp_offset).unwrap() > 4096 {return false}
    if read_ushort(dv, &mut temp_offset).unwrap() > 4096 {return false}

    if read_short(dv, &mut temp_offset).unwrap().abs() > 2000 {return false}
    if read_short(dv, &mut temp_offset).unwrap().abs() > 2000 {return false}

    // then check it ends in 0xFF
    if dv[lump.size as usize - 1] != 0xFF {
        // sometimes the graphics have up to 3 garbage 0x00 bytes at the end
        let mut found = false;
        for b in 1..4 {
            if found == false {
                if dv[lump.size as usize - b] == 0xFF {
                    found = true;
                } else if dv[lump.size as usize - b] != 0x00 {
                    return false;
                }
            }
        }
        if found == false {return false;}
    }
    // I think this is enough for now. If I get false positives, I'll look into more comprehensive checks.
    true
}

fn data_lump(name: &str) -> LumpTypes {
    match name {
        "PLAYPAL" => return LumpTypes::PlayPal,
        "COLORMAP" => return LumpTypes::ColorMap,
        "TEXTURE1" => return  LumpTypes::Texture1,
        "TEXTURE2" => return  LumpTypes::Texture2,
        "PNAMES" => return LumpTypes::PNames,
        "ENDOOM" => return LumpTypes::EndDoom,
        _ => return LumpTypes::ERROR
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
        if header_check(data, "MThd", offset) {
            return LumpTypes::MIDI;
        }    
        if header_check(data, "ID3", offset) {
            return LumpTypes::MP3;
        }
        if header_check(data, "MUS", offset) {
            return LumpTypes::MUS;
        }
        if header_check(data, "PNG", offset) {
            return LumpTypes::PNG;
        }
    }
    
    //Name based detection
    let name = wad_parsed.directory[index].name.clone();
    if text_lumps.contains(&name.as_str()) {return LumpTypes::Text;}
    if map_lumps.contains(&name.as_str()) {return LumpTypes::MapData;}
    if data_lumps.contains(&name.as_str()) {return data_lump(&name.as_str())}
    if name.starts_with("MAP") && name.len() > 3 && name.chars().nth(3).unwrap() >= '0' && name.chars().nth(3).unwrap() <= '9' {return LumpTypes::Map;}
    if name.starts_with("E") && name.len() > 3 && name.chars().nth(1).unwrap() >= '0' && name.chars().nth(1).unwrap() <= '9' && name.chars().nth(2).unwrap() == 'M' &&
        name.chars().nth(3).unwrap() >= '0' && name.chars().nth(3).unwrap() <= '9' {return LumpTypes::Map;}
    if name.ends_with("_START") {return LumpTypes::Marker;}
    if name.ends_with("_END") {return LumpTypes::Marker;}

    if wad_parsed.directory[index].size == 0 {return LumpTypes::Marker;}

    //between markers
    for i in (0..index).rev() {
        if wad_parsed.directory[i].name.ends_with("_END") {break;}
        if wad_parsed.directory[i].name.ends_with("_START") {
            let pre = wad_parsed.directory[i].name.trim_end_matches("START");
            if graphic_markers.contains(&pre) {return  LumpTypes::Graphic;}
            if flat_markers.contains(&pre) {return  LumpTypes::Flat;}
        }
    }

    //shitty name-based detection
    if name.starts_with("D_") {return LumpTypes::Music;}

    if is_doom_gfx(data, wad_parsed.directory[index].clone(), wad_parsed.directory[index].offset as usize) {
        return LumpTypes::Graphic
    }
    return LumpTypes::ERROR;
}

fn read_data_lumps(wad_data: &Vec<u8>, wad_parsed: &mut WADData) {
    for i in 0..wad_parsed.directory.len() {
        if detect_lump_type(wad_parsed, i, wad_data) == LumpTypes::Graphic {
            read_sprites(wad_data, wad_parsed, i);
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
        sprites: vec![]
    };

    read_directory(&map, &mut offset, &mut wad_parsed);
    read_pallete(&map, &mut offset, &mut wad_parsed);
    read_colormap(&map, &mut offset, &mut wad_parsed);
    read_data_lumps(&map, &mut wad_parsed);
    println!("lump[4]: {}", wad_parsed.directory[1267].name);
    println!("lump type: {:?}", detect_lump_type(&wad_parsed, 1267, &map));
    println!("\ndone parsing!");
}