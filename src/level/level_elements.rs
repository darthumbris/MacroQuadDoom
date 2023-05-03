use crate::vector::{Vector2, Transform};
use super::level_lightmap::*;
use super::level_portal::*;
use super::level_mesh::SectorPlane;
use super::{PolyNode, MiniBSP, Angle, AutoMapLineStyle, BaseDecal, Part, PolyObj, LevelLocals};
use super::level_actor::Actor;

pub struct SubSector {
    sector: Box<Sector>,
    polys: Box<PolyNode>,
    bsp: Box<MiniBSP>,
    first_line: Box<Seg>,
    render_sector: Box<Sector>,
    section: Box<Section>,
    subsector_num: i32,
    line_count: u32,
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

struct Line {
    v1: Vertex,
    v2: Vertex,
    delta: Vector2<f64>,

    flags: u32,
    flags2: u32,
    activation: u32,
    special: i32,
    args: [i32;5],
    alpha: f64,
    sidedef: [Box<Side>;2],
    bbox: [f64;4],
    front_sector: Box<Sector>,
    back_sector: Box<Sector>,
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
    sector: Box<Sector>, //sector sidedef is facing
    attached_decals: Box<BaseDecal>,
    textures: [Part;3],
    linedef: Box<Line>,
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

pub struct Sector {
    splane: [Splane;2],
    level: Box<LevelLocals>,
    e: ExtSector,
    floorplane: SectorPlane,
    ceilingplane: SectorPlane,
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
    sound_target,
    thing_list: Vec<Actor>,
    gravity: f64, //1.0 is normal?

    floor_data,
    ceiling_data,
    lighting_data,

    interpolations: [;4],

    touching_thing_list: Vec<SecNode>,

    sec_act_target,

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

struct Seg {

}

struct Section {

}

struct ExtSector {}

struct LevelElements {
    vertexes: Vec<Vertex>,
    sectors: Vec<Sector>,
    extsectors: Vec<ExtSector>,
    line_buffer: Vec<Box<Line>>,
    subsector_buffer: Vec<Box<SubSector>>,
    lines: Vec<Line>,
    sides: Vec<Side>,
    seg_buffer: Vec<Box<Seg>>,
    segs: Vec<Seg>,
    subsectors: Vec<SubSector>,
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
    sector: &Sector,
    thing: Actor,
    thing_prev: Box<SecNode>,
    thing_next: Box<SecNode>,
    sec_prev: Box<SecNode>,
    sec_next: Box<SecNode>,
    visited: bool
}


//TODO TObjPtr and DObject what they do
