use crate::parser::*;

struct WADLevelBlockmap {
    x: i16,
    y: i16,
    num_cols: i16,
    num_rows: i16,
    blocklists: Vec<Vec<u16>>
}

struct WADLevelSector {
    floor_height: i16,
    ceiling_height: i16,
    floor_texture: String,
    ceiling_texture: String,
    light_level: i16,
    special: i16,
    tag: i16
}

struct WADLevelSubSector {
    num_segs: i16,
    start_seg: i16
}

struct WADLevelSeg {
    start: u16,
    end: u16,
    angle: i16,
    linedef: u16,
    direction: i16,
    offset: i16
}

struct WADLevelVertex {
    x: i16,
    y: i16
}

struct WADLevelSidedef {
    x_offset: i16,
    y_offset: i16,
    sector: u16,
    upper_texture: String,
    lower_texture: String,
    middle_texture: String,
}

struct WADLevelLinedef
{
  from: u16,
  to: u16,
  flags: u16,
  types: u16,
  tag: u16,
  right_sidedef: u16,
  left_sidedef: u16
}

struct WADLevelThing
{
  x: i16,
  y: i16,
  angle: i16,
  type_: i16,
  options: i16,
}

struct WADLevelNode {
    x_start: i16,
    y_start: i16,
    dx: i16,
    dy: i16,
    right_y_upper: i16,
    right_y_lower: i16,
    right_x_lower: i16,
    right_x_upper: i16,
    left_y_upper: i16,
    left_y_lower: i16,
    left_x_lower: i16,
    left_x_upper: i16,
    right_child: i16,
    left_child: i16
}



struct WADLevel {
    name: String,
    things: Vec<WADLevelThing>,
    linedefs: Vec<WADLevelLinedef>,
    sidedefs: Vec<WADLevelSidedef>,
    vertexes: Vec<WADLevelVertex>,
    segs: Vec<WADLevelSeg>,
    ssectors: Vec<WADLevelSubSector>,
    nodes: Vec<WADLevelNode>, //udmf stores the nodes in znodes
    sectors: Vec<WADLevelSector>,
    blockmap: WADLevelBlockmap,
    reject: Vec<Vec<bool>>
    // behavior, (HEXEN and udmf only)
    // znodes (udmf only)
}

#[derive(Debug, PartialEq)]
enum Format {
    UDMF,
    DOOM,
    HEXEN
}

pub fn get_map_lump(lump_name: String, wad_parsed: &WADData, wad_data: &Vec<u8>) -> Vec<u8> {
    let index = wad_parsed.lump_map.get(&lump_name).unwrap().to_owned() as usize;
    let wad_entry = &wad_parsed.directory[index];
    return wad_data[wad_entry.offset as usize..(wad_entry.offset as usize + wad_entry.size as usize)].to_vec();
}

fn parse_things(lump: &Vec<u8>) -> Vec<WADLevelThing> {
    let mut things: Vec<WADLevelThing> = vec![];

    let entry_len = 10;
    let len = lump.len() / entry_len;
    let mut offset: usize = 0;
    for _i in 0..len {
        let x = read_short(lump, &mut offset).unwrap();
        let y = read_short(lump, &mut offset).unwrap();
        let angle = read_short(lump, &mut offset).unwrap();
        let type_ = read_short(lump, &mut offset).unwrap();
        let options = read_short(lump, &mut offset).unwrap();
        let thing = WADLevelThing {
            x,
            y,
            angle,
            type_,
            options
        };
        things.push(thing);
    }
    things
}

fn parse_linedefs(lump: &Vec<u8>) -> Vec<WADLevelLinedef> {
    let mut linedefs: Vec<WADLevelLinedef> = vec![];

    let entry_len = 14;
    let len = lump.len() / entry_len;
    let mut offset: usize = 0;
    for _i in 0..len {
        let from = read_ushort(lump, &mut offset).unwrap();
        let to = read_ushort(lump, &mut offset).unwrap();
        let flags = read_ushort(lump, &mut offset).unwrap();
        let types = read_ushort(lump, &mut offset).unwrap();
        let tag = read_ushort(lump, &mut offset).unwrap();
        let right_sidedef = read_ushort(lump, &mut offset).unwrap();
        let left_sidedef = read_ushort(lump, &mut offset).unwrap();
        let linedef  = WADLevelLinedef {
            from,
            to,
            flags,
            types,
            tag,
            right_sidedef,
            left_sidedef
        };
        linedefs.push(linedef);
    }
    linedefs
}

