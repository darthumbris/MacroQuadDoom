use bitreader::BitReader;

use crate::parser::*;
use crate::behavior::*;

#[derive(Debug)]
#[repr(u16)]
enum Lumps {
    THINGS = 1,
    LINEDEFS = 2,
    SIDEDEFS = 3,
    VERTEXES = 4,
    SEGS= 5,
    SSECTORS = 6,
    NODES = 7,
    SECTORS = 8,
    REJECT = 9,
    BLOCKMAP = 10,
    BEHAVIOR = 11
}

pub struct WADLevelBlockmap {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub blockmap_lump: Vec<i32>
    // pub blocklists: Vec<Vec<u16>>
}

pub struct WADLevelSector {
    pub floor_height: i16,
    pub ceiling_height: i16,
    pub floor_texture: String,
    pub ceiling_texture: String,
    pub light_level: i16,
    pub special: i16,
    pub tag: i16
}

pub struct WADLevelSubSector {
    pub num_segs: i16,
    pub start_seg: i16
}

pub struct WADLevelSeg {
    pub start: u16, //starting vertex
    pub end: u16, //ending vertex
    pub angle: i16,
    pub linedef: u16, //index of the linedef
    pub direction: i16, //0: same as linedef, 1: opposite of linedef
    pub offset: i16
}

#[derive(Clone, Copy)]
pub struct WADLevelVertex {
    pub x: i16,
    pub y: i16
}

pub struct WADLevelSidedef {
    pub x_offset: i16,
    pub y_offset: i16,
    pub sector: u16,
    pub upper_texture: String,
    pub lower_texture: String,
    pub middle_texture: String,
}

#[derive(Clone)]
pub struct WADLevelHexenLinedef {
    pub type_: u8,
    pub arg1: u8,
    pub arg2: u8,
    pub arg3: u8,
    pub arg4: u8,
    pub arg5: u8
}

#[derive(Clone)]
pub struct WADLevelDoomLinedef {
    pub types: u16,
    pub tag: u16,
}

#[derive(Clone)]
pub struct WADLevelLinedef
{
  pub from: u16,
  pub to: u16,
  pub flags: u16,
  pub doom: Option<WADLevelDoomLinedef>,
  pub hex: Option<WADLevelHexenLinedef>,
  pub front_sidedef: u16,
  pub back_sidedef: u16
}

pub struct WADLevelHexenThing {
    pub thing_id: i16,
    pub z: i16,
    pub action_special: u8,
    pub arg1: u8,
    pub arg2: u8,
    pub arg3: u8,
    pub arg4: u8,
    pub arg5: u8
}

pub struct WADLevelThing
{
  pub x: i16,
  pub y: i16,
  pub angle: i16,
  pub type_: i16,
  pub options: i16,
  pub hex: Option<WADLevelHexenThing>
}

pub struct WADLevelNode {
    pub x_start: i16,
    pub y_start: i16,
    pub dx: i16,
    pub dy: i16,
    pub right_y_upper: i16,
    pub right_y_lower: i16,
    pub right_x_lower: i16,
    pub right_x_upper: i16,
    pub left_y_upper: i16,
    pub left_y_lower: i16,
    pub left_x_lower: i16,
    pub left_x_upper: i16,
    pub right_child: u16,
    pub left_child: u16
}

impl WADLevelNode {
    pub fn get_bbox(&self, j: usize, k: usize) -> i16 {
        match j {
            0 => {
                match k {
                    0 => {return self.right_y_upper}
                    1 => {return self.right_y_lower}
                    2 => {return self.right_x_lower}
                    3 => {return self.right_x_upper}
                    _ => {eprintln!("Should not happen");return -1}
                }
            }
            1 => {
                match k {
                    0 => {return self.left_y_upper}
                    1 => {return self.left_y_lower}
                    2 => {return self.left_x_lower}
                    3 => {return self.left_x_upper}
                    _ => {eprintln!("Should not happen"); return -1}
                }
            }
            _ => {eprintln!("Should not happen");return -1}
        }
    }
}

//TODO this should be in a udmf format 
pub struct WADLevel {
    pub name: String,
    pub things: Vec<WADLevelThing>,
    pub linedefs: Vec<WADLevelLinedef>,
    pub sidedefs: Vec<WADLevelSidedef>,
    pub vertexes: Vec<WADLevelVertex>,
    pub segs: Vec<WADLevelSeg>,
    pub ssectors: Vec<WADLevelSubSector>,
    pub nodes: Vec<WADLevelNode>, //udmf stores the nodes in znodes
    pub sectors: Vec<WADLevelSector>,
    pub blockmap: WADLevelBlockmap, //rawdata,
    pub reject: Vec<Vec<bool>>,
    pub format: Format,
    pub has_behavior: bool,
    pub is_text: bool,
    pub behavior: Option<WADLevelBehavior>,
    // behavior, (HEXEN and udmf only)
    pub znodes: Vec<Znode>, //TODO parse znodes
    pub gl_znodes: Vec<GlZnode> //TODO parse glznodes
    // znodes (udmf only)
}

