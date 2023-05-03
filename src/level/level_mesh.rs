use crate::vector::Vector3;

use super::LevelLocals;
use super::level_elements::{Sector, Side};
use super::level_lightmap::SurfaceType;

struct LevelMesh {
    vertices: Vec<Vector3<f32>>,
    uv_index: Vec<i32>,
    elements: Vec<u32>,
    mesh_surfaces: Vec<i32>,

    surfaces: Vec<Surface>
}

struct Surface {
    type_: SurfaceType,
    type_index: i32,
    vert_count: i32,
    start_vert_index: u32,
    plane: SectorPlane,
    control_sector: Sector,
    b_sky: bool
}

pub struct SectorPlane {
    normal: Vector3<f64>,
    d: f64,
    neg_ic: f64, // negative iC because that also saves a negation in all methods using this.

    // the plane is defined as a*x + b*y + c*z + d = 0
	// ic is 1/c, for faster Z calculations
}

impl LevelMesh {
    pub fn new(doom_map: &LevelLocals) -> Self {
        Self {
            vertices: vec![],
            uv_index: vec![],
            elements: vec![],
            mesh_surfaces: vec![],
            surfaces: vec![]
        };
        for i in 0..doom_map.elements.sides.len() {
            Self::create_side_surfaces(Self, doom_map, doom_map.elements.sides[i]);
        }
        Self::create_subsector_surfaces(Self, doom_map);

        Self::create_uvs(Self);
    }

    //Functions for creating the mesh
    fn create_subsector_surfaces(&mut self, doom_map: &LevelLocals) {}

    fn create_ceiling_surfaces() {}

    fn create_floor_surfaces() {}

    fn create_side_surfaces(&mut self, doom_map: &LevelLocals, side: &Side) {

    }

    fn create_uvs(&mut self) {

        for i in 0..self.surfaces.len() {
            let s = self.surfaces[i];
            let vert_count = s.vert_count;
            let pos = s.start_vert_index;
            let verts = &self.vertices[pos as usize];

            for j in 0..vert_count {
                self.uv_index.push(j);
            }

            if s.type_ == SurfaceType::STFloor || s.type_ == SurfaceType::STCeiling {
                for j in 2..vert_count {
                    if !Self::is_degenerate(verts[0], verts[j - 1], verts[j]) {
                        self.elements.push(pos);
                        self.elements.push(pos + j - 1);
                        self.elements.push(pos + j);
                        self.mesh_surfaces.push(i32::from(i));
                    }
                }
            }
            else if s.type_ == SurfaceType::STMiddleWall || s.type_ == SurfaceType::STLowerWall || s.type_ == SurfaceType::STUpperWall {
                if !Self::is_degenerate(verts[0], verts[1], verts[2]) {
                    self.elements.push(pos + 0);
                    self.elements.push(pos + 1);
                    self.elements.push(pos + 2);
                    self.mesh_surfaces.push(i32::from(i));
                }
                if !Self::is_degenerate(verts[1], verts[2], verts[3]) {
                    self.elements.push(pos + 3);
                    self.elements.push(pos + 2);
                    self.elements.push(pos + 1);
                    self.mesh_surfaces.push(i32::from(i));
                }
            }
        }
    }

    
    //Functions for checking the surfaces/sector
    fn is_top_side_sky() -> bool {}
    
    fn is_top_side_visible() -> bool {}
    
    fn is_bottom_side_visible() -> bool {}
    
    fn is_sky_sector() -> bool {}
    
    fn is_control_sector() -> bool {}

    
    fn to_plane(p0: &Vector3<f32>, p1: &Vector3<f32>, p2: &Vector3<f32>) -> SectorPlane {
        // let n: Vector3<f32> = ((p1 - p0) ^ (p2 - p1)) //
        // ^ is for cross product, | is for dot product
    }

    fn to_f32_vector2() {}
    
    fn to_f32_vector3() {}
    
    fn to_f32_vector4() {}

    //to check if the triangle is degenerate (zero cross product for two sides)
    fn is_degenerate(v0: &Vector3<f32>, v1: &Vector3<f32>, v2: &Vector3<f32>) -> bool {
        let ax = v1.x - v0.x;
        let ay = v1.y - v0.y;
        let az = v1.z - v0.z;
        let bx = v2.x - v0.x;
        let by = v2.y - v0.y;
        let bz = v2.z - v0.z;
        let crossx = ay * bz - az * by;
        let crossy = az * bx - ax * bz;
        let crossz = ax * by - ay * bx;
        let crosslengthsqr = crossx * crossx + crossy * crossy + crossz * crossz;
        let limit: f32 = 1.e-6;
        return crosslengthsqr <= limit;
    }
}