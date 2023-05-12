use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::behavior::parse_level::WADLevelLinedef;
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

    process_mask: PortalBits,
    found_portals: Vec<Box<LinePortal>>,
    groups_to_check: Vec<i32>,

    //level elements
    pub elements: LevelElements,

    //lightmaps
    lm: LightMaps,
    lp: LightProbes,

    //portal info
    portal_info: PortalInfo,
    //TODO sections: SectionContainer,
    canvas_tex_info: CanvasTextureInfo,
    local_event_manager: Rc<EventManager>,
    aabb_tree: Rc<AABBTree>,
    pub level_mesh: Rc<LevelMesh>,

    health_groups: HashMap<i32, HealthGroup>,

    block_map: BlockMap,
    poly_block_map: Vec<Box<PolyBlock>>,
    udmf_keys: [HashMap<i32, UDMFKeys>;4],


    load_sectors: Vec<Sector>,
    load_lines: Vec<Line>,
    load_sides: Vec<Side>,

    death_match_starts: Vec<PlayerStart>,
    player_starts: [PlayerStart; 8], //8 maxplayers
    all_player_starts: Vec<PlayerStart>,

    behaviors: BehaviorContainer,

    // tid_hash: [Rc<Actor>; 128],

    strife_dialogues: Vec<Box<StrifeDialogueNode>>,
    dialogue_roots: HashMap<i32, i32>,
    class_roots: HashMap<String, i32>, //Fname?
    bot_info: CajunMaster,

    ii_compatflags: i32,
	ii_compatflags2: i32,
	ib_compatflags: i32,
	i_compatflags: i32,
	i_compatflags2: i32,

	sector_marker: Rc<SectorMarker>,

	md5: [u8; 16],			// for savegame validation. If the MD5 does not match the savegame won't be loaded.
	time: i32,			// time in the hub
	maptime: i32,			// time in the map
	totaltime: i32,		// time in the game
	starttime: i32,
	partime: i32,
	sucktime: i32,
	spawnindex: u32,

	info: Rc<LevelInfo>,
	cluster: i32,
	cluster_flags: i32,
	level_num: i32,
	lump_num: i32,
    level_name: String,
    map_name: String,
    next_map: String,
    next_secret_map: String,
    author_name: String,
    f1_pic: String,
	//TODO translator: Box<Translator>,
	//TODO map_type: MapType,
	pub tag_manager: TagManager,
    //TODO interpolator: Interpolator,

	shader_start_time: u64,

	//TODO body_que,
	//TODO automap,
	body_que_slot: i32,

	players: [Rc<Player>;8], //8 max players


    num_map_sections: i32,

    flags: u32,
    flags2: u32,
    flags3: u32,

    fade_to_color: u32,
    outside_fog_color: u32,

    hazard_color: u32,
    hazard_flash_color: u32,

    music: String,
    music_order: i32,
    cd_track: i32,
    cd_id: u32,
    
    sky_texture1: TextureID,
    sky_texture2: TextureID,

    sky_speed1: f32,
    sky_speed2: f32,

    sky_pos1: f64,
    sky_pos2: f64,

    hw_sky_pos1: f32,
    hw_sky_pos2: f32,

    sky_stretch: bool,

    total_secrets: i32,
    found_secrets: i32,

    total_items: i32,
    found_items: i32,

    total_monster: i32,
    killed_monsters: i32,

    map_velocity: f64,
    avg_velocity: f64,

    gravity: f64,
    air_control: f64,
    air_friction: f64,
    air_supply: i32,
    default_environment: i32,

    sequence_list_head: Rc<SequenceNode>,

    //particles
    oldest_particle: u32,
    active_particles: u32,
    inactive_particles: u32,
    particles: Vec<Particle>,
    particles_in_subsec: Vec<u16>,
    thinkers: ThinkerCollection,

    scrolls: Vec<Vector2<f64>>,

    wall_vert_light: i8,
    wall_hori_light: i8,

    from_snapshot: bool,
    has_height_sectors: bool,
    has_dynamic_lights: bool,
    frozen_state: i32,

    team_damage: f64,

    fog_density: i32,
    outside_fog_density: i32,
    sky_fog: i32,

    death_sequence: String, //fname?
    pixel_stretch: f32,
    music_volume: f32,

    light_mode: LightMode,
    bright_fog: bool,
    light_additive_surfaces: bool,
    no_texture_fill: bool,
    impact_decal_count: i32,

    lights: Rc<DynamicLights>,

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

    // pub fn new() -> LevelLocals {
    //     LevelLocals { process_mask: PortalBits::default(), found_portals: (), groups_to_check: (), elements: (), lm: (), lp: (), portal_info: (), canvas_tex_info: (), local_event_manager: (), aabb_tree: (), level_mesh: (), health_groups: (), block_map: (), poly_block_map: (), udmf_keys: (), load_sectors: (), load_lines: (), load_sides: (), death_match_starts: (), player_starts: (), all_player_starts: (), behaviors: (), tid_hash: (), strife_dialogues: (), dialogue_roots: (), class_roots: (), bot_info: (), ii_compatflags: (), ii_compatflags2: (), ib_compatflags: (), i_compatflags: (), i_compatflags2: (), sector_marker: (), md5: (), time: (), maptime: (), totaltime: (), starttime: (), partime: (), sucktime: (), spawnindex: (), info: (), cluster: (), cluster_flags: (), level_num: (), lump_num: (), level_name: (), map_name: (), next_map: (), next_secret_map: (), author_name: (), f1_pic: (), tag_manager: (), shader_start_time: (), body_que_slot: (), players: (), num_map_sections: (), flags: (), flags2: (), flags3: (), fade_to_color: (), outside_fog_color: (), hazard_color: (), hazard_flash_color: (), music: (), music_order: (), cd_track: (), cd_id: (), sky_texture1: (), sky_texture2: (), sky_speed1: (), sky_speed2: (), sky_pos1: (), sky_pos2: (), hw_sky_pos1: (), hw_sky_pos2: (), sky_stretch: (), total_secrets: (), found_secrets: (), total_items: (), found_items: (), total_monster: (), killed_monsters: (), map_velocity: (), avg_velocity: (), gravity: (), air_control: (), air_friction: (), air_supply: (), default_environment: (), sequence_list_head: (), oldest_particle: (), active_particles: (), inactive_particles: (), particles: (), particles_in_subsec: (), thinkers: (), scrolls: (), wall_vert_light: (), wall_hori_light: (), from_snapshot: (), has_height_sectors: (), has_dynamic_lights: (), frozen_state: (), team_damage: (), fog_density: (), outside_fog_density: (), sky_fog: (), death_sequence: (), pixel_stretch: (), music_volume: (), light_mode: (), bright_fog: (), light_additive_surfaces: (), no_texture_fill: (), impact_decal_count: (), lights: (), sky_flat_num: () }
    // }

    pub fn translate_sector_special(&self, special: i16) -> i32 {
        //TODO implement this
        0
    }

    pub fn translate_linedef(&self, line: &Line, linedef: &WADLevelLinedef, line_index: i32) {
        //TODO
    }

    pub fn sector_has_tag(&self, index: usize, tag: i16) -> bool {
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
    start_for_sector: Vec<i32>,
    start_for_line: Vec<i32>,
    all_ids: Vec<TagItem>
} //TODO

