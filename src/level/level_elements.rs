use crate::vector::{Vector2, Transform};
use super::level_lightmap::*;
use super::level_portal::*;
use super::level_mesh::SectorPlane;
use super::{PolyNode, MiniBSP, AutoMapLineStyle, BaseDecal, Part, PolyObj, LevelLocals};
use super::level_actor::Actor;
use super::level_texture::{TextureID, TextureManipulation};

pub struct SubSector {
    pub sector: Option<Box<Sector>>,
    polys: Box<PolyNode>,
    bsp: Box<MiniBSP>,
    pub first_line: Vec<Box<Seg>>,
    render_sector: Box<Sector>,
    section: Box<Section>,
    subsector_num: i32,
    pub line_count: u32,
    flags : u16,
    map_section: i16,
    
    valid_count: i32,
    hacked: u8,

    portal_coverage: [PortalCoverage;2],
    light_map: [Box<LightMapSurface>;2]

    //fn buildpolybsp
    //fn index

    //TODO implement the functions for subsector

}

pub struct Line {
    pub v1: Vertex,
    pub v2: Vertex,
    delta: Vector2<f64>,

    flags: u32,
    flags2: u32,
    activation: u32,
    special: i32,
    args: [i32;5],
    alpha: f64,
    sidedef: [Option<Box<Side>>;2],
    bbox: [f64;4],
    pub front_sector: Option<Box<Sector>>,
    pub back_sector: Option<Box<Sector>>,
    valid_count: i32,
    lock_number: i32,
    portal_index: u32,
    portal_transfered: u32,
    auto_map_style: AutoMapLineStyle,
    health: i32,
    health_group: i32,
    line_num: i32,

    //TODO functions
}

pub struct Side {
    pub sector: Box<Sector>, //sector sidedef is facing //geen option
    attached_decals: Box<BaseDecal>,
    textures: [Part;3],
    pub linedef: Box<Line>, //is geen option
    left_side: u32,
    right_side: u32,
    texel_length: u16,
    light: i16,
    tier_lights: [i16;3],
    flags: u16,
    udmf_index: i32,
    light_head: Box<LightNode>,
    lightmap: Box<LightMapSurface>,
    segs: Vec<Box<Seg>>, //all segs in ascending order
    num_segs: i32,
    side_num: i32,

    //TODO functions, Part and BaseDecal struct
}

impl Side {
    pub fn v1(&self) -> &Vertex {
        if self.linedef.sidedef[0].is_some() {
            return &self.linedef.v1
        }
        else {
            return &self.linedef.v2
        }
    }

    pub fn v2(&self) -> &Vertex {
        if self.linedef.sidedef[0].is_some() {
            return &self.linedef.v2
        }
        else {
            return &self.linedef.v1
        }
    }

    pub fn index(&self) -> i32{
        self.side_num
    }
}

struct Vertex {
    p: Vector2<f64>,

    vertex_num: i32,
    view_angle: u32, //u32 angle_t
    angle_time: i32,
    dirty: bool,
    num_heights: i32,
    num_sectors: i32,
    sectors: Vec<Box<Sector>>,
    height_list: Vec<f32>,

    //TODO functions and constructors etc
}

impl Vertex {
    pub fn f_pos(&self) -> Vector2<f64>{
        self.p
    }
}

pub struct Sector {
    splane: [Splane;2],
    level: Box<LevelLocals>,
    pub e: Box<ExtSector>, //geen option
    pub floorplane: SectorPlane,
    pub ceilingplane: SectorPlane,
    center_spot: Vector2<f64>,
    lines: Vec<Box<Line>>,
    height_sec: Option<Box<Sector>>,

    sector_portal_thinglist: SecNode,
    touching_render_things: SecNode,

    special_colors: [PalEntry;5],
    additive_colors: [PalEntry;5],
    color_map: ColorMap,

    special: i32,
    sky: i32,
    valid_count: i32,

    //color maps
    bottom_map: u32,
    mid_map: u32,
    top_map: u32,

    trans_door: bool,
    light_level: i16,
    more_flags: u16,
    flags: u32,

    portals: [u32;2],
    portal_group: i32,

    sector_num: i32,

    //GL only stuff
    subsector_count: i32,
    reflect: [f32;2],
    trans_door_height: f64,
    subsectors: Vec<SubSector>, //TODO maybe smart pointers
    portals_fc: [SectorPortalGroup;2],

