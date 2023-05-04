use super::level_elements::SubSector;
use super::level_elements::Sector;
use super::level_elements::Side;
use std::rc::Rc;

pub struct LightMaps {
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

#[derive(PartialEq)]
pub enum SurfaceType {
    STNull,
	STMiddleWall,
	STUpperWall,
	STLowerWall,
	STCeiling,
	STFloor
}

pub struct LightNode {}

pub struct LightProbes {
    light_probes: Vec<LightProbe>,
    min_x: i32,
    min_y: i32,
    width: i32,
    height: i32,
    cell_size: i32, // = 32
    cells: Vec<LightProbeCell>
}

struct LightProbe {
    x: f32,
    y: f32,
    z: f32,
    red: f32,
    green: f32,
    blue: f32
}

struct LightProbeCell {
    first_probe: Option<Rc<[LightProbe]>>,
    probe_count: i32
}

struct DynamicLights {}

struct LightMode {}

pub struct ColorMap {}

pub enum PalEntry {
    Argb {a: u8, r: u8, g: u8, b: u8},
    D(u32)
}

/*TODO maybe use Rc<T> instead of Box<T>, also maybe need to use RefCell<T>
    Rc<T> for multiple owners
    Box<T> and RefCell<T> single owner
    Box<T> and RefCell<T> allow mutable (RefCell<T> at runtime)

    probably want to use an Rc<T> that holds an RefCell<T>
    i.e. Rc<RefCell<T>>

    make sure not to use RefCell<Rc<T>> (might cause memory leaks)

    also check out Rc::clone vs Rc::downgrade (for ownership)
    to use Rc::downgrade (which will create a Weak<T>) use the Rc::upgrade (which will return an Option<Rc<T>>)

    for multithreaded Mutex<T> replaces RefCell<T>
*/