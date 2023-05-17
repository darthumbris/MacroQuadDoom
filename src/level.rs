use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::parser::parse_level::WADLevelLinedef;
use crate::vector::Vector2;
use bitflags::bitflags;
use num_derive::{FromPrimitive};
use num_enum::IntoPrimitive;

mod level_mesh;
mod level_elements;
mod level_lightmap;
mod level_portal;
mod level_actor;
pub mod level_texture;
mod level_light;
mod level_bsp;
mod level_poly;
pub mod level_load;
mod level_behavior;

use level_portal::*;
use level_elements::*;
use level_lightmap::*;
use level_mesh::LevelMesh;
use level_texture::*;
use level_light::*;
use level_actor::*;
use level_bsp::*;
use level_poly::*;

//TODO split this up in multiple structs (level stats, music, lights etc)
//TODO give everything types
//TODO see what is needed etc
//TODO reduce all the reference/Box etc and instead use something like keeping a vector with the indexes or somethign

// #[derive(DerefMut)]
#[derive(Default)]
pub struct LevelLocals {
    //TODO level,

    pub process_mask: PortalBits,
    pub found_portals: Vec<Box<LinePortal>>,
    pub groups_to_check: Vec<i32>,

    //level elements
    pub elements: LevelElements,

    //lightmaps
    pub lm: LightMaps,
    pub lp: LightProbes,

    //portal info
    pub portal_info: PortalInfo,
    //TODO sections: SectionContainer,
    pub canvas_tex_info: CanvasTextureInfo,
    pub local_event_manager: Rc<EventManager>,
    pub aabb_tree: Rc<AABBTree>,
    pub level_mesh: Rc<LevelMesh>,

    pub health_groups: HashMap<i32, HealthGroup>,

    pub block_map: BlockMap,
    pub poly_block_map: Vec<Box<PolyBlock>>,
    pub udmf_keys: [HashMap<i32, UDMFKeys>;4],


    pub load_sectors: Vec<Sector>,
    pub load_lines: Vec<Line>,
    pub load_sides: Vec<Side>,

    pub death_match_starts: Vec<PlayerStart>,
    pub player_starts: [PlayerStart; 8], //8 maxplayers
    pub all_player_starts: Vec<PlayerStart>,

    pub behaviors: BehaviorContainer,

    // tid_hash: [Rc<Actor>; 128],

    pub strife_dialogues: Vec<Box<StrifeDialogueNode>>,
    pub dialogue_roots: HashMap<i32, i32>,
    pub class_roots: HashMap<String, i32>, //Fname?
    pub bot_info: CajunMaster,

    pub ii_compatflags: i32,
	pub ii_compatflags2: i32,
	pub ib_compatflags: i32,
	pub i_compatflags: i32,
	pub i_compatflags2: i32,

	pub sector_marker: Rc<SectorMarker>,

	pub md5: [u8; 16],			// for savegame validation. If the MD5 does not match the savegame won't be loaded.
	pub time: i32,			// time in the hub
	pub maptime: i32,			// time in the map
	pub totaltime: i32,		// time in the game
	pub starttime: i32,
	pub partime: i32,
	pub sucktime: i32,
	pub spawnindex: u32,

	pub info: Rc<LevelInfo>,
	pub cluster: i32,
	pub cluster_flags: i32,
	pub level_num: i32,
	pub lump_num: i32,
    pub level_name: String,
    pub map_name: String,
    pub next_map: String,
    pub next_secret_map: String,
    pub author_name: String,
    pub f1_pic: String,
	//TODO translator: Box<Translator>,
	//TODO map_type: MapType,
	pub tag_manager: TagManager,
    //TODO interpolator: Interpolator,

	pub shader_start_time: u64,

	//TODO body_que,
	//TODO automap,
	pub body_que_slot: i32,

	pub players: [Rc<Player>;8], //8 max players


    pub num_map_sections: i32,

    flags: u32,
    flags2: u32,
    pub flags3: u32,

    fade_to_color: u32,
    outside_fog_color: u32,

    pub hazard_color: u32,
    pub hazard_flash_color: u32,

    pub music: String,
    pub music_order: i32,
    pub cd_track: i32,
    pub cd_id: u32,
    
    pub sky_texture1: TextureID,
    pub sky_texture2: TextureID,

    pub sky_speed1: f32,
    pub sky_speed2: f32,

    pub sky_pos1: f64,
    pub sky_pos2: f64,

    pub hw_sky_pos1: f32,
    pub hw_sky_pos2: f32,

    pub sky_stretch: bool,

    pub total_secrets: i32,
    pub found_secrets: i32,

    pub total_items: i32,
    pub found_items: i32,

    pub total_monster: i32,
    pub killed_monsters: i32,

    pub map_velocity: f64,
    pub avg_velocity: f64,

    pub gravity: f64,
    pub air_control: f64,
    pub air_friction: f64,
    pub air_supply: i32,
    pub default_environment: i32,

    pub sequence_list_head: Rc<SequenceNode>,

    //particles
    pub oldest_particle: u32,
    pub active_particles: u32,
    pub inactive_particles: u32,
    pub particles: Vec<Particle>,
    pub particles_in_subsec: Vec<u16>,
    pub thinkers: ThinkerCollection,

    pub scrolls: Vec<Vector2<f64>>,

    pub wall_vert_light: i8,
    pub wall_hori_light: i8,

    pub from_snapshot: bool,
    pub has_height_sectors: bool,
    pub has_dynamic_lights: bool,
    pub frozen_state: i32,

