use super::level_elements::SubSector;
use super::level_elements::Sector;
use super::level_elements::Side;
use std::rc::Rc;

#[derive(Default)]
pub struct LightMaps {
    _surfaces: Vec<LightMapSurface>,
    _tex_coords: Vec<f32>,
    _tex_count: i32,
    _tex_size: i32,
    _tex_data: Vec<u16>,
}

#[derive(Clone)]
pub struct LightMapSurface {
    _type_: SurfaceType,
    _subsector: Box<SubSector>,
    _side: Box<Side>,
    _control_sector: Box<Sector>,
    _light_map_num: u32,
    _tex_coords: Vec<f32>
}

#[derive(PartialEq, Clone, Debug)]
pub enum SurfaceType {
    STNull,
	STMiddleWall,
	STUpperWall,
	STLowerWall,
	STCeiling,
	STFloor
}

#[derive(Clone)]
pub struct LightNode {}

#[derive(Default)]
pub struct LightProbes {
    _light_probes: Vec<LightProbe>,
    _min_x: i32,
    _min_y: i32,
    _width: i32,
    _height: i32,
    _cell_size: i32, // = 32
    _cells: Vec<LightProbeCell>
}

struct LightProbe {
    _x: f32,
    _y: f32,
    _z: f32,
    _red: f32,
    _green: f32,
    _blue: f32
}

struct LightProbeCell {
    _first_probe: Option<Rc<[LightProbe]>>,
    _probe_count: i32
}

// struct DynamicLights {}

// struct LightMode {}

#[derive(Clone)]
pub struct ColorMap {
    pub light_color: PalEntry,
    pub fade_color: PalEntry,
    pub desaturation: u8,
    pub blend_factor: u8,
    pub fog_density: u16
}

impl ColorMap {
    pub fn new() -> ColorMap {
        ColorMap { light_color: PalEntry::new(), fade_color: PalEntry::new(), desaturation: 0, blend_factor: 0, fog_density: 0 }
    }
}

#[derive(Clone, Copy)]
pub enum PalEntry {
    Argb(Argb),
    D(u32)
}

impl PalEntry {
    pub fn new() -> PalEntry {
        PalEntry::D(0)
    }

    pub fn new_neg() -> PalEntry {
        PalEntry::D(u32::max_value())
    }

    pub fn new_rgb(r: u8, g: u8, b: u8) -> PalEntry {
        PalEntry::Argb(Argb{ a: 0, r, g, b })
    }

    pub fn d(&self) -> u32 {
        match self {
            PalEntry::D(t) => {return t.to_owned()}
            PalEntry::Argb(t) => {return u32::from_le_bytes([t.a, t.r, t.g, t.b])}
        }
    }

    pub fn argb(&self) -> Argb {
        match self {
            PalEntry::D(t) => {
                let argb = t.to_le_bytes();
                let a = argb[0];
                let r = argb[1];
                let g = argb[2];
                let b = argb[3];
                return Argb{a, r, g, b}
            }
            PalEntry::Argb(t) => {return t.to_owned()}
        }
    }

    pub fn set_rgb(&mut self, other: u32) {
        match self {
            PalEntry::D(t) => {
                *t = other & 0xffffff;
            }
            PalEntry::Argb(t) => {
                let argb = other.to_le_bytes();
                *t = Argb{a: argb[0] & 0xff, r: argb[1] & 0xff, g: argb[2] & 0xff, b: argb[3] & 0xff};
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Argb {
    a: u8,
    r: u8,
    g: u8,
    b: u8
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