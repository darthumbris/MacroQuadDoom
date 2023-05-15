use std::rc::{Rc, Weak};
use std::cell::RefCell;

use bitflags::bitflags;


use crate::vector::{Vector2, Transform};
use super::level_lightmap::*;
use super::level_portal::*;
use super::level_mesh::SectorPlane;
use super::{PolyNode, MiniBSP, AutoMapLineStyle, BaseDecal, Part, PolyObj, LevelLocals};
use super::level_actor::Actor;
use super::level_texture::{TextureID, TextureManipulation};

pub type SectorIndex = i32; //if -1 -> does not exist (NULL)
pub type ExtSectorIndex = i32; //if -1 -> does not exist (NULL)
pub type LineIndex = i32; //if -1 -> does not exist (NULL)
pub type SubSectorIndex = i32; //if -1 -> does not exist (NULL)
pub type LineDefIndex = i32; //if -1 -> does not exist (NULL)
pub type SegIndex = i32; //if -1 -> does not exist (NULL)
pub type SideDefIndex = i32; //if -1 -> does not exist (NULL)
pub type VertexIndex = i32;

#[derive(Clone)]
pub struct SubSector {
    pub sector: SectorIndex,
    polys: Box<PolyNode>,
    bsp: Box<MiniBSP>,
    pub first_line: Vec<SegIndex>,
    render_sector: SectorIndex,
    section: Box<Section>,
    pub subsector_num: i32,
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

    pub flags: u32,
    pub flags2: u32,
    activation: u32,
    pub special: i32,
    pub args: [i32;5],
    pub alpha: f64,
    pub sidedef: [SideDefIndex;2],
    bbox: [f64;4],
    pub front_sector: SectorIndex,
    pub back_sector: SectorIndex,
    valid_count: i32,
    lock_number: i32,
    pub portal_index: u32,
    pub portal_transfered: u32,
    auto_map_style: AutoMapLineStyle,
    health: i32,
    health_group: i32,
    pub line_num: i32,

    //TODO functions
}

impl Line {
    pub fn new() -> Line {
        Line { v1: Vertex::new(0, 0), v2: Vertex::new(0, 0), delta: Vector2::<f64>::new(), flags: 0, flags2: 0, activation: 0, special: 0, args: [0;5], alpha: 0., sidedef: [-1;2], bbox: [0.;4], front_sector: -1, back_sector: -1, valid_count: 0, lock_number: 0, portal_index: 0, portal_transfered: 0, auto_map_style: AutoMapLineStyle::new(), health: 0, health_group: 0, line_num: 0 }
    }

    pub fn adjust_line(&self) {
        println!("adjusting line TODO");
        //TODO
    }

    pub fn index(&self) -> i32 {
        self.line_num
    }

    pub fn delta(&self) -> Vector2<f64> {
        self.delta
    }
}

#[derive(Clone)]
pub struct Side {
    pub sector: SectorIndex, //sector sidedef is facing //geen option
    attached_decals: Option<BaseDecal>,
    textures: [Part;3],
    pub linedef: LineDefIndex, //is geen option
    pub left_side: u32,
    pub right_side: u32,
    pub texel_length: u16,
    light: i16,
    tier_lights: [i16;3],
    pub flags: u16,
    pub udmf_index: i32,
    light_head: Option<LightNode>,
    lightmap: Option<LightMapSurface>,
    segs: Vec<SegIndex>, //all segs in ascending order
    num_segs: i32,
    pub side_num: i32,

    //TODO functions, Part and BaseDecal struct
}

impl Side {
    pub fn new() -> Side {
        Side { sector: -1, attached_decals: None, textures: [Part::new(); 3], linedef: 0, left_side: 0, right_side: 0, texel_length: 0, light: 0, tier_lights: [0;3], 
            flags: 0, udmf_index: 0, light_head: None, lightmap: None, segs: vec![], num_segs: -1, side_num: 0 }
    }

    pub fn index(&self) -> i32{
        self.side_num
    }

    pub fn set_texture_x_offset(&mut self, offset: f64) {
        
    }

    pub fn set_texture_y_offset(&mut self, offset: f64) {

    }

    pub fn set_texture_x_scale(&mut self, scale: f64) {

    }

    pub fn set_texture_y_scale(&mut self, scale: f64) {

    }

    pub fn set_texture(&mut self, which: usize, tex: TextureID) {
        self.textures[which].texture = tex;
    }

