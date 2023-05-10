use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;

use crate::behavior::parse_level::WADLevelLinedef;
use crate::parser::parse_level::WADLevel;

use super::LevelLocals;
use super::level_elements::{Vertex, Sector, ExtSector, SectorFlags, SectorE, Line, SideDefIndex, LineFlags, Side, SectorIndex, Sides};
use super::level_lightmap::PalEntry;
use super::level_texture::{MissingTextureTracker, TextureID, TextureManager, MapSideDef};
use super::LevelFlags;

pub struct MapLoader<'a, 'b> {
    pub level: &'a mut  LevelLocals,
    pub tex_manager: &'b TextureManager,
    pub force_node_build: bool,
    // first_gl_vertex: i32,
    side_count: i32,
    line_map: Vec<i32>,
    side_temp: Vec<SideInit>,
    
}

impl MapLoader<'_, '_> {
    pub fn new<'a, 'b>(level: &'a mut LevelLocals, tex_manager: &'b TextureManager) -> MapLoader<'a, 'b>{
        MapLoader { level, tex_manager, force_node_build: false, side_count: 0, line_map: vec![], side_temp: vec![] }
    }

    /* 
     * for behavior: first loadbehavior, then load default modules , then load the ACS lump
     * if strife also load the dialogues
     */


    /* This function will load a single level (i.e. e1m1) and for normal doom map may 
     * need a translator to make it a udmf like map?
     * also needs to load the scripts */
    pub fn load_level() {
    
    }

    fn load_vertexes(&mut self, map: &WADLevel) {
        if map.vertexes.len() == 0 {
            eprintln!("Map has no vertexes");
        }
        for v in &map.vertexes {
            self.level.vertexes.push(Rc::new(RefCell::new(Vertex::new(v.x, v.y))));
        }
    }

    fn load_sectors(&mut self, map: &mut WADLevel, missing_textures: &MissingTextureTracker) {
        let def_sec_type;
    
        if (self.level.flags & LevelFlags::SndSeqTotalCtrl.bits()) != 0 { def_sec_type = 0; } else {def_sec_type = 1;}
    
        for i in 0..map.sectors.len() {
            self.level.extsectors.push(ExtSector::new());
            let mut sector = Sector::new(i as i32);
            let ms = &mut map.sectors[i];
    
            if map.has_behavior {
                sector.flags |= SectorFlags::FloorDrop.bits();
            }
            let floor = SectorE::Floor as usize;
            sector.set_plane_tex_z(floor, f64::from(ms.floor_height), None);
            sector.floorplane.set(0., 0., 1., -sector.get_plane_tex_z(floor));
    
            let ceiling = SectorE::Ceiling as usize;
            sector.set_plane_tex_z(ceiling, f64::from(ms.ceiling_height), None);
            sector.ceilingplane.set(0., 0., -1., sector.get_plane_tex_z(ceiling));
    
            Self::set_texture_sector(self, &mut sector, i, floor, &mut ms.floor_texture, missing_textures, true);
            Self::set_texture_sector(self, &mut sector, i, ceiling, &mut ms.ceiling_texture, missing_textures, true);
    
            sector.light_level = ms.light_level;
    
            if map.has_behavior { sector.special = i32::from(ms.special); }
            else { sector.special = self.level.translate_sector_special(ms.special);}
    
            self.level.tag_manager.add_sector_tag(i, ms.tag);
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
            if self.level.outside_fog_color != 0xff000000 && sector.get_texture(ceiling) == self.level.sky_flat_num || sector.special & 0xff == 87 /*sectour_outside*/ {
                sector.color_map.fade_color.set_rgb(self.level.outside_fog_color);
            }
            else if (self.level.flags & LevelFlags::HasFadeTable.bits()) != 0 {
                sector.color_map.fade_color = PalEntry::D(0x939393);
            }
            else {
                sector.color_map.fade_color.set_rgb(self.level.fade_to_color);
            }
    
            sector.friction = 59392./65536.;
            sector.move_factor = 2048./65536.;
            sector.sector_num = i as i32;
            sector.ibo_count = -1;
            self.level.sectors.push(Rc::new(RefCell::new(sector)));
            // sectors.push(sector);
        }
    }


