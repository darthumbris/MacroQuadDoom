
#[derive(Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

#[derive(Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

#[derive(Clone, Copy)]
pub struct Angle<T> {
    degrees: T
}

#[derive(Clone, Copy)]
pub struct Transform {
    x_offset: f64,
    y_offset: f64,
    base_y_offset: f64,
    x_scale: f64,
    y_scale: f64,
    angle: Angle<f64>,
    base_angle: Angle<f64>
}

impl Transform {
    pub fn new() -> Transform {
        Transform { x_offset: 0., y_offset: 0., base_y_offset: 0., x_scale: 0., y_scale: 0., angle: Angle { degrees: 0. }, base_angle: Angle { degrees: 0. } }
    }
}

impl Vector2<f32> {
    
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn new() -> Vector2<f32> {
        Vector2 { x: f32::default(), y: f32::default() }
    }
}

impl Vector2<f64> {
    
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn new() -> Vector2<f64> {
        Vector2 { x: f64::default(), y: f64::default() }
    }
}

impl Vector3<f32> {
    
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn new() -> Vector3<f32> {
        Vector3 { x: f32::default(), y: f32::default(), z: f32::default() }
    }

    pub fn xy(&self) -> Vector2<f32> {
        Vector2 { x: self.x, y: self.y }
    }
}

impl Vector3<f64> {
    
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn new() -> Vector3<f64> {
        Vector3 { x: f64::default(), y: f64::default(), z: f64::default() }
    }

    pub fn xy(&self) -> Vector2<f64> {
        Vector2 { x: self.x, y: self.y }
    }
}