impl TagManager {
    pub fn add_sector_tag(&self, i: usize, tag: i16) {
        //TODO
    }

    pub fn add_line_id(&self, i: usize, tag: u16) {

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

struct AutoMapLineStyle {}

impl AutoMapLineStyle {
    pub fn new() -> AutoMapLineStyle {
        AutoMapLineStyle {  }
    }
}

struct Particle {}

#[derive(Default)]
struct LevelInfo {}

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
    SMT_Player1Start = 1,
	SMT_Player2Start,
	SMT_Player3Start,
	SMT_Player4Start,
	SMT_Player5Start,
	SMT_Player6Start,
	SMT_Player7Start,
	SMT_Player8Start,
	SMT_DeathmatchStart,
	SMT_SSeqOverride,
	SMT_PolyAnchor,
	SMT_PolySpawn,
	SMT_PolySpawnCrush,
	SMT_PolySpawnHurt,
	SMT_SlopeFloorPointLine,
	SMT_SlopeCeilingPointLine,
	SMT_SetFloorSlope,
	SMT_SetCeilingSlope,
	SMT_VavoomFloor,
	SMT_VavoomCeiling,
	SMT_CopyFloorPlane,
	SMT_CopyCeilingPlane,
	SMT_VertexFloorZ,
	SMT_VertexCeilingZ,
	SMT_EDThing,
}

//(special_number, min_script_args, max_script_args, args_on_line)
// type Special = (u32, i16, i16, i16);



//TODO ExtSector, Sector, Seg
