use std::rc::Rc;
use std::cell::RefCell;

use crate::behavior::parse_level::WADLevelLinedef;
use crate::game::{Game, GameType};
use crate::parser::parse_level::WADLevel;
use crate::vector::{Vector3, Vector2};

use super::level_actor::ClassActor;
use super::level_mesh::LevelMesh;
use super::{LevelLocals, ActionSpecials, SpecialMapThings, MapThingFlags};
use super::level_elements::{Vertex, Sector, ExtSector, SectorFlags, SectorE, Line, SideDefIndex, LineFlags, Side, SectorIndex, Sides};
use super::level_lightmap::PalEntry;
use super::level_texture::{MissingTextureTracker, TextureID, TextureManager, MapSideDef, TextureType, TexManFlags, FakeColorMap};
use super::LevelFlags;
use crate::file_system::FileSystem;

pub struct MapLoader<'a, 'b> {
    pub level: &'a mut  LevelLocals,
    pub tex_manager: &'b TextureManager,
    // pub game: Rc<Weak<Game>>,
    pub force_node_build: bool,
    pub map_things_converted: Vec<MapThing>,
    // first_gl_vertex: i32,
    side_count: i32,
    line_map: Vec<i32>,
    side_temp: Vec<SideInit>,
    file_system: Option<FileSystem>,
    fake_color_maps: Vec<FakeColorMap>
}

impl MapLoader<'_, '_> {
    pub fn new<'a, 'b>(level: &'a mut LevelLocals, tex_manager: &'b TextureManager) -> MapLoader<'a, 'b>{
        MapLoader { level, tex_manager, force_node_build: false, side_count: 0, line_map: vec![], side_temp: vec![], file_system: None, fake_color_maps: vec![], map_things_converted: vec![] }
    }

    /* 
     * for behavior: first loadbehavior, then load default modules , then load the ACS lump
     * if strife also load the dialogues
     */


    /* This function will load a single level (i.e. e1m1) and for normal doom map may 
     * need a translator to make it a udmf like map?
     * also needs to load the scripts */
    pub fn load_level(&mut self, map: &mut WADLevel, game: &Game) {
        /*TODO 
         * loadbehavior()
         * T_LoadScripts();
         * Level->Behaviors.LoadDefaultModules();
         * LoadMapinfoACSLump();
         * LoadStrifeConversations();
        */

        let mut missing_textures: MissingTextureTracker = MissingTextureTracker::new();

        if !map.is_text {
            Self::load_vertexes(self, map);
            println!("going to load sectors");
            Self::load_sectors(self, map, &mut missing_textures);
            println!("finished loading sectors");
            println!("going to load lines");
            if !map.has_behavior {Self::load_linedefs(self, map)}
            else {Self::load_linedefs2(self, map)}
            println!("finished loading lines");
            println!("going to load sides");
            Self::load_sidedefs(self, map, &mut missing_textures);
            println!("finished loading sides");
            Self::finish_loading_linedefs(self);
            println!("finished loading sides");
            println!("going to load things");
            if !map.has_behavior {Self::load_things(self, map, game)}
            else {Self::load_things2(self, map, game)}
            println!("finished loading things");
        }
        else {
            //TODO parse textmap
        }

        /* TODO
         * CalcIndices();
         * PostProcessLevel();
         * LoopSidedefs();
         * 
         * 
         * LoadBlockMap();
         * LoadReject();
         * 
         * Spawn Functions
         * 
         * LoadLightMap();
         */

         println!("map has {} vertexes", self.level.vertexes.len());
         println!("map has {} sectors", self.level.sectors.len()); //TODO check why no sectors
         println!("map has {} lines", self.level.lines.len());
         println!("map has {} sides", self.level.sides.len()); //TODO check why no sides
         println!("map has {} segs", self.level.segs.len());
         println!("map has {} subsectors", self.level.subsectors.len());
         println!("map has {} nodes", self.level.nodes.len());

        self.level.level_mesh = Rc::new(LevelMesh::new(&self.level, self.tex_manager));
    }