    pub team_damage: f64,

    pub fog_density: i32,
    pub outside_fog_density: i32,
    pub sky_fog: i32,

    pub death_sequence: String, //fname?
    pub pixel_stretch: f32,
    pub music_volume: f32,

    pub light_mode: LightMode,
    pub bright_fog: bool,
    pub light_additive_surfaces: bool,
    pub no_texture_fill: bool,
    pub impact_decal_count: i32,

    pub lights: Rc<DynamicLights>,

    //TODO corpse_queue,
    //TODO fraggle_script_thinker,
    //TODO acs_thinker,

    //TODO spot_state,

    pub sky_flat_num: TextureID
}

impl DerefMut for LevelLocals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

impl Deref for LevelLocals {
    type Target = LevelElements;
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl LevelLocals {

    pub fn translate_sector_special(&self, _special: i16) -> i32 {
        //TODO implement this
        0
    }

    pub fn translate_linedef(&self, _line: &Line, _linedef: &WADLevelLinedef, _line_index: i32) {
        //TODO
    }

    pub fn sector_has_tag(&self, _index: usize, _tag: i16) -> bool {
        //TODO
        false
    }

    pub fn line_has_id(&self, line: i32, tag: i32) -> bool {
        //TODO
        self.tag_manager.line_has_id(line, tag)
    }
}

#[derive(Default)]
pub struct TagManager {
    pub start_for_sector: Vec<i32>,
    pub start_for_line: Vec<i32>,
    pub all_ids: Vec<TagItem>
} //TODO

impl TagManager {
    pub fn add_sector_tag(&self, _i: usize, _tag: i16) {
        //TODO
    }

    pub fn add_line_id(&self, _i: usize, _tag: u16) {
        //TODO
    }

    pub fn line_has_id(&self, line: i32, tag: i32) -> bool {
        if self.line_has_ids(line) {
            let mut ndx = self.start_for_line[line as usize] as usize;
            while self.all_ids[ndx].tag == line {
                if self.all_ids[ndx].tag == tag {return true}
                ndx += 1;
            }
        }
        false
    }

    fn line_has_ids(&self, sector: i32) -> bool {
        sector >= 0 && sector < self.start_for_line.len() as i32 && self.start_for_line[sector as usize] >= 0
    }
}

pub struct TagItem {
    pub target: i32,
    pub tag: i32,
    pub next_tag: i32
}



#[derive(Clone, Copy)]
pub struct Part {
    pub texture: TextureID,
    //TODO
}

impl Part {
    pub fn new() -> Part {
        Part { texture: TextureID { tex_num: 0 } }
    }
}

#[derive(Clone)]
pub struct BaseDecal {}

pub struct AutoMapLineStyle {}

impl AutoMapLineStyle {
    pub fn new() -> AutoMapLineStyle {
        AutoMapLineStyle {  }
    }
}

pub struct Particle {}

#[derive(Default)]
pub struct LevelInfo {}

bitflags! {
    pub struct LevelFlags: u32 {
   
        const SndSeqTotalCtrl = 0x00001000;
        const HasFadeTable = 0x00000008;
        const Level2ClipMidTex = 0x00000200;
        const Level2WrapMidTex = 0x00000400;
        const Level2CheckSwitchRange = 0x00000800;
        //TODO add all other values
    }

    pub struct MapThingFlags: u32 {
        const SkillMask = 0x0007;
        const SkillShift = 1;

        const Ambush = 0x0008;
        const Dormant = 0x0010;

        const ClassMask = 0x00e0;
        const ClassShift = 5;

        const Single = 0x0100;
        const Cooperative = 0x0200;
        const DeathMatch = 0x0400;

        const Shadow = 0x0800;
        const AltShadow = 0x1000;
        const Friendly = 0x2000;
        const StandStill = 0x4000;
        const StrifeSomething = 0x8000;

        const Secret = 0x080000;
        const NoInFighting = 0x100000;
        const NoCount = 0x200000;

        const BNotSingle = 0x0010;
        const BNotDeathMatch = 0x0020;
        const BNotCooperative = 0x0040;
        const BFriendly = 0x0080;
        const BBadEditorCheck = 0x0100;

        const SStandStill = 0x0008;
        const SAmbush = 0x0020;
        const SFriendly = 0x0040;
        const SShadow = 0x0100;
        const SAltShadow = 0x0200;
    }
}

#[derive(FromPrimitive)]
pub enum ActionSpecials {
    TranslucentLine = 208,
    TransferHeights = 209,
    StaticInit = 190,
    SectorSet3dFloor = 160,
}

#[derive(IntoPrimitive)]
#[repr(i16)]
pub enum SpecialMapThings {
    Player1Start = 1,
	Player2Start,
	Player3Start,
	Player4Start,
	Player5Start,
	Player6Start,
	Player7Start,
	Player8Start,
	DeathmatchStart,
	SSeqOverride,
	PolyAnchor,
	PolySpawn,
	PolySpawnCrush,
	PolySpawnHurt,
	SlopeFloorPointLine,
	SlopeCeilingPointLine,
	SetFloorSlope,
	SetCeilingSlope,
	VavoomFloor,
	VavoomCeiling,
	CopyFloorPlane,
	CopyCeilingPlane,
	VertexFloorZ,
	VertexCeilingZ,
	EDThing,
}

//(special_number, min_script_args, max_script_args, args_on_line)
// type Special = (u32, i16, i16, i16);



//TODO ExtSector, Sector, Seg