fn parse_sidedefs(lump: &Vec<u8>) -> Vec<WADLevelSidedef> {
    let mut sidedefs: Vec<WADLevelSidedef> = vec![];

    let entry_len = 12;
    let len = lump.len() / entry_len;
    let mut offset: usize = 0;
    for _i in 0..len {
        let x_offset = read_short(lump, &mut offset).unwrap();
        let y_offset = read_short(lump, &mut offset).unwrap();
        let mut upper_texture: String = String::new();
        let mut lower_texture: String = String::new();
        let mut middle_texture: String = String::new();
        copy_and_capitalize_buffer(& mut upper_texture, lump, &mut offset, 8);
        copy_and_capitalize_buffer(& mut lower_texture, lump, &mut offset, 8);
        copy_and_capitalize_buffer(& mut middle_texture, lump, &mut offset, 8);
        let sector = read_ushort(lump, &mut offset).unwrap();
        let sidedef  = WADLevelSidedef {
            x_offset,
            y_offset,
            sector,
            upper_texture,
            lower_texture,
            middle_texture
        };
        sidedefs.push(sidedef);
    }
    sidedefs
}

fn parse_vertexes(lump: &Vec<u8>) -> Vec<WADLevelVertex> {
    let mut vertexes: Vec<WADLevelVertex> = vec![];

    let entry_len = 4;
    let len = lump.len() / entry_len;
    let mut offset: usize = 0;
    for _i in 0..len {
        let x = read_short(lump, &mut offset).unwrap();
        let y = read_short(lump, &mut offset).unwrap();
        let vertex  = WADLevelVertex {
            x,
            y
        };
        vertexes.push(vertex);
    }
    vertexes
}

fn parse_segs(lump: &Vec<u8>) -> Vec<WADLevelSeg> {
    let mut segs: Vec<WADLevelSeg> = vec![];

    let entry_len = 12;
    let len = lump.len() / entry_len;
    let mut offset: usize = 0;
    for _i in 0..len {
        let start = read_ushort(lump, &mut offset).unwrap();
        let end = read_ushort(lump, &mut offset).unwrap();
        let angle = read_short(lump, &mut offset).unwrap();
        let linedef = read_ushort(lump, &mut offset).unwrap();
        let direction = read_short(lump, &mut offset).unwrap();
        let offset = read_short(lump, &mut offset).unwrap();
        let seg  = WADLevelSeg {
            start,
            end,
            angle,
            linedef,
            direction,
            offset
        };
        segs.push(seg);
    }
    segs
}

fn parse_subsectors(lump: &Vec<u8>) -> Vec<WADLevelSubSector> {
    let mut ssectors: Vec<WADLevelSubSector> = vec![];

    let entry_len = 4;
    let len = lump.len() / entry_len;
    let mut offset: usize = 0;
    for _i in 0..len {
        let num_segs = read_short(lump, &mut offset).unwrap();
        let start_seg = read_short(lump, &mut offset).unwrap();
        let ssector  = WADLevelSubSector {
            num_segs,
            start_seg
        };
        ssectors.push(ssector);
    }
    ssectors
}

fn parse_nodes(lump: &Vec<u8>) -> Vec<WADLevelNode> {
    let mut nodes: Vec<WADLevelNode> = vec![];

    let entry_len = 28;
    let len = lump.len() / entry_len;
    let mut offset: usize = 0;
    for _i in 0..len {
        let x_start = read_short(lump, &mut offset).unwrap();
        let y_start = read_short(lump, &mut offset).unwrap();
        let dx = read_short(lump, &mut offset).unwrap();
        let dy = read_short(lump, &mut offset).unwrap();
        let right_y_upper = read_short(lump, &mut offset).unwrap();
        let right_y_lower = read_short(lump, &mut offset).unwrap();
        let right_x_lower = read_short(lump, &mut offset).unwrap();
        let right_x_upper = read_short(lump, &mut offset).unwrap();
        let left_y_upper = read_short(lump, &mut offset).unwrap();
        let left_y_lower = read_short(lump, &mut offset).unwrap();
        let left_x_lower = read_short(lump, &mut offset).unwrap();
        let left_x_upper = read_short(lump, &mut offset).unwrap();
        let right_child = read_short(lump, &mut offset).unwrap();
        let left_child = read_short(lump, &mut offset).unwrap();
        let node  = WADLevelNode {
            x_start,
            y_start,
            dx,
            dy,
            right_y_upper,
            right_y_lower,
            right_x_lower,
            right_x_upper,
            left_y_upper,
            left_y_lower,
            left_x_lower,
            left_x_upper,
            right_child,
            left_child
        };
        nodes.push(node);
    }
    nodes
}

