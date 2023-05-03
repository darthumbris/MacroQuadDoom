use super::level_elements::SubSector;
use super::level_elements::Sector;
use super::level_elements::Side;

struct LightMaps {
    surfaces: Vec<LightMapSurface>,
    tex_coords: Vec<f32>,
    tex_count: i32,
    tex_size: i32,
    tex_data: Vec<u16>,
}

pub struct LightMapSurface {
    type_: SurfaceType,
    subsector: Box<SubSector>,
    side: Box<Side>,
    control_sector: Box<Sector>,
    light_map_num: u32,
    tex_coords: Vec<f32>
}

pub enum SurfaceType {
    STNull,
	STMiddleWall,
	STUpperWall,
	STLowerWall,
	STCeiling,
	STFloor
}

pub struct LightNode {}

struct LightProbes<'a> {
    light_probes: Vec<LightProbe>,
    min_x: i32,
    min_y: i32,
    width: i32,
    height: i32,
    cell_size: i32, // = 32
    cells: Vec<LightProbeCell<'a>>
}

struct LightProbe {
    x: f32,
    y: f32,
    z: f32,
    red: f32,
    green: f32,
    blue: f32
}

struct LightProbeCell<'a> {
    first_probe: Option<&'a [LightProbe]>,
    probe_count: i32
}

struct DynamicLights {}

struct LightMode {}

pub struct ColorMap {}

pub union PalEntry {
    a: ARGB,
    d: u32
}

struct ARGB {
    a: u8,
    r: u8,
    g: u8,
    b: u8
}