    pub fn get_texture(&self, which: usize) -> TextureID {
        self.textures[which].texture
    }
}

#[derive(Clone, Debug)]
pub struct Vertex {
    p: Vector2<f64>,

    pub vertex_num: i32,
    view_angle: u32, //u32 angle_t
    angle_time: i32,
    dirty: bool,
    num_heights: i32,
    num_sectors: i32,
    sectors: Vec<SectorIndex>,
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

    pub fn fx(&self) -> f64 {
        self.p.x
    }

    pub fn fy(&self) -> f64 {
        self.p.y
    }
}

#[derive(Clone)]
pub struct Sector {
    splane: [Splane;2],
    pub level: Option<RefCell<Weak<RefCell<LevelLocals>>>>,
    pub e: ExtSectorIndex, //geen option
    pub floorplane: SectorPlane,
    pub ceilingplane: SectorPlane,
    center_spot: Vector2<f64>,
    lines: Vec<LineIndex>,
    pub height_sec: SectorIndex,

    pub sector_portal_thinglist: SecNode,
    pub touching_render_things: SecNode,

    special_colors: [PalEntry;5],
    additive_colors: [PalEntry;5],
    pub color_map: ColorMap,

    pub special: i32,
    sky: i32,
    valid_count: i32,

    //color maps
    pub bottom_map: u32,
    pub mid_map: u32,
    pub top_map: u32,

    trans_door: bool,
    pub light_level: i16,
    pub more_flags: u16,
    pub flags: u32,

    portals: [u32;2],
    portal_group: i32,

    pub sector_num: i32,

    //GL only stuff
    subsector_count: i32,
    reflect: [f32;2],
    trans_door_height: f64,
    subsectors: Vec<SubSectorIndex>, //TODO maybe smart pointers
    portals_fc: [SectorPortalGroup;2],

    vbo_index: [i32;4],
    ibo_index: [i32;4],
    vbo_height: [[f64;2];2],
    vbo_count: [i32;2],
    pub ibo_count: i32,

    has_light_map: bool,

    //Stuff not to do with renderer
    //TODO sound_target, 
    pub thing_list: Vec<Actor>,
    pub gravity: f64, //1.0 is normal?

    //TODO floor_data,
    //TODO ceiling_data,
    //TODO lighting_data,

    //TODO interpolations: [;4],

    pub touching_thing_list: Vec<SecNode>,

    //TODO sec_act_target,

    pub friction: f64,
    pub move_factor: f64,

    pub terrain_num: [i32;2],
    pub sec_name: String,
    pub sec_type: i16,
    
    sound_traversed: u8,
    stair_lock: i8,

    pub prev_sec: i32,
    pub next_sec: i32,

    damage_type: String,
    damage_amount: i32,
    damage_interval: i16,
    leaky_damage: i16,

    pub zone_number: u16,

    health_floor: i32,
    health_ceiling: i32,
    health_3d: i32,
    health_floor_group: i32,
    health_ceiling_group: i32,
    health_3d_group: i32,
}

bitflags! {
    pub struct SectorFlags: u32 {
   
        const Silent = 1;
        const NoFallingDamage = 2;
        const FloorDrop = 4;
        const NoRespawn = 8;
        const Friction = 16;
        const Push = 32;
        const SilentMove = 64;
        const DmgTerrainFx = 128;
        const EndGodMode = 256;
        const EndLevel = 512;
        const Hazard = 1024;
        const NoAttack = 2048;
        const Exit1 = 4096;
        const Exit2 = 8196;
        const KillMonsters = 16384;
        const WasSecret = 1 << 30;
        const Secret = 1 << 31;

        const DamageFlags = Self::EndGodMode.bits() | Self::EndLevel.bits() | Self::DmgTerrainFx.bits() | Self::Hazard.bits();
        const NoModify = Self::Secret.bits() | Self::WasSecret.bits();
        const SpecialFlags = Self::DamageFlags.bits() | Self::Friction.bits() | Self::Push.bits() | Self::Exit1.bits() | Self::Exit2.bits() | Self::KillMonsters.bits();
    }

    pub struct SectorMoreFlags: u16 {

        const FakeFloorOnly = 2;
        const ClipFakePlanes = 4;
        const NoFakeLight = 8;
        const IgnoreHeightSec = 16;
        const UnderWater = 32;
        const ForcedUnderWater = 64;
        const Drawn = 128;
        const Hidden = 256;
        const Overlapping = 512;
        const NoSkyWalls = 1024;
        const Lift = 2048;

    }

    pub struct LineFlags: u32 {
        const ClipMidTex = 0x00080000;
        const WrapMidTex = 0x00100000;
        const CheckSwitchRange = 0x00400000;
        const AddTrans = 0x00000400;	// additive translucency (can only be set internally)
    }

    pub struct Sides: u32 {
        const Top = 0;
        const Mid = 1;
        const Bottom = 2;
    }
}

