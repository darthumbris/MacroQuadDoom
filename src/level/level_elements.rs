use crate::vector::{Vector2, Transform};
use super::level_lightmap::*;
use super::level_portal::*;
use super::level_mesh::SectorPlane;
use super::{PolyNode, MiniBSP, AutoMapLineStyle, BaseDecal, Part, PolyObj, LevelLocals};
use super::level_actor::Actor;
use super::level_texture::{TextureID, TextureManipulation};

pub type SectorIndex = i32; //if -1 -> does not exist (NULL)
pub type ExtSectorIndex = i32; //if -1 -> does not exist (NULL)

pub struct SubSector {
    pub sector: SectorIndex,
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
    pub front_sector: SectorIndex,
    pub back_sector: SectorIndex,
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
    pub sector: SectorIndex, //sector sidedef is facing //geen option
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

pub struct Vertex {
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

    pub fn new(x: i16, y: i16) -> Vertex {
        let p: Vector2<f64> = Vector2 { x: x as f64 / 65536., y: y as f64 / 65536. };
        Vertex { p, vertex_num: 0, view_angle: 0, angle_time: 0, dirty: true, num_heights: 0, num_sectors: 0, sectors: vec![], height_list: vec![] }
    }
}

// #[derive(Clone, Copy)]
pub struct Sector {
    splane: [Splane;2],
    level: Option<Box<LevelLocals>>,
    pub e: ExtSectorIndex, //geen option
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
    pub fn get_texture(&self, pos: usize) -> TextureID {
        self.splane[pos].texture
    }

    pub fn new(e: ExtSectorIndex) -> Sector {
        Sector {e, floorplane: SectorPlane::new(), ceilingplane: SectorPlane::new(), splane: [Splane::new();2], level: None, center_spot: Vector2::<f64>::new(), lines: vec![], height_sec: None, sector_portal_thinglist: SecNode::default(), touching_render_things: SecNode::default(), special_colors: [PalEntry::new(); 5], additive_colors: [PalEntry::new(); 5], color_map: ColorMap {}, special: 0, sky: 0, valid_count: 0, bottom_map: 0, mid_map: 0, top_map: 0, trans_door: false, light_level: 0, more_flags: 0, flags: 0, portals: [0;2], portal_group: 0, sector_num: 0, subsector_count: 0, reflect: [0.;2], trans_door_height: 0., subsectors: vec![], portals_fc: [SectorPortalGroup::default(); 2], vbo_index: [0;4], ibo_index: [0;4], vbo_height: [[0.;2];2], vbo_count: [0;2], ibo_count: 0, has_light_map: false, thing_list: vec![], gravity: 0., touching_thing_list: vec![], friction: 0., move_factor: 0., terrain_num: [0;2], sec_name: String::new(), sec_type: 0, sound_traversed: 0, stair_lock: 0, prev_sec: 0, next_sec: 0, damage_type: String::new(), damage_amount: 0, damage_interval: 0, leaky_damage: 0, zone_number: 0, health_floor: 0, health_ceiling: 0, health_3d: 0, health_floor_group: 0, health_ceiling_group: 0, health_3d_group: 0 }
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

pub struct Seg {
    pub v1: Box<Vertex>,
    pub v2: Box<Vertex>,
    sidedef: Box<Side>,
    linedef: Box<Line>,

    front_sector: SectorIndex,
    back_sector: SectorIndex,

    side_frac: f32,
    seg_num: i32,
}

struct Section {

}

pub struct ExtSector {
    pub x_floor: XFloor,//3Dfloors
    fake_floor: Vec<SectorIndex>,
    mid_tex: MidTex,
    linked: LinkedSectors
}

impl ExtSector {
    pub fn new() -> ExtSector {
        ExtSector { x_floor: XFloor::new(), fake_floor: vec![], mid_tex: MidTex::new(), linked: LinkedSectors::new() }
    }
}

pub struct XFloor {
    pub f_floors: Vec<Floor3D>,
    light_list: Vec<LightList>,
    attached: Vec<SectorIndex>
}

impl XFloor {
    pub fn new() -> XFloor {
        XFloor { f_floors: vec![], light_list: vec![], attached: vec![] }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Floor3D {
    pub model: SectorIndex
}
struct LightList {}

struct MidTex {
    floor: Plane,
    ceiling: Plane
}

impl MidTex {
    pub fn new() -> MidTex {
        MidTex { floor: Plane::new(), ceiling: Plane::new() }
    }
}

struct Plane {
    attached_sectors: Vec<Box<Sector>>,
    attached_lines: Vec<Box<Line>>
}

impl Plane {
    pub fn new() -> Plane {
        Plane { attached_sectors: vec![], attached_lines: vec![] }
    }
}

struct Linked {
    floor: LinkedSectors,
    ceiling: LinkedSectors
}

struct LinkedSectors {
    sectors: Vec<LinkedSector>
}

impl LinkedSectors {
    pub fn new() -> LinkedSectors {
        LinkedSectors { sectors: vec![] }
    }
}

#[derive(Default)]
struct LinkedSector {
    sector: Option<Box<Sector>>,
    type_: i32
}

pub struct LevelElements {
    vertexes: Vec<Vertex>,
    pub sectors: Vec<Sector>,
    pub extsectors: Vec<ExtSector>,
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

#[derive(Clone, Copy)]
pub struct Splane {
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

impl Splane {
    pub fn new() -> Splane {
        Splane { x_form: Transform::new(), flags: 0, lights: 0, alpha: 0., tex_z: 0., glow_color: PalEntry::new(), glow_height: 0., texture: TextureID{tex_num: 0}, texture_fx: TextureManipulation {} }
    }
}

#[derive(Default)]
struct SecNode{
    sector: Option<Box<Sector>>,
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