    vbo_index: [i32;4],
    ibo_index: [i32;4],
    vbo_height: [[f64;2];2],
    vbo_count: [i32;2],
    ibo_count: i32,

    has_light_map: bool,

    //Stuff not to do with renderer
    //TODO sound_target, 
    thing_list: Vec<Actor>,
    gravity: f64, //1.0 is normal?

    //TODO floor_data,
    //TODO ceiling_data,
    //TODO lighting_data,

    //TODO interpolations: [;4],

    touching_thing_list: Vec<SecNode>,

    //TODO sec_act_target,

    friction: f64,
    move_factor: f64,

    terrain_num: [i32;2],
    sec_name: String,
    sec_type: i16,
    
    sound_traversed: u8,
    stair_lock: i8,

    prev_sec: i32,
    next_sec: i32,

    damage_type: String,
    damage_amount: i32,
    damage_interval: i16,
    leaky_damage: i16,

    zone_number: u16,

    health_floor: i32,
    health_ceiling: i32,
    health_3d: i32,
    health_floor_group: i32,
    health_ceiling_group: i32,
    health_3d_group: i32,
}

impl Sector {
    pub fn GetTexture(&self, pos: usize) -> TextureID {
        self.splane[pos].texture
    }
}

#[derive(PartialEq)]
pub enum SectorE {
    Floor = 0,
    Ceiling = 1,
    WallTop,
    WallBottom,
    Sprites
}

struct Seg {
    pub v1: Box<Vertex>,
    pub v2: Box<Vertex>,
    sidedef: Box<Side>,
    linedef: Box<Line>,

    front_sector: Box<Sector>,
    back_sector: Option<Box<Sector>>,

    side_frac: f32,
    seg_num: i32,
}

struct Section {

}

struct ExtSector {
    pub x_floor: XFloor,//3Dfloors
    fake_floor: Vec<Box<Sector>>,
    mid_tex: MidTex,
    linked: LinkedSectors
}

struct XFloor {
    pub f_floors: Vec<Box<Floor3D>>,
    light_list: Vec<LightList>,
    attached: Vec<Box<Sector>>
}

struct Floor3D {
    pub model: Box<Sector>
}
struct LightList {}

struct MidTex {
    floor: Plane,
    ceiling: Plane
}

struct Plane {
    attached_sectors: Vec<Box<Sector>>,
    attached_lines: Vec<Box<Line>>
}

struct Linked {
    floor: LinkedSectors,
    ceiling: LinkedSectors
}

struct LinkedSectors {
    sectors: Vec<LinkedSector>
}

#[derive(Default)]
struct LinkedSector {
    sector: Option<Box<Sector>>,
    type_: i32
}

pub struct LevelElements {
    vertexes: Vec<Vertex>,
    sectors: Vec<Sector>,
    extsectors: Vec<ExtSector>,
    line_buffer: Vec<Box<Line>>,
    subsector_buffer: Vec<Box<SubSector>>,
    lines: Vec<Line>,
    pub sides: Vec<Side>,
    seg_buffer: Vec<Box<Seg>>,
    segs: Vec<Seg>,
    pub subsectors: Vec<SubSector>,
    nodes: Vec<Node>,
    game_subsectors: Vec<SubSector>,
    game_nodes: Vec<Node>,
    head_game_node: Box<Node>,
    reject_matrix: Vec<u8>,
    z_zones: Vec<Zone>,
    poly_objects: Vec<PolyObj>,
    sector_portals: Vec<SectorPortal>,
    line_portals: Vec<LinePortal>,
}

struct Zone {}

struct Node {}

struct Splane {
    x_form: Transform,
    flags: i32,
    lights: i32,
    alpha: f64,
    tex_z: f64,
    glow_color: PalEntry,
    glow_height: f32,
    texture: TextureID,
    texture_fx: TextureManipulation
}

struct SecNode{
    sector: Box<Sector>,
    thing: Actor,
    thing_prev: Box<SecNode>,
    thing_next: Box<SecNode>,
    sec_prev: Box<SecNode>,
    sec_next: Box<SecNode>,
    visited: bool
}

pub struct SectorMarker {}

pub struct UDMFKeys {}


//TODO TObjPtr and DObject what they do
