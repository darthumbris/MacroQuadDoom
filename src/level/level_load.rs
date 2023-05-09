use crate::parser::parse_level::WADLevel;

use super::LevelLocals;
use super::level_elements::{Vertex, Sector, ExtSector, SectorFlags, SectorE, Line, SideDefIndex, LineFlags};
use super::level_lightmap::PalEntry;
use super::level_texture::MissingTextureTracker;
use super::LevelFlags;

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

fn set_texture(sector: &Sector, index: usize, pos: usize, name: &String, track: &MissingTextureTracker, truncate: bool) {

}

pub fn load_sectors(map: &WADLevel, missing_textures: &MissingTextureTracker, level: &LevelLocals) -> Vec<Sector> {
    let mut sectors: Vec<Sector> = Vec::with_capacity(map.sectors.len());
    let mut ext_sectors: Vec<ExtSector> = Vec::with_capacity(map.sectors.len());
    let def_sec_type;

    if (level.flags & LevelFlags::SndSeqTotalCtrl.bits()) != 0 { def_sec_type = 0; } else {def_sec_type = 1;}

    for i in 0..map.sectors.len() {
        ext_sectors.push(ExtSector::new());
        let mut sector = Sector::new(i as i32);
        let ms = &map.sectors[i];

        if map.has_behavior {
            sector.flags |= SectorFlags::FloorDrop.bits();
        }
        let floor = SectorE::Floor as usize;
        sector.set_plane_tex_z(floor, f64::from(ms.floor_height), None);
        sector.floorplane.set(0., 0., 1., -sector.get_plane_tex_z(floor));

        let ceiling = SectorE::Ceiling as usize;
        sector.set_plane_tex_z(ceiling, f64::from(ms.ceiling_height), None);
        sector.ceilingplane.set(0., 0., -1., sector.get_plane_tex_z(ceiling));

        set_texture(&sector, i, floor, &ms.floor_texture, missing_textures, true);
        set_texture(&sector, i, ceiling, &ms.ceiling_texture, missing_textures, true);

        sector.light_level = ms.light_level;

        if map.has_behavior { sector.special = i32::from(ms.special); }
        else { sector.special = level.translate_sector_special(ms.special);}

        level.tag_manager.add_sector_tag(i, ms.tag);
        sector.sec_type = def_sec_type;
        sector.next_sec = -1;
        sector.prev_sec = -1;
        sector.set_alpha(floor, 1.);
        sector.set_alpha(ceiling, 1.);
        sector.set_x_scale(floor, 1.);
        sector.set_x_scale(ceiling, 1.);
        sector.set_y_scale(floor, 1.);
        sector.set_y_scale(ceiling, 1.);
        
        sector.gravity = 1.;
        sector.zone_number = 0xffff;
        sector.terrain_num[floor] = -1;
        sector.terrain_num[ceiling] = -1;

        sector.color_map.light_color = PalEntry::new_rgb(255,255,255);
        if level.outside_fog_color != 0xff000000 && sector.get_texture(ceiling) == level.sky_flat_num || sector.special & 0xff == 87 /*sectour_outside*/ {
            sector.color_map.fade_color.set_rgb(level.outside_fog_color);
        }
        else if (level.flags & LevelFlags::HasFadeTable.bits()) != 0 {
            sector.color_map.fade_color = PalEntry::D(0x939393);
        }
        else {
            sector.color_map.fade_color.set_rgb(level.fade_to_color);
        }

        sector.friction = 59392./65536.;
        sector.move_factor = 2048./65536.;
        sector.sector_num = i as i32;
        sector.ibo_count = -1;

        sectors.push(sector);
    }

    sectors
}

fn set_side_num(sidedef: &mut SideDefIndex, side_count: u16, level: &LevelLocals) {
    if side_count == 0xffff /*NO_INDEX */ {
        *sidedef = -1;
    }
    else if (side_count as usize) < level.elements.sides.len() {
        *sidedef = side_count as i32;
        //TODO something something side_temp vec update
    }
    else {
        eprintln!("{} sidedefs is not enough", side_count);
    }
}

fn save_line_special(line: &Line) {
    if line.sidedef[0] == -1 {
        return
    }

    let side_count = line.sidedef[0];
    if line.special != 190 /*Static_INIT*/ || line.args[1] == 1 /*Init_COLOR ? */ {
        //TODO something something side_temp vec update
        //TODO something something side_temp vec update
    }
    else {
        //TODO something something side_temp vec update
    }
}

