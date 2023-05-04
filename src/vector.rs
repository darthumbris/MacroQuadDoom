
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

pub struct Angle<T> {
    degrees: T
}

pub struct Transform {
    x_offset: f64,
    y_offset: f64,
    base_y_offset: f64,
    x_scale: f64,
    y_scale: f64,
    angle: Angle<f64>,
    base_angle: Angle<f64>
}

impl Vector2<f32> {
    
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
}

impl Vector2<f64> {
    
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
}