pub struct Znode {
    //TODO
}

pub struct GlZnode {
    //TODO
}

#[derive(Debug, PartialEq)]
pub enum Format {
    UDMF,
    DOOM,
    HEXEN
}

pub fn get_lump_from_dir(index: usize, wad_parsed: &WADData, wad_data: &Vec<u8>) -> Vec<u8> {
    let wad_entry = &wad_parsed.directory[index];
    wad_data[wad_entry.offset as usize..(wad_entry.offset as usize + wad_entry.size as usize)].to_vec()
}

fn parse_hexen_things(lump: &Vec<u8>) -> Vec<WADLevelThing> {
    let mut things: Vec<WADLevelThing> = vec![];

    let entry_len = 20;
    let len = lump.len() / entry_len;
    let mut offset: usize = 0;
    for _i in 0..len {
        let thing_id = read_short(lump, &mut offset).unwrap();
        let x = read_short(lump, &mut offset).unwrap();
        let y = read_short(lump, &mut offset).unwrap();
        let z = read_short(lump, &mut offset).unwrap();
        let angle = read_short(lump, &mut offset).unwrap();
        let type_ = read_short(lump, &mut offset).unwrap();
        let options = read_short(lump, &mut offset).unwrap();
        let action_special = read_u8(lump, &mut offset).unwrap();
        let arg1 = read_u8(lump, &mut offset).unwrap();
        let arg2 = read_u8(lump, &mut offset).unwrap();
        let arg3 = read_u8(lump, &mut offset).unwrap();
        let arg4 = read_u8(lump, &mut offset).unwrap();
        let arg5 = read_u8(lump, &mut offset).unwrap();

        let thing = WADLevelThing {
            x,
            y,
            angle,
            type_,
            options,
            hex: Some(WADLevelHexenThing {
                thing_id,
                z,
                action_special,
                arg1,
                arg2,
                arg3,
                arg4,
                arg5
            })
        };
        things.push(thing);
    }
    things
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
            options,
            hex: None
        };
        things.push(thing);
    }
    things
}

fn parse_hexen_linedefs(lump: &Vec<u8>) -> Vec<WADLevelLinedef> {
    let mut linedefs: Vec<WADLevelLinedef> = vec![];

    let entry_len = 16;
    let len = lump.len() / entry_len;
    let mut offset: usize = 0;
    for _i in 0..len {
        let from = read_ushort(lump, &mut offset).unwrap();
        let to = read_ushort(lump, &mut offset).unwrap();
        let flags = read_ushort(lump, &mut offset).unwrap();
        let type_ = read_u8(lump, &mut offset).unwrap();
        let arg1 = read_u8(lump, &mut offset).unwrap();
        let arg2 = read_u8(lump, &mut offset).unwrap();
        let arg3 = read_u8(lump, &mut offset).unwrap();
        let arg4 = read_u8(lump, &mut offset).unwrap();
        let arg5 = read_u8(lump, &mut offset).unwrap();
        let front_sidedef = read_ushort(lump, &mut offset).unwrap();
        let back_sidedef = read_ushort(lump, &mut offset).unwrap();
        let linedef  = WADLevelLinedef {
            from,
            to,
            flags,
            doom: None,
            hex: Some(WADLevelHexenLinedef { type_, arg1, arg2, arg3, arg4, arg5}),
            front_sidedef,
            back_sidedef
        };
        linedefs.push(linedef);
    }
    linedefs
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
        let front_sidedef = read_ushort(lump, &mut offset).unwrap();
        let back_sidedef = read_ushort(lump, &mut offset).unwrap();
        let linedef  = WADLevelLinedef {
            from,
            to,
            flags,
            doom: Some(WADLevelDoomLinedef { types, tag }),
            hex: None,
            front_sidedef,
            back_sidedef
        };
        linedefs.push(linedef);
    }
    linedefs
}