    fn load_linedefs(&mut self, map: &mut WADLevel) {

        let mut line_count = map.linedefs.len();
        let mut side_count = map.sidedefs.len();
        let mut skipped = 0;
        let mut i = 0;
    
        while i < line_count {
            let linedef = &map.linedefs[i];
            let v1 = linedef.from;
            let v2 = linedef.to;
    
            if v1 as usize >= self.level.elements.vertexes.len() || v2  as usize >= self.level.elements.vertexes.len() {
                eprintln!("Line {} has invalid vertices: {} and/or {}.\nThe map only contains {} vertices.",
                i + skipped, v1, v2, self.level.elements.vertexes.len());
            }
            else if v1 == v2 || 
            (self.level.elements.vertexes[v1 as usize].borrow_mut().fx() == self.level.elements.vertexes[v2 as usize].borrow_mut().fx()
            && self.level.elements.vertexes[v1 as usize].borrow_mut().fy() == self.level.elements.vertexes[v2 as usize].borrow_mut().fy()
            ) {
                println!("removing 0-length line {}", i + skipped);
                map.linedefs.remove(i);
                self.force_node_build = true;
                skipped += 1;
                line_count -= 1;
            }
            else {
                side_count += 1;
                if linedef.back_sidedef != 0xffff /*NO_INDEX */ {side_count += 1;}
                i +=1;
            }
        }
        
        self.level.lines.reserve(line_count);
        Self::allocate_sidedefs(self, &map, side_count);
        for i in 0..line_count {
            let mut linedef = &mut map.linedefs[i];
            let mut line = Line::new();
    
            line.alpha = 1.;
            line.portal_index = u32::max_value();
            line.portal_transfered = u32::max_value();
            //TODO check if the translate of map linedef special,tag and flags is needed?

            self.level.translate_linedef(&line, linedef, -1);
            if line.special != 190 /*Static_INIT ? */ && line.args[1] != 254 /*InitEdLine */ && line.args[1] != 253 /*InitEdSector */{
                let temp = linedef.clone();
                self.level.tag_manager.add_line_id(i, temp.doom.unwrap().tag);
            }
            if line.special == 190 /*Static_INIT ? */ && line.args[1] == 254 /*InitEdLine */ {
                Self::process_eternity_doom_linedef(self, &line, &linedef);
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
    
            let vertexes = &self.level.vertexes;
            line.v1 = vertexes[linedef.from as usize].borrow_mut().clone();
            line.v2 = vertexes[linedef.to as usize].borrow_mut().clone();
    
            Self::set_side_num(self, &mut line.sidedef[0], linedef.front_sidedef);
            Self::set_side_num(self, &mut line.sidedef[1], linedef.back_sidedef);
    
            line.adjust_line();
            Self::save_line_special(self, &line);
    
            if self.level.flags2 & LevelFlags::Level2ClipMidTex.bits() != 0 {
                line.flags |= LineFlags::ClipMidTex.bits();
            }
            if self.level.flags2 & LevelFlags::Level2WrapMidTex.bits() != 0 {
                line.flags |= LineFlags::WrapMidTex.bits();
            }
            if self.level.flags2 & LevelFlags::Level2CheckSwitchRange.bits() != 0 {
                line.flags |= LineFlags::CheckSwitchRange.bits();
            }
            self.level.lines.push(Rc::new(RefCell::new(line)));
        }
    }

    //This is for hexen map formats
    fn load_linedefs2(&mut self, map: &WADLevel) {
    
        //TODO make this
    }
    
    fn load_sidedefs(&mut self, map: &WADLevel, missing_textures: &MissingTextureTracker) {
        for i in 0..self.level.sides.len() {
            let sideinit = self.side_temp[i];
            let mut index = 0;
            match sideinit {
                SideInit::A(t) => {index = t.map as usize}
                SideInit::B(t) => {}
            }
            let map_sidedef = &map.sidedefs[index];
            let mut side = self.level.sides[i].borrow_mut();
            
            side.set_texture_x_offset(map_sidedef.x_offset as f64);
            side.set_texture_y_offset(map_sidedef.y_offset as f64);
            side.set_texture_x_scale(1.);
            side.set_texture_y_scale(1.);
            side.linedef = -1;
            side.flags = 0;
            side.udmf_index = i as i32;

            if map_sidedef.sector as usize >= self.level.sectors.len() {
                println!("sidedef {} has a bad sector", i);
                side.sector = 0;
            }
            else {side.sector = map_sidedef.sector as i32;}

            let mut top_texture = map_sidedef.upper_texture.clone();
            let mut middle_texture = map_sidedef.middle_texture.clone();
            let mut bottom_texture = map_sidedef.lower_texture.clone();
            top_texture.truncate(8);
            middle_texture.truncate(8);
            bottom_texture.truncate(8);

            let imsd: MapSideDef = MapSideDef {
                top_texture,
                middle_texture,
                bottom_texture
            };
            let sec = side.sector;
            Self::process_side_textures(self, !map.has_behavior, &mut side, sec, &imsd, missing_textures, &sideinit);

        }
    }
    
    fn finish_loading_linedefs(&mut self, map: &WADLevel) {
    
    }
    
    fn load_things(&mut self, map: &WADLevel) {
    
    }
    
    //This is for hexen map formats
    fn load_things2(&mut self, map: &WADLevel) {
    
    }

    fn set_side_num(&self, sidedef: &mut SideDefIndex, side_count: u16) {
        if side_count == 0xffff /*NO_INDEX */ {*sidedef = -1;}
        else if (side_count as usize) < self.level.elements.sides.len() {
            *sidedef = side_count as i32;
            match self.side_temp[side_count as usize] {
                SideInit::A(mut t) => {t.map = side_count as u32}
                SideInit::B(mut t) => {/*TODO What*/}
            }
        }
        else {eprintln!("{} sidedefs is not enough", side_count);}
    }

    fn save_line_special(&self, line: &Line) {
        if line.sidedef[0] == -1 {return}
    
        let side_count = line.sidedef[0] as usize;
        let side_i = self.side_temp[side_count];
        if line.special != 190 /*Static_INIT*/ || line.args[1] == 1 /*Init_COLOR ? */ {            
            match side_i {
                SideInit::A(mut t) => {
                    t.special = line.special as i16;
                    t.tag = line.args[0] as i16;
                }
                SideInit::B(mut t) => {/*TODO What*/}
            }
        }
        else {
            match side_i {
                SideInit::A(mut t) => { t.special = 0;}
                SideInit::B(mut t) => {/*TODO What*/}
            }
        }
    }

    fn allocate_sidedefs(&mut self, map: &WADLevel, count: usize) {
        self.level.sides.reserve(count);
        self.level.sides.fill(Rc::new(RefCell::new(Side::new())));

        self.side_temp.resize_with(count.max(self.level.vertexes.len()), || SideInit::new());

        for i in 0..count {
            let temp = self.side_temp[i];
            match temp {
                SideInit::A(mut t) => {
                    t.special = 0;
                    t.tag = 0;
                    t.alpha = i16::min_value();
                    t.map = u32::max_value();
                }
                SideInit::B(t) => {/*TODO What*/}
            }
        }
        if count < map.sidedefs.len() {
            println!("map has {} unused sidedefs", map.sidedefs.len() - count);
        }
        self.side_count = 0;
    }

    fn process_eternity_doom_linedef(&self, line: &Line, maplinedef: &WADLevelLinedef) {
        //TODO
        Self::init_eternity_doom(self);

        let record_num = maplinedef.doom.as_ref().unwrap().tag;

    }

    fn init_eternity_doom(&self) {
        //TODO
    }

    fn set_texture_sector(&self, sector: &mut Sector, index: usize, pos: usize, name: &mut String, track: &MissingTextureTracker, truncate: bool) {
        //TODO
        let position_names = ["floor", "ceiling"];
    
        if truncate { name.pop(); }
        let mut texture: TextureID = self.tex_manager.check_for_texture();

        if !texture.exists() {
             if track.contains_key(name) && track.get(name).unwrap().count <= 20 /*Missing_texture_warn_limit */ {
                println!("unkown {:?} texture {:?} in sector {}", position_names[pos], name, index);
            }
            
            texture = self.tex_manager.get_default_texture();
        }
        sector.set_texture(pos, texture);
    }

    fn set_texture_side_blend(&self, side: &Side, pos: usize, blend: &u32, name: &String) {

    }

    fn set_texture_side(&self, side: &Side, pos: usize, name: &String, missing_textures: &MissingTextureTracker) {

    }

    fn set_texture_side_no_error(&self, side: &Side, pos: usize, color: &u32, name: &String, valid_color: &bool, is_fog: bool) {

    }

    fn process_side_textures(&self, check_transfer_map:bool, side: &mut Side, sector: SectorIndex, imsd: &MapSideDef, missing_textures: &MissingTextureTracker, sideinit: &SideInit) {
        match sideinit {
            SideInit::A(t) => {
                let sec = &self.level.sectors[sector as usize];
                match t.special {
                    209 /*Tranfer_Heights */ => {
                        if sector != -1 {
                            Self::set_texture_side_blend(&self, side, Sides::Bottom.bits() as usize, &sec.borrow_mut().bottom_map, &imsd.bottom_texture);
                            Self::set_texture_side_blend(&self, side, Sides::Mid.bits() as usize, &sec.borrow_mut().mid_map, &imsd.middle_texture);
                            Self::set_texture_side_blend(&self, side, Sides::Top.bits() as usize, &sec.borrow_mut().top_map, &imsd.top_texture);
                        }
                    }

                    190 /*Static_INIT*/ => {
                        let color:u32 = u32::from_le_bytes([0,255,255,255]);
                        let fog:u32 = 0;
                        let color_good: bool = false;
                        let fog_good: bool = false;

                        Self::set_texture_side_no_error(self,side, Sides::Bottom.bits() as usize, &fog, &imsd.bottom_texture, &fog_good, true);
                        Self::set_texture_side_no_error(self,side, Sides::Top.bits() as usize, &color, &imsd.top_texture, &color_good, false);
                        Self::set_texture_side(self,side, Sides::Mid.bits() as usize, &imsd.middle_texture, missing_textures);

                        if color_good | fog_good {
                            for i in 0..self.level.sectors.len() {
                                if self.level.sector_has_tag(i, t.tag) {
                                    if color_good {
                                        self.level.sectors[i].borrow_mut().color_map.light_color.set_rgb(color);
                                        self.level.sectors[i].borrow_mut().color_map.blend_factor = (color >> 24 & 0xff) as u8;
                                    }
                                    if fog_good {
                                        self.level.sectors[i].borrow_mut().color_map.fade_color.set_rgb(fog);
                                    }
                                }
                            }
                        }
                    }   

                    160 /*Sector_Set3dFloor */ => {
                        if imsd.top_texture.chars().nth(0).unwrap() == '#' {
                            let mut shortened = imsd.top_texture.clone();
                            shortened.remove(0);
                            let id = shortened.to_string().parse::<i128>().unwrap();
                            side.set_texture(Sides::Top.bits() as usize, TextureID { tex_num: id as i32});
                        }
                        else {
                            Self::set_texture_side(self,side, Sides::Top.bits() as usize, &imsd.top_texture, missing_textures);
                        }

                        Self::set_texture_side(self,side, Sides::Mid.bits() as usize, &imsd.middle_texture, missing_textures);
                        Self::set_texture_side(self,side, Sides::Bottom.bits() as usize, &imsd.bottom_texture, missing_textures);
                    }
                    208 /*Translucent Line */ => {
                        //TODO
                    }
                    _ => {
                        Self::set_texture_side(self,side, Sides::Mid.bits() as usize, &imsd.middle_texture, missing_textures);
                        Self::set_texture_side(self,side, Sides::Top.bits() as usize, &imsd.top_texture, missing_textures);
                        Self::set_texture_side(self,side, Sides::Bottom.bits() as usize, &imsd.bottom_texture, missing_textures);
                    }                 
                }

            }

            SideInit::B(t) => {}
        }
    }
}










#[derive(Clone, Copy)]
pub enum SideInit {
    A(SideInitA),
    B(SideInitB)
}

impl SideInit {
    pub fn new() -> SideInit {
        SideInit::A(SideInitA { tag: 0, special: 0, alpha: 0, map: 0 })
    }
}

#[derive(Clone, Copy)]
pub struct SideInitA {
    pub tag: i16,
    pub special: i16,
    pub alpha: i16,
    pub map: u32
}

#[derive(Clone, Copy)]
pub struct SideInitB {
    pub first: u32,
    pub next: u32,
    pub lineside: u8
}