impl Sector {
    pub fn new(e: ExtSectorIndex) -> Sector {
        Sector {e, floorplane: SectorPlane::new(), ceilingplane: SectorPlane::new(), splane: [Splane::new();2], level: None, center_spot: Vector2::<f64>::new(), lines: vec![], height_sec: -1, special_colors: [PalEntry::new(); 5], additive_colors: [PalEntry::new(); 5], color_map: ColorMap::new(), special: 0, sky: 0, valid_count: 0, bottom_map: 0, mid_map: 0, top_map: 0, trans_door: false, light_level: 0, more_flags: 0, flags: 0, portals: [0;2], portal_group: 0, sector_num: 0, subsector_count: 0, reflect: [0.;2], trans_door_height: 0., subsectors: vec![], portals_fc: [SectorPortalGroup::default(); 2], vbo_index: [0;4], ibo_index: [0;4], vbo_height: [[0.;2];2], vbo_count: [0;2], ibo_count: 0, has_light_map: false, thing_list: vec![], gravity: 0., touching_thing_list: vec![], friction: 0., move_factor: 0., terrain_num: [0;2], sec_name: String::new(), sec_type: 0, sound_traversed: 0, stair_lock: 0, prev_sec: 0, next_sec: 0, damage_type: String::new(), damage_amount: 0, damage_interval: 0, leaky_damage: 0, zone_number: 0, health_floor: 0, health_ceiling: 0, health_3d: 0, health_floor_group: 0, health_ceiling_group: 0, health_3d_group: 0, sector_portal_thinglist: SecNode::default(), touching_render_things: SecNode::default() }
        // Sector {e, floorplane: SectorPlane::new(), ceilingplane: SectorPlane::new(), splane: [Splane::new();2], level: None, height_sec: -1, sector_portal_thinglist: SecNode::default(), touching_render_things: SecNode::default(), color_map: ColorMap::new(), special: 0, bottom_map: 0, mid_map: 0, top_map: 0, light_level: 0, more_flags: 0, flags: 0, sector_num: 0, ibo_count: 0, thing_list: vec![], gravity: 0., friction: 0., move_factor: 0., terrain_num: [0;2], sec_name: String::new(), sec_type: 0, prev_sec: 0, next_sec: 0, zone_number: 0, health_floor: 0, health_ceiling: 0, health_3d: 0, health_floor_group: 0, health_ceiling_group: 0, health_3d_group: 0 }
    }
    
    pub fn get_texture(&self, pos: usize) -> TextureID {
        self.splane[pos].texture
    }

    pub fn set_plane_tex_z(&mut self, pos: usize, val: f64, dirtify: Option<bool>) {
        println!("setting plane_ tex_z: {}", val);
        self.splane[pos].tex_z = val;
        if dirtify.unwrap_or(false) { Self::set_all_vertices_dirty(self);}
        Self::check_overlap(self)
    }

    fn check_overlap(&mut self) {
        if self.splane[SectorE::Floor as usize].tex_z > self.splane[SectorE::Ceiling as usize].tex_z && 
                !self.floorplane.is_slope() && !self.ceilingplane.is_slope() {
            self.more_flags |= SectorMoreFlags::Overlapping.bits();
        }
        else { self.more_flags &= !SectorMoreFlags::Overlapping.bits(); }
    }

    fn set_all_vertices_dirty(&mut self) {
        Self::set_vertices_dirty(self);
        let binding = &mut self.level.as_mut().unwrap().borrow_mut().upgrade().unwrap();
        let ext_sector = &binding.borrow_mut().elements.extsectors[self.e as usize];
        let mut bind = binding.borrow_mut();
        let sectors = &mut bind.sectors;
        for i in 0..ext_sector.fake_floor.len() {
            let index = ext_sector.fake_floor[i] as usize;
            sectors[index].borrow_mut().set_all_vertices_dirty();
        }

        for i in 0..ext_sector.x_floor.attached.len() {
            let index = ext_sector.x_floor.attached[i] as usize;
            sectors[index].borrow_mut().set_all_vertices_dirty();
        }
    }