fn parse_sidedefs(lump: &Vec<u8>) -> Vec<WADLevelSidedef> {
    let mut sidedefs: Vec<WADLevelSidedef> = vec![];

    let entry_len = 30;
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
        let right_child = read_ushort(lump, &mut offset).unwrap();
        let left_child = read_ushort(lump, &mut offset).unwrap();
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

//TODO see if this is even needed
fn parse_blockmap(lump: &Vec<u8>) -> WADLevelBlockmap {
    let mut offset: usize = 0;

    let x = read_short(lump, &mut offset).unwrap();
    let y = read_short(lump, &mut offset).unwrap();
    let width = read_ushort(lump, &mut offset).unwrap() & 0xffff;
    let height = read_ushort(lump, &mut offset).unwrap() & 0xffff;

    // let num_blocks: u32 = u32::from(width) * u32::from(height);
    // let mut blocklists: Vec<Vec<u16>> = vec![];
    let count = lump.len() / 2 - 4;
    let mut blockmap_lump: Vec<i32> = vec![];
    for _i in 0..count {
        let t = read_short(lump, &mut offset).unwrap();
        let t_res;
        if t == -1 {t_res = (0xffffffff as u32) as i32} else { t_res = ((t as u32) & 0xffff) as i32}
        blockmap_lump.push(t_res);
    }
    // for i in 0..blocklist_offsets.len() {
    //     let mut blocklist: Vec<u16> = vec![];

    //     offset = blocklist_offsets[i] as usize;
    //     read_ushort(lump, &mut offset).unwrap();//skip the first 0x0000 start of the blocklist

    //     let mut linedef_index = read_ushort(lump, &mut offset).unwrap();
    //     while linedef_index != 65535 {
    //         blocklist.push(linedef_index);
    //         linedef_index = read_ushort(lump, &mut offset).unwrap();
    //     }
    //     blocklists.push(blocklist);
    // }
    let blockmap = WADLevelBlockmap {
        x,
        y,
        width,
        height,
        blockmap_lump
        // blocklists
    };
    blockmap
}

fn parse_rejects(lump: &Vec<u8>, sector_size: usize) -> Vec<Vec<bool>> {
    let mut rejects: Vec<Vec<bool>> = vec![vec![false; sector_size]; sector_size];
    let mut col = 0;
    let mut row = 0;

    let mut offset = 0;
    while offset < sector_size {
        let mut reader = BitReader::new(&lump[offset..offset+1]);
        for _i in 0..8 {
            let bit = reader.read_u8(1).unwrap();
            if col == sector_size {
                col = 0;
                row += 1;
            }

            // Check if we have already filled the table even if we still have bits left (rounding)
            if row >= sector_size {break}

            rejects[col][row] = bit != 0;
            col += 1;
        }
        offset += 1;
    }
    rejects
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

    // println!("index of blockmap: {}, len: {}, name: {}", wad_parsed.lump_map.get("BLOCKMAP").unwrap(), wad_parsed.lump_map.len(), wad_parsed.directory[104].name);

    let name = wad_parsed.directory[index].name.to_string();
    if format == Format::DOOM || format == Format::HEXEN {
        let mut has_behavior = false;
        let is_text = false;
        let things;
        let linedefs;
        let sidedefs = parse_sidedefs(&get_lump_from_dir(index + Lumps::SIDEDEFS as usize, wad_parsed, wad_data));
        let vertexes = parse_vertexes(&get_lump_from_dir(index + Lumps::VERTEXES as usize, wad_parsed, wad_data));
        let segs = parse_segs(&get_lump_from_dir(index + Lumps::SEGS as usize, wad_parsed, wad_data));
        let ssectors = parse_subsectors(&get_lump_from_dir(index + Lumps::SSECTORS as usize, wad_parsed, wad_data));
        let nodes = parse_nodes(&get_lump_from_dir(index + Lumps::NODES as usize, wad_parsed, wad_data));
        let sectors = parse_sectors(&get_lump_from_dir(index + Lumps::SECTORS as usize, wad_parsed, wad_data));
        let reject = parse_rejects(&get_lump_from_dir(index + Lumps::REJECT as usize, wad_parsed, wad_data), sectors.len());
        let blockmap = parse_blockmap(&get_lump_from_dir(index + Lumps::BLOCKMAP as usize, wad_parsed, wad_data));
        println!("blockmap parser for now disabled: {:?}", Lumps::BLOCKMAP);
        if format == Format::DOOM {
            things = parse_things(&get_lump_from_dir(index + Lumps::THINGS as usize, wad_parsed, wad_data));
            linedefs = parse_linedefs(&get_lump_from_dir(index + Lumps::LINEDEFS as usize, wad_parsed, wad_data));
            wad_parsed.levels.push(WADLevel { name, things, linedefs, sidedefs, vertexes, segs, ssectors, nodes, sectors, reject, format, has_behavior, is_text, behavior: None, znodes: vec![], gl_znodes: vec![], blockmap });
        }
        else if format == Format::HEXEN {
            has_behavior = true;
            things = parse_hexen_things(&get_lump_from_dir(index + Lumps::THINGS as usize, wad_parsed, wad_data));
            linedefs = parse_hexen_linedefs(&get_lump_from_dir(index + Lumps::LINEDEFS as usize, wad_parsed, wad_data));
            let behavior = Some(WADLevelBehavior::new());
            println!("Still need to implement parsing of behavior: {:?}", Lumps::BEHAVIOR);
            wad_parsed.levels.push(WADLevel { name, things, linedefs, sidedefs, vertexes, segs, ssectors, nodes, sectors, reject, format, has_behavior, is_text, behavior, znodes: vec![], gl_znodes: vec![], blockmap });
        }
    }
    
    //TODO check for udmf
}