fn parse_sectors(lump: &Vec<u8>) -> Vec<WADLevelSector> {
    let mut sectors: Vec<WADLevelSector> = vec![];

    let entry_len = 26;
    let len = lump.len() / entry_len;
    let mut offset: usize = 0;
    for _i in 0..len {
        let floor_height = read_short(lump, &mut offset).unwrap();
        let ceiling_height = read_short(lump, &mut offset).unwrap();
        let mut floor_texture: String = String::new();
        let mut ceiling_texture: String = String::new();
        copy_and_capitalize_buffer(& mut floor_texture, lump, &mut offset, 8);
        copy_and_capitalize_buffer(& mut ceiling_texture, lump, &mut offset, 8);
        let light_level = read_short(lump, &mut offset).unwrap();
        let special = read_short(lump, &mut offset).unwrap();
        let tag = read_short(lump, &mut offset).unwrap();
        let sector  = WADLevelSector {
            floor_height,
            ceiling_height,
            floor_texture,
            ceiling_texture,
            light_level,
            special,
            tag
        };
        sectors.push(sector);
    }
    sectors
}

// Function that reads the mapdata and checks if it is a udmf, doom or hexen map format
pub fn read_levels(wad_data: &Vec<u8>, wad_parsed: &mut WADData, index: usize) {
    let format: Format;
    if wad_parsed.directory[index + 1].name == "TEXTMAP" {
        format = Format::UDMF;
    }
    else {
        let map_lumps: Vec<&str> = vec!["THINGS","LINEDEFS","SIDEDEFS","VERTEXES","SEGS","TEXTMAP",
                "SSECTORS","NODES","SECTORS","REJECT","BLOCKMAP","BEHAVIOR","ZNODES"];
        let mut pos = 1;
        let mut mapdatalumps: Vec<&String> = vec![];
        let mut next_lump = &wad_parsed.directory[index + pos].name;
        while map_lumps.contains(&next_lump.as_str()) {
            mapdatalumps.push(next_lump);
            pos += 1;
            if wad_parsed.directory.len() == pos + index {break;}
            next_lump = &wad_parsed.directory[index + pos].name;
        }

        if mapdatalumps.contains(&&"BEHAVIOR".to_owned()) {format = Format::HEXEN;}
        else {format = Format::DOOM;}
    }

    let name = &wad_parsed.directory[index].name;
    if format == Format::DOOM {
        let things = parse_things(&get_map_lump("THINGS".to_owned(), wad_parsed, wad_data));
        let linedefs = parse_linedefs(&get_map_lump("LINEDEFS".to_owned(), wad_parsed, wad_data));
        let sidedefs = parse_sidedefs(&get_map_lump("SIDEDEFS".to_owned(), wad_parsed, wad_data));
        let vertexes = parse_vertexes(&get_map_lump("VERTEXES".to_owned(), wad_parsed, wad_data));
        let segs = parse_segs(&get_map_lump("SEGS".to_owned(), wad_parsed, wad_data));
        let ssectors = parse_subsectors(&get_map_lump("SSECTORS".to_owned(), wad_parsed, wad_data));
        let nodes = parse_nodes(&get_map_lump("NODES".to_owned(), wad_parsed, wad_data));
        let sectors = parse_sectors(&get_map_lump("SECTORS".to_owned(), wad_parsed, wad_data));
        //reject
        //blockmap
    }
    if format == Format::HEXEN {

    }

    
    // let level = WADLevel {
    //     name,
    //     things,
    //     linedefs,
    //     sidedefs,
    //     vertexes,
    //     segs,
    //     ssectors,
    //     nodes,
    //     sectors,
    //     blockmap,
    //     reject
    // };
}