pub fn load_linedefs(map: &mut WADLevel, level: &LevelLocals) -> Vec<Line> {

    let mut line_count = map.linedefs.len();
    let mut side_count = map.sidedefs.len();
    let mut skipped = 0;
    let mut i = 0;

    while i < line_count {
        let linedef = &map.linedefs[i];
        let v1 = linedef.from;
        let v2 = linedef.to;

        if v1 as usize >= level.elements.vertexes.borrow().len() || v2  as usize >= level.elements.vertexes.borrow().len() {
            eprintln!("Line {} has invalid vertices: {} and/or {}.\nThe map only contains {} vertices.",
            i + skipped, v1, v2, level.elements.vertexes.borrow().len());
        }
        else if v1 == v2 || 
        (level.elements.vertexes.borrow()[v1 as usize].fx() == level.elements.vertexes.borrow()[v2 as usize].fx()
        && level.elements.vertexes.borrow()[v1 as usize].fy() == level.elements.vertexes.borrow()[v2 as usize].fy()
        ) {
            println!("removing 0-length line {}", i + skipped);
            map.linedefs.remove(i);
            //TODO forcenodebuild = true
            skipped += 1;
            line_count -= 1;
        }
        else {
            side_count += 1;
            if linedef.back_sidedef != 0xffff /*NO_INDEX */ {
                side_count += 1;
            }
            i +=1;
        }
    }
    
    let mut lines: Vec<Line> = Vec::with_capacity(line_count);
    //TODO allocate sidedefs
    for i in 0..line_count {
        let mut linedef = &mut map.linedefs[i];
        let mut line = Line::new();

        line.alpha = 1.;
        line.portal_index = u32::max_value();
        line.portal_transfered = u32::max_value();

        //TODO check if the translate of old linedef special is needed?
        level.translate_linedef(&line, linedef, -1);
        if line.special != 190 /*Static_INIT ? */ && line.args[1] != 254 /*InitEdLine */ && line.args[1] != 253 /*InitEdSector */{
            let temp = linedef.clone();
            level.tag_manager.add_line_id(i, temp.doom.unwrap().tag);
        }
        if line.special == 190 /*Static_INIT ? */ && line.args[1] == 254 /*InitEdLine */ {
            //TODO processEdLineDef function
        }

        if linedef.front_sidedef != 0xffff /*NO_INDEX */ && (linedef.front_sidedef as usize) >= side_count {
            linedef.front_sidedef = 0; //dummy sidedef
            println!("Linedef {} has a bad sidedef", i);
        }
        if linedef.back_sidedef != 0xffff /*NO_INDEX */ && (linedef.back_sidedef as usize) >= side_count {
            linedef.back_sidedef = 0; //dummy sidedef
            println!("Linedef {} has a bad sidedef", i);
        }
        if linedef.front_sidedef == 0xffff /*NO_INDEX */ {
            linedef.front_sidedef = 0;
            println!("Linedef {} has no front sidedef", i);
        }

        let vertexes = level.elements.vertexes.borrow();
        // let temp = vertexes[linedef.from as usize];
        line.v1 = vertexes[linedef.from as usize].clone();
        line.v2 = vertexes[linedef.to as usize].clone();

        set_side_num(&mut line.sidedef[0], linedef.front_sidedef, level);
        set_side_num(&mut line.sidedef[1], linedef.back_sidedef, level);

        line.adjust_line();
        save_line_special(&line);

        if level.flags2 & LevelFlags::Level2ClipMidTex.bits() != 0 {
            line.flags |= LineFlags::ClipMidTex.bits();
        }
        if level.flags2 & LevelFlags::Level2WrapMidTex.bits() != 0 {
            line.flags |= LineFlags::WrapMidTex.bits();
        }
        if level.flags2 & LevelFlags::Level2CheckSwitchRange.bits() != 0 {
            line.flags |= LineFlags::CheckSwitchRange.bits();
        }
        lines.push(line);
    }

    lines
}

pub fn load_linedefs2(map: &WADLevel) -> Vec<Line> {
    let lines: Vec<Line> = vec![];

    lines

    //TODO make this
}

pub fn load_sidedefs2() {

}

fn finish_loading_linedefs() {

}

pub fn load_things() {

}

pub fn load_things2() {

}