    fn load_vertexes(&mut self, map: &WADLevel) {
        if map.vertexes.len() == 0 {
            eprintln!("Map has no vertexes");
        }
        println!("going to load: {} vertexes", map.vertexes.len());
        for v in &map.vertexes {
            self.level.vertexes.push(Rc::new(RefCell::new(Vertex::new(v.x, v.y))));
        }
        println!("loaded vertexes: {}", self.level.vertexes.len());
    }

    fn load_sectors(&mut self, map: &mut WADLevel, missing_textures: &MissingTextureTracker) {
        let def_sec_type;
    
        if (self.level.flags & LevelFlags::SndSeqTotalCtrl.bits()) != 0 { def_sec_type = 0; } else {def_sec_type = 1;}
        println!("going to load: {} sectors", map.sectors.len());
        for i in 0..map.sectors.len() {
            println!("going to load sector: {}", i);
            self.level.extsectors.push(ExtSector::new());
            println!("made a new extsector: {}", i);
            let mut sector = Sector::new(i as i32);
            println!("made a new sector: {}", i);
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
            println!("going to push sector: {}", i);
            self.level.sectors.push(Rc::new(RefCell::new(sector)));
            println!("pushed sector: {}", i);
        }
    }


    fn load_linedefs(&mut self, map: &mut WADLevel) {

        let mut line_count = map.linedefs.len();
        let mut side_count = map.sidedefs.len();
        self.line_map.resize(line_count, 0);
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
                self.line_map[i] = (i + skipped) as i32;
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
    
    fn finish_loading_linedefs(&mut self) {
        for i in 0..self.level.lines.len() {
            let mut index = self.level.lines[i].borrow().sidedef[0] as usize;
            if self.level.lines[i].borrow().sidedef[0] == -1 {index = 0}
            println!("trying to acces: {} and len is: {}", index, self.side_temp.len());
            println!("sidedef[0]: {}, i: {}", self.level.lines[i].borrow().sidedef[0], i);
            match self.side_temp[index] {
                SideInit::A(t) => { Self::finish_loading_linedef(self, i, t.alpha)}
                SideInit::B(_) => {}
            }
        }
    }

    fn finish_loading_linedef(&mut self, line_index: usize, alpha: i16) {
        let mut line = self.level.lines[line_index].borrow_mut();
        let mut alpha = alpha;
        let mut additive = false;

        if line.sidedef[0] != -1 {line.front_sector = line.sidedef[0]} else {line.front_sector = -1}
        if line.sidedef[1] != -1 {line.back_sector = line.sidedef[1]} else {line.back_sector = -1}
        
        let dx: f64 = line.v2.fx() - line.v1.fx();
        let dy: f64 = line.v2.fy() - line.v1.fy();

        let line_num = line.index() as usize;

        if line.front_sector == -1 {println!("Line {} has no front sector", self.line_map[line_num])}

        let len = ((dx * dx + dy * dy).sqrt() + 0.5) as i32;

        if line.sidedef[0] != -1 {
            let mut side = self.level.sides[line.sidedef[0] as usize].borrow_mut();
            side.linedef = line_index as i32;
            side.texel_length = len as u16;
        }
        if line.sidedef[1] != -1 {
            let mut side = self.level.sides[line.sidedef[1] as usize].borrow_mut();
            side.linedef = line_index as i32;
            side.texel_length = len as u16;
        }

        let match_special = num::FromPrimitive::from_i32(line.special);
        match match_special {
            Some(ActionSpecials::TranslucentLine) => {
                if alpha == i16::min_value() {
                    alpha = line.args[1] as i16;
                    if line.args[2] == 0 {additive = false} else {additive = true}
                }
                else if alpha < 0 {
                    alpha = -alpha;
                    additive = true;
                }

                let d_alpha: f64 = alpha as f64 / 255.;
                if line.args[0] == 0 {
                    line.alpha = d_alpha;
                    if additive {line.flags |= LineFlags::AddTrans.bits()}
                }
                else {
                    for j in 0..self.level.lines.len() {
                        if self.level.line_has_id(j as i32, line.args[0]) {
                            self.level.lines[j].borrow_mut().alpha = d_alpha;
                            if additive {self.level.lines[j].borrow_mut().flags |= LineFlags::AddTrans.bits()}
                        }
                    }
                }
                line.special = 0;
            }
            _ => {}
        }
    }

    fn make_skill(flags: i32) -> u16 {
        let mut res: u16 = 0;
        if (flags & 1) != 0 { res |= 1+2}
        if (flags & 2) != 0 { res |= 4}
        if (flags & 4) != 0 { res |= 8+16}
        res
    }
    
    fn load_things(&mut self, map: &WADLevel, game: &Game) {
        let thing_count = map.things.len();

        self.map_things_converted.resize_with(thing_count, || MapThing::default());
        for i in 0..thing_count {
            let mut mapthing = &mut self.map_things_converted[i];
            let mut flags = map.things[i].options;

            mapthing.gravity = 1.;
            mapthing.conversation = 0;
            mapthing.skill_filter = Self::make_skill(flags as i32);
            mapthing.class_filter = 0xffff;
            mapthing.render_style = 19; //TODO StyleCount
            mapthing.alpha = -1.;
            mapthing.health = 1.;
            mapthing.float_bob_phase = -1;

            mapthing.pos.x = map.things[i].x as f64;
            mapthing.pos.y = map.things[i].y as f64;
            mapthing.angle = map.things[i].angle;
            mapthing.ed_num = map.things[i].type_;
            //TODO mapthing.info

            
            //TODO is in an ifnded NO_EDATA
            if mapthing.info.is_some() && mapthing.info.as_deref().unwrap().special == SpecialMapThings::SMT_EDThing.into() {
                Self::process_eternity_map_thing(self);
            }
            else {
                flags &= !MapThingFlags::SkillMask.bits() as i16;
                mapthing.flags = (((flags & 0xf) | 0x7e0) as i16) as u32;
                match game.game_info.game_type {
                    GameType::Strife => {
                        mapthing.flags &= !MapThingFlags::Ambush.bits();
                        if flags as u32 & MapThingFlags::SShadow.bits() != 0 {mapthing.flags |= MapThingFlags::Shadow.bits()}
                        if flags as u32 & MapThingFlags::SAltShadow.bits() != 0 {mapthing.flags |= MapThingFlags::AltShadow.bits()}
                        if flags as u32 & MapThingFlags::SStandStill.bits() != 0 {mapthing.flags |= MapThingFlags::StandStill.bits()}
                        if flags as u32 & MapThingFlags::SAmbush.bits() != 0 {mapthing.flags |= MapThingFlags::Ambush.bits()}
                        if flags as u32 & MapThingFlags::SFriendly.bits() != 0 {mapthing.flags |= MapThingFlags::Friendly.bits()}
                    }
                    _ => {
                        if flags as u32 & MapThingFlags::BBadEditorCheck.bits() != 0 {flags &= 0x1f}
                        if flags as u32 & MapThingFlags::BNotDeathMatch.bits() != 0 {mapthing.flags |= MapThingFlags::DeathMatch.bits()}
                        if flags as u32 & MapThingFlags::BNotCooperative.bits() != 0 {mapthing.flags |= MapThingFlags::Cooperative.bits()}
                        if flags as u32 & MapThingFlags::BFriendly.bits() != 0 {mapthing.flags |= MapThingFlags::Friendly.bits()}
                    }
                }
                if flags as u32 & MapThingFlags::BNotSingle.bits() != 0 {mapthing.flags |= MapThingFlags::Single.bits()}
            }
        }
    }

    fn process_eternity_map_thing(&self) {
        //TODO
    }
    
    //This is for hexen map formats
    fn load_things2(&mut self, map: &WADLevel, game: &Game) {
        //TODO
    }

    fn set_side_num(&self, sidedef: &mut SideDefIndex, side_count: u16) {
        if side_count == 0xffff /*NO_INDEX */ {*sidedef = -1;}
        else if (side_count as usize) < self.level.elements.sides.len() {
            *sidedef = side_count as i32;
            match self.side_temp[side_count as usize] {
                SideInit::A(mut t) => {t.map = side_count as u32}
                SideInit::B(_t) => {/*TODO What*/}
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
                SideInit::B(_t) => {/*TODO What*/}
            }
        }
        else {
            match side_i {
                SideInit::A(mut t) => { t.special = 0;}
                SideInit::B(_t) => {/*TODO What*/}
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
                SideInit::B(_t) => {/*TODO What*/}
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
        let position_names = ["floor", "ceiling"];
    
        if truncate { name.pop(); }
        let mut texture: TextureID = self.tex_manager.check_for_texture(name, TextureType::Flat, TexManFlags::Overridable.bits() | TexManFlags::TryAny.bits());

        if !texture.exists() {
            if track.contains_key(name) && track.get(name).unwrap().count <= 20 /*Missing_texture_warn_limit */ {
                println!("unkown {:?} texture {:?} in sector {}", position_names[pos], name, index);
            }
            
            texture = self.tex_manager.get_default_texture();
        }
        sector.set_texture(pos, texture);
    }

    fn set_texture_side_blend(&self, side: &mut Side, pos: usize, blend: &mut u32, name: &String) {
        let mut texture: TextureID;
        *blend = Self::color_map_for_name(&self, name);
        if *blend == 0 {
            texture = self.tex_manager.check_for_texture(name, TextureType::Wall, TexManFlags::Overridable.bits() | TexManFlags::TryAny.bits());
            if !texture.exists() {
                let mut short = name.clone();
                short.truncate(8);
                *blend = u32::from_str_radix(&short.to_string(), 16).unwrap();

                texture = TextureID::new();
            }
            else {*blend = 0;}
        }
        else { texture = TextureID::new(); }
        side.set_texture(pos, texture);
    }

    fn set_texture_side(&self, side: &mut Side, pos: usize, name: &String, track: &MissingTextureTracker) {        
        let position_names = ["top", "middle", "bottom"];
        let side_names = ["first", "second"];

        let mut texture = self.tex_manager.check_for_texture(name, TextureType::Wall, TexManFlags::Overridable.bits() | TexManFlags::TryAny.bits());

        if !texture.exists() {
            if track.contains_key(name) && track.get(name).unwrap().count <= 20 /*Missing_texture_warn_limit */ {
                //error for all things that use this side
                for i in 0..self.level.lines.len() {
                    for j in 0..2 {
                        if self.level.lines[i].borrow().sidedef[j] == side.udmf_index { //TODO not completly sure about the udfm_index
                            println!("unkown {:?} texture {:?} on {} side of linedef {}", position_names[pos], name, side_names[j], i);
                        }
                    }
                }
            }
            texture = self.tex_manager.get_default_texture();
        }
        side.set_texture(pos, texture);
    }

    fn set_texture_side_no_error(&self, side: &mut Side, pos: usize, color: &mut u32, name: &String, valid_color: &mut bool, is_fog: bool) {
        let mut texture: TextureID;

        *valid_color = false;
        texture = self.tex_manager.check_for_texture(name, TextureType::Wall, TexManFlags::Overridable.bits() | TexManFlags::TryAny.bits());
        if !texture.exists() {


            if !name.starts_with("#") {
                *color = u32::from_str_radix(&name.to_string(), 16).unwrap();
                texture = TextureID::new();
                // *valid_color = *stop == 0 && 
                //TODO weird stuff here?
                return
            }
            else {
                let mut reduced = name.clone();
                reduced.remove(0);
                reduced.truncate(7);
                let len = name.len();
                texture = TextureID::new();
                
                if len >= 7 {
                    let mut name2:Vec<u8> = vec![b'\0'; 7];
                    for (i, c) in reduced.chars().enumerate() {
                        match c {
                            '0'..='9' | 'a'..='f' |'A'..='F' => {name2[i] = c as u8}
                            _ => {name2[i] = b'0'}
                        }
                    }
                    let mut factor;
                    if len == 7 {factor = 0} else {
                        let val = ((name2[6] as u8) & 223 - ('A' as u8)) as i32;
                        factor = i32::clamp(val, 0, 25);
                    }
                    let bluestr = String::from_utf8(name2.clone()[4..6].to_vec()).unwrap();
                    let greenstr = String::from_utf8(name2.clone()[2..4].to_vec()).unwrap();
                    let redstr = String::from_utf8(name2.clone()[0..2].to_vec()).unwrap();
                    let blue = i32::from_str_radix(&bluestr, 16).unwrap();
                    let green = i32::from_str_radix(&greenstr, 16).unwrap();
                    let red = i32::from_str_radix(&redstr, 16).unwrap();

                    if !is_fog {
                        if factor == 0 {
                            *valid_color = false;
                            return
                        }
                        factor = factor * 255 / 25;
                    }
                    else {
                        factor = 0;
                    }
                    *color = u32::from_le_bytes([factor as u8, red as u8, green as u8, blue as u8]);
                    texture = TextureID::new();
                    *valid_color = true;
                    return
                }
            }
        }
        side.set_texture(pos, texture);
    }

    fn process_side_textures(&self, check_transfer_map:bool, side: &mut Side, sector: SectorIndex, imsd: &MapSideDef, missing_textures: &MissingTextureTracker, sideinit: &SideInit) {
        match sideinit {
            SideInit::A(t) => {
                let sec = &self.level.sectors[sector as usize];
                let match_special = num::FromPrimitive::from_i16(t.special);
                match match_special {
                    Some(ActionSpecials::TransferHeights) => {
                        if sector != -1 {
                            Self::set_texture_side_blend(&self, side, Sides::Bottom.bits() as usize, &mut sec.borrow_mut().bottom_map, &imsd.bottom_texture);
                            Self::set_texture_side_blend(&self, side, Sides::Mid.bits() as usize, &mut sec.borrow_mut().mid_map, &imsd.middle_texture);
                            Self::set_texture_side_blend(&self, side, Sides::Top.bits() as usize, &mut sec.borrow_mut().top_map, &imsd.top_texture);
                        }
                    }

                    Some(ActionSpecials::StaticInit) => {
                        let mut color:u32 = u32::from_le_bytes([0,255,255,255]);
                        let mut fog:u32 = 0;
                        let mut color_good: bool = false;
                        let mut fog_good: bool = false;

                        Self::set_texture_side_no_error(self,side, Sides::Bottom.bits() as usize, &mut fog, &imsd.bottom_texture, &mut fog_good, true);
                        Self::set_texture_side_no_error(self,side, Sides::Top.bits() as usize, &mut color, &imsd.top_texture, &mut color_good, false);
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

                    Some(ActionSpecials::SectorSet3dFloor) => {
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
                    Some(ActionSpecials::TranslucentLine) => {
                        let mut lump_num = -1;
                        if self.file_system.is_some() {
                            lump_num = self.file_system.as_ref().unwrap().check_num_for_name(&imsd.middle_texture);
                        }
                        if check_transfer_map {
                            if imsd.middle_texture.starts_with("TRANMAP") {
                                side.set_texture(Sides::Mid.bits() as usize, TextureID::new());
                            }
                            else if lump_num > 0 && self.file_system.as_ref().unwrap().file_length(lump_num) == 65536 {
                                //TODO


                                side.set_texture(Sides::Mid.bits() as usize, TextureID::new());
                            }
                            else {
                                Self::set_texture_side(self,side, Sides::Mid.bits() as usize, &imsd.middle_texture, missing_textures);
                            }
                        }
                        else {
                            Self::set_texture_side(self,side, Sides::Mid.bits() as usize, &imsd.middle_texture, missing_textures);
                        }
                        Self::set_texture_side(self,side, Sides::Top.bits() as usize, &imsd.top_texture, missing_textures);
                        Self::set_texture_side(self,side, Sides::Bottom.bits() as usize, &imsd.bottom_texture, missing_textures);
                    }
                    None => {
                        Self::set_texture_side(self,side, Sides::Mid.bits() as usize, &imsd.middle_texture, missing_textures);
                        Self::set_texture_side(self,side, Sides::Top.bits() as usize, &imsd.top_texture, missing_textures);
                        Self::set_texture_side(self,side, Sides::Bottom.bits() as usize, &imsd.bottom_texture, missing_textures);
                    }                 
                }
            }
            SideInit::B(t) => {}
        }
    }


    fn color_map_for_name(&self, name: &String) -> u32 {
        if name.starts_with("COLORMAP") {
            for i in (0..self.fake_color_maps.len() - 1).rev() {
                if !self.fake_color_maps[i].name.starts_with(name) { //TODO should be strncmp(name, fc.name, 8)
                    return i as u32
                }
            }
            if name.starts_with("WATERMAP") { //TODO should be strncmp(name, "WATERMAP", 8)
                return u32::from_le_bytes([128, 0, 0x4f, 0xa5]);
            }
        }
        0
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

#[derive(Default)]
pub struct MapThing {
    thing_id: i32,
    pos: Vector3<f64>,
    angle: i16,
    skill_filter: u16,
    class_filter: u16,
    ed_num: i16,
    flags: u32,
    special: i32,
    args: [i32;5],
    conversation: i32,
    gravity: f64,
    alpha: f64,
    fill_color: u32,
    scale: Vector2<f32>,
    health: f64,
    score: i32,
    pitch: i16,
    roll: i16,
    render_style: u32,
    float_bob_phase: i32,
    friendly_see_blocks: i32,
    arg_0_str: String,
    pub info: Option<Rc<DoomEternityEntry>>
}

#[derive(Default)]
pub struct DoomEternityEntry {
    type_: Rc<ClassActor>,
    special: i16,
    args_defined: i8,
    no_skill_flags: bool,
    args: [i32;5]
}

/*
TODO
*   MapLoader::LoadLevel() {
*      LoadBehavior();
*      T_LoadScripts();
*      Level->Behaviors.LoadDefaultModules();
*      LoadMapinfoACSLump();
*      LoadStrifeConversations();
* 
*      if (!textmap) {
*          LoadVertexes(); DONE
*          LoadLineDefs(); DONE
*          LoadSideDefs2(); DONE
*          FinishLoadingLineDefs(); DONE
*          LoadThings();
*      }
*      else {
*          ParseTextMap();
*      }
* 
*      CalcIndices();
*      PostProcessLevel();
* 
*      LoopSidedefs();
* 
*      if (something)  {
*           LoadExtendedNodes();
*           if (!NodesLoaded) {
*                LoadGLNodes();
*           }
*      }
* 
* 
*      LoadBlockMap();
*      LoadReject();
*      GroupLines();
*      FloodZones();
*      SetRenderSector();
*      FixMiniSegReferences();
*      FixHoles();
*      CalcIndices();
* 
*      CreateSections();
* 
*      SpawnSlopeMakers();
* 
*      Spawn3DFloors();
* 
*      SpawnThings();
* 
*      if (someasd) {
*           LoadLightMap();
*      }
* 
*      SpawnSpecials();
*      
*      otherstuff
*      
*      Level->levelMesh = new DoomLevelMesh(*Level);
* }
* 
*/