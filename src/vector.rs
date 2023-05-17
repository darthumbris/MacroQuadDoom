use std::{f32::consts::PI as pi32, f64::consts::PI as pi64};


#[derive(Clone, Copy, Default, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle<T> {
    degrees: T
}

impl Angle<f64> {
    pub fn from_degrees(degrees: f64) -> Angle<f64> {
        Angle { degrees }
    }

    pub fn from_radians(radians: f64) -> Angle<f64> {
        Angle { degrees: radians * 180.0 / pi64 }
    }

    pub fn from_bam_u(f: u32) -> Angle<f64> {
        Angle { degrees: f as f64 * (90. / 0x40000000 as f64) }
    }

    pub fn from_bam_i(f: i32) -> Angle<f64> {
        Angle { degrees: f as f64 * (90. / 0x40000000 as f64) }
    }

    pub fn add(&mut self, angle: &Angle<f64>) {
        self.degrees += angle.degrees;
    }

    pub fn subtract(&mut self, angle: &Angle<f64>) {
        self.degrees -= angle.degrees;
    }

    pub fn subtract_result(&self, angle: &Angle<f64>) -> Angle<f64> {
        Angle { degrees: self.degrees - angle.degrees }
    }

    //What the fuck
    pub fn normalized360(&self) -> Angle<f64> {
        Angle { degrees: (90. / 0x40000000 as f64) * self.bams() as f64 }
    }

    fn normalized180(&self) -> Angle<f64> {
        Angle { degrees: (90. / 0x40000000 as f64) * (self.bams() as i32) as f64 }
    }

    fn bams(&self) -> u32 {
        ((((0x40000000 as f64 / 90.) * self.degrees).floor() + 0.5) as i32) as u32
    }

    pub fn abs_angle(angle_1: &Angle<f64>, angle_2: &Angle<f64>) -> Angle<f64> {
        let degrees = angle_1.subtract_result(angle_2).normalized180().degrees.abs();
        Angle { degrees}
    }

    pub fn to_vector(&self, length: f64) -> Vector2<f64> {
        Vector2 { x: length * f64::cos(self.degrees * pi64 / 180. ), y: length * f64::sin(self.degrees * pi64 / 180. ) }
    }
}

impl Angle<f32> {
    pub fn from_degrees(degrees: f32) -> Angle<f32> {
        Angle { degrees }
    }

    pub fn from_radians(radians: f32) -> Angle<f32> {
        Angle { degrees: radians * 180.0 / pi32 }
    }

    pub fn from_bam_u(f: u32) -> Angle<f32> {
        Angle { degrees: f as f32 * (90. / 0x40000000 as f32) }
    }

    pub fn from_bam_i(f: i32) -> Angle<f32> {
        Angle { degrees: f as f32 * (90. / 0x40000000 as f32) }
    }

    pub fn add(&mut self, angle: &Angle<f32>) {
        self.degrees += angle.degrees;
    }

    pub fn subtract(&mut self, angle: &Angle<f32>) {
        self.degrees -= angle.degrees;
    }

    pub fn subtract_result(&self, angle: &Angle<f32>) -> Angle<f32> {
        Angle { degrees: self.degrees - angle.degrees }
    }

    //What the fuck
    pub fn normalized360(&self) -> Angle<f32> {
        Angle { degrees: (90. / 0x40000000 as f32) * self.bams() as f32 }
    }

    fn normalized180(&self) -> Angle<f32> {
        Angle { degrees: (90. / 0x40000000 as f32) * (self.bams() as i32) as f32 }
    }

    fn bams(&self) -> u32 {
        ((((0x40000000 as f32 / 90.) * self.degrees).floor() + 0.5) as i32) as u32
    }

    pub fn abs_angle(angle_1: &Angle<f32>, angle_2: &Angle<f32>) -> Angle<f32> {
        let degrees = angle_1.subtract_result(angle_2).normalized180().degrees.abs();
        Angle { degrees}
    }

    pub fn to_vector(&self, length: f32) -> Vector2<f32> {
        Vector2 { x: length * f32::cos(self.degrees * pi32 / 180. ), y: length * f32::sin(self.degrees * pi32 / 180. ) }
    }
}