    fn set_vertices_dirty(&mut self) {
        let binding = &mut self.level.as_mut().unwrap().borrow_mut().upgrade().unwrap();
        let ext_sector = &binding.borrow_mut().elements.extsectors[self.e as usize];
        let vertices = &mut binding.borrow_mut().vertexes;
        for i in 0..ext_sector.vertices.len() {
            let index = ext_sector.vertices[i] as usize;
            vertices[index].borrow_mut().dirty = true;
        }
    }

    pub fn get_plane_tex_z(&self, pos: usize) -> f64 {
        // println!("get_plane_tex_z: {}", self.splane[pos].tex_z);
        self.splane[pos].tex_z
    }

    pub fn set_alpha(&mut self, pos: usize, val: f64) {
        self.splane[pos].alpha = val;
    }

    pub fn set_x_scale(&mut self, pos: usize, val: f64) {
        self.splane[pos].x_form.x_scale = val;
    }

    pub fn set_y_scale(&mut self, pos: usize, val: f64) {
        self.splane[pos].x_form.y_scale = val;
    }

    pub fn set_texture(&mut self, pos: usize, texure: TextureID) {
        //TODO
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
    pub v1: VertexIndex,
    pub v2: VertexIndex,
    sidedef: SideDefIndex,
    linedef: LineDefIndex,

    front_sector: SectorIndex,
    back_sector: SectorIndex,

    side_frac: f32,
    pub seg_num: i32,
}

#[derive(Clone)]
struct Section {

}

pub struct ExtSector {
    pub x_floor: XFloor,//3Dfloors
    pub fake_floor: Vec<SectorIndex>,
    mid_tex: MidTex,
    linked: LinkedSectors,
    pub vertices: Vec<VertexIndex>
}

impl ExtSector {
    pub fn new() -> ExtSector {
        ExtSector { x_floor: XFloor::new(), fake_floor: vec![], mid_tex: MidTex::new(), linked: LinkedSectors::new(), vertices: vec![] }
    }
}

pub struct XFloor {
    pub f_floors: Vec<Floor3D>,
    light_list: Vec<LightList>,
    pub attached: Vec<SectorIndex>
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

#[derive(Default)]
pub struct LevelElements {
    pub vertexes: Vec<Rc<RefCell<Vertex>>>,
    pub sectors: Vec<Rc<RefCell<Sector>>>,
    pub extsectors: Vec<ExtSector>,
    line_buffer: Vec<Box<Line>>,
    subsector_buffer: Vec<Box<SubSector>>,
    pub lines: Vec<Rc<RefCell<Line>>>, //TODO this maybe the correct one
    pub sides: Vec<Rc<RefCell<Side>>>,
    seg_buffer: Vec<Box<Seg>>,
    pub segs: Vec<Rc<RefCell<Seg>>>,
    pub subsectors: Vec<Rc<RefCell<SubSector>>>,
    pub nodes: Vec<Rc<RefCell<Node>>>,
    game_subsectors: Vec<SubSector>,
    game_nodes: Vec<Node>,
    head_game_node: Rc<Node>,
    reject_matrix: Vec<u8>,
    z_zones: Vec<Zone>,
    poly_objects: Vec<PolyObj>,
    sector_portals: Vec<SectorPortal>,
    line_portals: Vec<LinePortal>,
}

struct Zone {}

#[derive(Default)]
pub struct Node {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,

    len: f32,

    //union / enum
    //bbox[2][4]
    //nb_bbox[2][4]
    

    //union enum
    //void* children[2]
    //int children[2]
    pub node_num: i32
}

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

#[derive(Default, Clone)]
pub struct SecNode{
    sector: Option<Box<Sector>>,
    thing: Actor,
    thing_prev: Option<Box<SecNode>>,
    thing_next: Option<Box<SecNode>>,
    sec_prev: Option<Box<SecNode>>,
    sec_next: Option<Box<SecNode>>,
    visited: bool
}

#[derive(Default)]
pub struct SectorMarker {}

pub struct UDMFKeys {}


//TODO TObjPtr and DObject what they do
