use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::behavior::parse_level::WADLevelLinedef;
use crate::vector::Vector2;
use bitflags::bitflags;

mod level_mesh;
mod level_elements;
mod level_lightmap;
mod level_portal;
mod level_actor;
mod level_texture;
mod level_light;
mod level_bsp;
mod level_poly;
mod level_load;
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
    local_event_manager: Box<EventManager>,
    aabb_tree: Box<AABBTree>,
    // level_mesh: Box<LevelMesh>,

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

    tid_hash: [Box<Actor>; 128],

    strife_dialogues: Vec<Box<StrifeDialogueNode>>,
    dialogue_roots: HashMap<i32, i32>,
    class_roots: HashMap<String, i32>, //Fname?
    bot_info: CajunMaster,

    ii_compatflags: i32,
	ii_compatflags2: i32,
	ib_compatflags: i32,
	i_compatflags: i32,
	i_compatflags2: i32,

	sector_marker: Box<SectorMarker>,

	md5: [u8; 16],			// for savegame validation. If the MD5 does not match the savegame won't be loaded.
	time: i32,			// time in the hub
	maptime: i32,			// time in the map
	totaltime: i32,		// time in the game
	starttime: i32,
	partime: i32,
	sucktime: i32,
	spawnindex: u32,

	info: Box<LevelInfo>,
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

	players: [Box<Player>;8], //8 max players


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

    sequence_list_head: Box<SequenceNode>,

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

    lights: Box<DynamicLights>,

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
}

pub struct TagManager {} //TODO

impl TagManager {
    pub fn add_sector_tag(&self, i: usize, tag: i16) {
        //TODO
    }

    pub fn add_line_id(&self, i: usize, tag: u16) {

    }
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
}



//TODO ExtSector, Sector, Seg