#[derive(Clone, Copy)]
pub struct Transform {
    pub x_offset: f64,
    pub y_offset: f64,
    pub base_y_offset: f64,
    pub x_scale: f64,
    pub y_scale: f64,
    pub angle: Angle<f64>,
    pub base_angle: Angle<f64>
}

impl Transform {
    pub fn new() -> Transform {
        Transform { x_offset: 0., y_offset: 0., base_y_offset: 0., x_scale: 0., y_scale: 0., angle: Angle { degrees: 0. }, base_angle: Angle { degrees: 0. } }
    }
}

impl Vector2<f32> {
    pub fn new() -> Vector2<f32> {
        Vector2 { x: f32::default(), y: f32::default() }
    }

    pub fn new_params(x: f32, y: f32) -> Vector2<f32> {
        Vector2 { x, y }
    }
    
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }


    pub fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0.
    }

    pub fn angle(&self) -> Angle<f32> {
        Angle::<f32>::from_radians(self.y.atan2(self.x))
    }

    pub fn add(&mut self, rhs: &Vector2<f32>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

    pub fn add_result(&self, rhs: &Vector2<f32>) -> Vector2<f32> {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }

    pub fn subtract(&mut self, rhs: &Vector2<f32>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }

    pub fn subtract_result(&self, rhs: &Vector2<f32>) -> Vector2<f32> {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Vector2<f64> {
    pub fn new() -> Vector2<f64> {
        Vector2 { x: f64::default(), y: f64::default() }
    }
    
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }


    pub fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0.
    }

    pub fn angle(&self) -> Angle<f64> {
        Angle::<f64>::from_radians(self.y.atan2(self.x))
    }

    pub fn add(&mut self, rhs: &Vector2<f64>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

    pub fn add_result(&self, rhs: &Vector2<f64>) -> Vector2<f64> {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }

    pub fn subtract(&mut self, rhs: &Vector2<f64>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }

    pub fn subtract_result(&self, rhs: &Vector2<f64>) -> Vector2<f64> {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Vector3<f32> {
    pub fn new() -> Vector3<f32> {
        Vector3 { x: f32::default(), y: f32::default(), z: f32::default() }
    }

    pub fn new_params(x: f32, y: f32, z: f32) -> Vector3<f32> {
        Vector3 { x, y, z }
    }
    
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }


    pub fn xy(&self) -> Vector2<f32> {
        Vector2 { x: self.x, y: self.y }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0. && self.z == 0.
    }

    pub fn cross(&self, rhs: &Vector3<f32>) -> Vector3<f32> {
        Vector3 { 
            x: self.y * rhs.z - self.z * rhs.y, 
            y: self.z * rhs.x - self.x * rhs.z, 
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn dot(&self, rhs: &Vector3<f32>) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn sub(&self, rhs: &Vector3<f32>) -> Vector3<f32> {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }

    pub fn add(&self, rhs: &Vector3<f32>) -> Vector3<f32> {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }

    pub fn unit(&self) -> Vector3<f32> {
        let mut len = Self::length(self);
        if len != 0.0 {len = 1. / len}
        Vector3 { x: self.x * len as f32, y: self.y  * len as f32, z: self.z  * len as f32 } 
    }
}

impl Vector3<f64> {
    pub fn new() -> Vector3<f64> {
        Vector3 { x: f64::default(), y: f64::default(), z: f64::default() }
    }

    pub fn new_params(x: f64, y: f64, z: f64) -> Vector3<f64> {
        Vector3 { x, y, z }
    }
    
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }


    pub fn xy(&self) -> Vector2<f64> {
        Vector2 { x: self.x, y: self.y }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0. && self.z == 0.
    }

    pub fn cross(&self, rhs: &Vector3<f64>) -> Vector3<f64> {
        Vector3 { 
            x: self.y * rhs.z - self.z * rhs.y, 
            y: self.z * rhs.x - self.x * rhs.z, 
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn dot(&self, rhs: &Vector3<f64>) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn sub(&self, rhs: &Vector3<f64>) -> Vector3<f64> {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }

    pub fn add(&self, rhs: &Vector3<f64>) -> Vector3<f64> {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }

    pub fn unit(&self) -> Vector3<f64> {
        let mut len = Self::length(self);
        if len != 0.0 {len = 1. / len}
        Vector3 { x: self.x * len, y: self.y  * len, z: self.z  * len } 
    }
}