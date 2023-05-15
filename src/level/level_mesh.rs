use std::borrow::Borrow;

use macroquad::models::Vertex;
use macroquad::prelude::{Mesh, Vec3, Vec2, Color};
use macroquad::texture::Texture2D;

use crate::vector::{Vector3, Vector2};

use super::LevelLocals;
use super::level_elements::{Sector, Side, SubSector, SectorE, SectorIndex, Sides};
use super::level_lightmap::SurfaceType;
use super::level_texture::{TextureID, TextureManager};

#[derive(Default, Debug)]
pub struct LevelMesh {
    pub vertices: Vec<Vector3<f32>>,
    pub uv_index: Vec<i32>,
    elements: Vec<u32>,
    mesh_surfaces: Vec<i32>,

    surfaces: Vec<Surface>
}

#[derive(Debug)]
struct Surface {
    type_: SurfaceType,
    type_index: i32,
    vert_count: u32,
    start_vert_index: u32,
    plane: SectorPlane,
    control_sector: SectorIndex,
    b_sky: bool
}

struct SidesS {
    v1_bottom: f64,
    v2_bottom: f64,
    v1_top: f64,
    v2_top: f64,
    back: Option<SidesBack>
}

#[derive(Clone, Copy)]
struct SidesBack {
    v1_top_back: f64,
    v1_bottom_back: f64,
    v2_top_back: f64,
    v2_bottom_back: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct SectorPlane {
    normal: Vector3<f64>,
    d: f64,
    neg_ic: f64, // negative iC because that also saves a negation in all methods using this.

    // the plane is defined as a*x + b*y + c*z + d = 0
	// ic is 1/c, for faster Z calculations
}

impl SectorPlane {
    pub fn new() -> SectorPlane {
        SectorPlane { normal: Vector3::<f64>::new(), d: 0., neg_ic: 0.}
    }
    
    pub fn z_at_point(&self, point: &Vector2<f32>) -> f64{
        // println!("z_at_point: {:?}", point);
        // println!("z_at_point: {}", (self.d + self.normal.x * f64::from(point.x) + self.normal.y * f64::from(point.y)) * self.neg_ic);
        (self.d + self.normal.x * f64::from(point.x) + self.normal.y * f64::from(point.y)) * self.neg_ic
    }
    
    pub fn flip_verts(&mut self) {
        self.normal = Vector3::<f64>{x: -self.normal.x, y: -self.normal.y, z: -self.normal.z};
        self.d = -self.d;
        self.neg_ic = -self.neg_ic;
    }

    pub fn is_slope(&self) -> bool {
        !self.normal.xy().is_zero()
    }

    pub fn set(&mut self, aa: f64, bb: f64, cc: f64, dd: f64) {
        self.normal.x = aa;
        self.normal.y = bb;
        self.normal.z = cc;
        self.d = dd;
        self.neg_ic = -1. / cc;
        // println!("set sectorplane: normal: {:?}, d: {}, neg: {}", self.normal, self.d, self.neg_ic);
    }
}

impl LevelMesh {
    pub fn new(doom_map: &LevelLocals, tex_man: &TextureManager) -> LevelMesh {
        let mut level_mesh = LevelMesh {
            vertices: vec![],
            uv_index: vec![],
            elements: vec![],
            mesh_surfaces: vec![],
            surfaces: vec![]
        };
        for i in 0..doom_map.elements.sides.len() {
            Self::create_side_surfaces(&mut level_mesh, doom_map, &doom_map.elements.sides[i].borrow_mut(), tex_man);
        }
        Self::create_subsector_surfaces(&mut level_mesh, doom_map);

        Self::create_uvs(&mut level_mesh);
        level_mesh
    }

    pub fn to_macro_mesh(&self, texture: Texture2D) -> Mesh {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u16> = vec![];

        for i in 0..self.vertices.len() {
            let vertex = self.vertices[i];
            // let uv_index = self.uv_index[i];
            let position: Vec3 = Vec3 { x: vertex.x, y: vertex.y, z: vertex.z};
            let uv: Vec2 = Vec2 { x: 0., y: 0. }; //TODO calc the uvs
            let color: Color = Color { r: 25., g: 45., b: 75., a: 1. };
            let vertex_macro: Vertex = Vertex { position, uv, color };
            indices.push(self.uv_index[i] as u16);
            vertices.push(vertex_macro);
        }

        Mesh { vertices, indices, texture: Some(texture) }
    }

    //Functions for creating the mesh
    fn create_subsector_surfaces(&mut self, doom_map: &LevelLocals) {
        for i in 0..doom_map.elements.subsectors.len() {
            let sub = &doom_map.elements.subsectors[i].borrow_mut();

            if sub.line_count < 3 {continue;}

            let sector = sub.sector;

            if sector == -1 || Self::is_control_sector(sector) {continue;} 
            let sec = sector;

            Self::create_floor_surfaces(self, doom_map, sub, sec, i as i32, false);
            Self::create_ceiling_surfaces(self, doom_map, sub, sec, i as i32, false);
            let cur_sec = &doom_map.elements.sectors[sec as usize].borrow_mut();
            let ext_sec = &doom_map.elements.extsectors[cur_sec.e as usize];
            for j in 0..ext_sec.x_floor.f_floors.len() {
                Self::create_floor_surfaces(self, doom_map, sub, ext_sec.x_floor.f_floors[j].model, i as i32, true);
                Self::create_ceiling_surfaces(self, doom_map, sub, ext_sec.x_floor.f_floors[j].model, i as i32, true);
            }
        }
    }

    fn create_ceiling_surfaces(&mut self, doom_map: &LevelLocals, sub: &SubSector, sec_index: SectorIndex, type_index: i32, is_3d_floor: bool) {
        let sector = &doom_map.elements.sectors[sec_index as usize].borrow_mut();
        let b_sky = Self::is_sky_sector(sector);
        let mut plane: SectorPlane;

        if !is_3d_floor {
            plane = sector.ceilingplane;
        }
        else {
            plane = sector.floorplane;
            plane.flip_verts();
        }

        let vert_count = sub.line_count;
        let start_vert_index = self.vertices.len() as u32;

        // self.vertices.resize_with((vert_count + start_vert_index) as usize, Default::default());
        self.vertices.resize((vert_count + start_vert_index) as usize, Vector3::<f32>::new());
        let verts = &mut self.vertices;

        for i in 0..vert_count as usize{
            let seg = &doom_map.elements.segs[sub.first_line[i] as usize].borrow_mut();
            let v1_seg = doom_map.vertexes[seg.v1 as usize].borrow_mut();
            let v1 = Self::to_f32_vector2(&v1_seg.f_pos());

            verts[i + start_vert_index as usize].x = v1.x;
            verts[i + start_vert_index as usize].y = v1.y;
            verts[i + start_vert_index as usize].z = plane.z_at_point(&verts[i + start_vert_index as usize].xy()) as f32;
        }
        let type_ = SurfaceType::STCeiling;
        let control_sector = if is_3d_floor {sec_index} else {-1};

        self.surfaces.push(Surface { type_, type_index, vert_count, start_vert_index, plane, control_sector, b_sky });
    }

    fn create_floor_surfaces(&mut self, doom_map: &LevelLocals, sub: &SubSector, sec_index: SectorIndex, type_index: i32, is_3d_floor: bool) {
        let sector = &doom_map.elements.sectors[sec_index as usize].borrow_mut();
        let b_sky = Self::is_sky_sector(sector);
        let mut plane: SectorPlane;

        if !is_3d_floor {
            plane = sector.floorplane;
        }
        else {
            plane = sector.ceilingplane;
            plane.flip_verts();
        }

        let vert_count = sub.line_count;
        let start_vert_index = self.vertices.len() as u32;

        self.vertices.resize((vert_count + start_vert_index) as usize, Vector3::<f32>::new());
        let verts = &mut self.vertices;

        for i in 0..vert_count as usize{
            let seg = &doom_map.elements.segs[sub.first_line[vert_count as usize - 1 - i] as usize].borrow_mut();
            let v1_seg = doom_map.vertexes[seg.v1 as usize].borrow_mut();
            let v1 = Self::to_f32_vector2(&v1_seg.f_pos());

            verts[i + start_vert_index as usize].x = v1.x;
            verts[i + start_vert_index as usize].y = v1.y;
            verts[i + start_vert_index as usize].z = plane.z_at_point(&verts[i + start_vert_index as usize].xy()) as f32;
        }

        let type_ = SurfaceType::STFloor;
        let control_sector = if is_3d_floor {sec_index} else {-1};

        self.surfaces.push(Surface { type_, type_index, vert_count, start_vert_index, plane, control_sector, b_sky });
    }

    fn create_side_surfaces(&mut self, doom_map: &LevelLocals, side: &Side, tex_man: &TextureManager) {
        let front_index = side.sector;
        let linedef = &doom_map.lines[side.linedef as usize].borrow_mut();
        let back_index = if linedef.front_sector == front_index {linedef.back_sector} else {linedef.front_sector};
        if Self::is_control_sector(front_index) { return }

        let line = linedef;
        let v1;
        let v2;
        if line.sidedef[0] >= 0 {
            v1 = Self::to_f32_vector2(&line.v1.borrow().f_pos());
            v2 = Self::to_f32_vector2(&line.v2.borrow().f_pos());
        }
        else {
            v1 = Self::to_f32_vector2(&line.v2.borrow().f_pos());
            v2 = Self::to_f32_vector2(&line.v1.borrow().f_pos());
        }

        let front_sector = &doom_map.elements.sectors[front_index as usize].borrow_mut();

        let v1_top = front_sector.ceilingplane.z_at_point(&v1);
        let v1_bottom = front_sector.floorplane.z_at_point(&v1);
        let v2_top = front_sector.ceilingplane.z_at_point(&v2);
        let v2_bottom = front_sector.floorplane.z_at_point(&v2);

        let mut sides: SidesS = SidesS { v1_bottom, v2_bottom, v1_top, v2_top, back: None};

        let type_index = side.index();

        // let dx = Vector2::<f32> {x: v2.x, y: v2.y};

        if back_index >= 0 {
            let back_sec = &doom_map.elements.sectors[back_index as usize].borrow_mut();
            Self::create_side_surfaces_back_sector(self, doom_map, back_sec, front_sector, &v1, &v2, type_index);

            let v1_top_back = back_sec.ceilingplane.z_at_point(&v1);
            let v1_bottom_back = back_sec.floorplane.z_at_point(&v1);
            let v2_top_back = back_sec.ceilingplane.z_at_point(&v2);
            let v2_bottom_back = back_sec.floorplane.z_at_point(&v2);

            sides.back = Some(SidesBack {v1_bottom_back, v1_top_back, v2_bottom_back, v2_top_back});

            if v1_top == v1_top_back && v1_bottom == v1_bottom_back && v2_top == v2_top_back && v2_bottom == v2_bottom_back {
                return
            }
            if v1_bottom < v1_bottom_back || v2_bottom < v2_bottom_back {
                Self::create_side_surfaces_bottom_seg(self, side, &v1, &v2, type_index, &mut sides, tex_man);
            }
            if v1_top > v1_top_back || v2_top > v2_top_back {
                let b_sky = Self::is_top_side_sky(front_sector, back_sec, side);
                Self::create_side_surfaces_top_seg(self, side, &v1, &v2, type_index, &mut sides, b_sky, tex_man);
            }
        }
        if back_index == -1 {
            Self::create_side_surfaces_middle_seg(self, &v1, &v2, type_index, &sides);
        }
    }

    fn create_side_surfaces_back_sector(&mut self, doom_map: &LevelLocals, back: &Sector, front: &Sector, v1: &Vector2<f32>, v2: &Vector2<f32>, type_index: i32) {
        let ext_sec = &doom_map.elements.extsectors[front.e as usize];
        for i in 0..ext_sec.x_floor.f_floors.len() {
            let x_floor = ext_sec.x_floor.f_floors[i];

            let mut both_sides = false;
            let ext_sec_back = &doom_map.elements.extsectors[back.e as usize];
            for j in 0..ext_sec_back.x_floor.f_floors.len() {
                if ext_sec_back.x_floor.f_floors[j] == x_floor {
                    both_sides = true;
                    break;
                }
            }
            if both_sides {
                continue;
            }

            let type_ = SurfaceType::STMiddleWall;
            let control_sector = x_floor.model;

            let mut verts: [Vector3<f32>;4] = [Vector3::<f32>::new();4];
            verts[0].x = v2.x;
            verts[2].x = v2.x;
            verts[0].y = v2.y;
            verts[2].y = v2.y;
            verts[1].x = v1.x;
            verts[3].x = v1.x;
            verts[1].y = v1.y;
            verts[3].y = v1.y;
            let model = &doom_map.elements.sectors[x_floor.model as usize].borrow_mut();
            verts[0].z = model.floorplane.z_at_point(v2) as f32;
            verts[1].z = model.floorplane.z_at_point(v1) as f32;
            verts[2].z = model.ceilingplane.z_at_point(v2) as f32;
            verts[3].z = model.ceilingplane.z_at_point(v1) as f32;

            let start_vert_index = self.vertices.len() as u32;
            let vert_count = 4;
            self.vertices.push(verts[0]);
            self.vertices.push(verts[1]);
            self.vertices.push(verts[2]);
            self.vertices.push(verts[3]);

            let plane = Self::to_plane(&verts[0], &verts[1], &verts[2]);
            self.surfaces.push(Surface { type_, type_index, vert_count, start_vert_index, plane, control_sector, b_sky: false});
        }
    }

    fn create_side_surfaces_middle_seg(&mut self, v1: &Vector2<f32>, v2: &Vector2<f32>, type_index: i32, sides: &SidesS) {
        

        let mut verts: [Vector3<f32>;4] = [Vector3::<f32>::new(); 4];
        verts[0].x = v1.x;
        verts[2].x = v1.x;
		verts[0].y = v1.y;
        verts[2].y = v1.y;
		verts[1].x = v2.x;
        verts[3].x = v2.x;
		verts[1].y = v2.y;
        verts[3].y = v2.y;
		verts[0].z = sides.v1_bottom as f32;
		verts[1].z = sides.v2_bottom as f32;
		verts[2].z = sides.v1_top as f32;
		verts[3].z = sides.v2_top as f32;

        let start_vert_index = self.vertices.len() as u32;
        let vert_count = 4;
        self.vertices.push(verts[0]);
        self.vertices.push(verts[1]);
        self.vertices.push(verts[2]);
        self.vertices.push(verts[3]);

        let plane = Self::to_plane(&verts[0], &verts[1], &verts[2]);
        let type_ = SurfaceType::STMiddleWall;
        let control_sector = -1;

        self.surfaces.push(Surface { type_, type_index, vert_count, start_vert_index, plane, control_sector, b_sky: false });

    }
    fn create_side_surfaces_top_seg(&mut self, side: &Side, v1: &Vector2<f32>, v2: &Vector2<f32>, type_index: i32, sides: &mut SidesS, b_sky: bool, tex_man: &TextureManager) {
        if b_sky || Self::is_top_side_visible(side, tex_man) {
            let mut verts: [Vector3<f32>;4] = [Vector3::<f32>::new(); 4];
            verts[0].x = v1.x;
            verts[2].x = v1.x;
            verts[0].y = v1.y;
            verts[2].y = v1.y;
            verts[1].x = v2.x;
            verts[3].x = v2.x;
            verts[1].y = v2.y;
            verts[3].y = v2.y;
            verts[0].z = sides.back.unwrap().v1_top_back as f32;
            verts[1].z = sides.back.unwrap().v2_top_back as f32;
            verts[2].z = sides.v1_top as f32;
            verts[3].z = sides.v2_top as f32;

            let start_vert_index = self.vertices.len() as u32;
            let vert_count = 4;
            self.vertices.push(verts[0]);
            self.vertices.push(verts[1]);
            self.vertices.push(verts[2]);
            self.vertices.push(verts[3]);

            let plane = Self::to_plane(&verts[0], &verts[1], &verts[2]);
            let type_ = SurfaceType::STUpperWall;
            let control_sector = -1;

            self.surfaces.push(Surface { type_, type_index, vert_count, start_vert_index, plane, control_sector, b_sky })
        }

        sides.v1_top = sides.back.unwrap().v1_top_back;
        sides.v2_top = sides.back.unwrap().v2_top_back;
    }
    fn create_side_surfaces_bottom_seg(&mut self, side: &Side, v1: &Vector2<f32>, v2: &Vector2<f32>, type_index: i32, sides: &mut SidesS, tex_man: &TextureManager) {
        if Self::is_bottom_side_visible(side, tex_man) {
            let mut verts: [Vector3<f32>;4] = [Vector3::<f32>::new(); 4];
            verts[0].x = v1.x;
            verts[2].x = v1.x;
            verts[0].y = v1.y;
            verts[2].y = v1.y;
            verts[1].x = v2.x;
            verts[3].x = v2.x;
            verts[1].y = v2.y;
            verts[3].y = v2.y;
            verts[0].z = sides.v1_bottom as f32;
            verts[1].z = sides.v2_bottom as f32;
            verts[2].z = sides.back.unwrap().v1_bottom_back as f32;
            verts[3].z = sides.back.unwrap().v2_bottom_back as f32;

            let start_vert_index = self.vertices.len() as u32;
            let vert_count = 4;
            self.vertices.push(verts[0]);
            self.vertices.push(verts[1]);
            self.vertices.push(verts[2]);
            self.vertices.push(verts[3]);

            let plane = Self::to_plane(&verts[0], &verts[1], &verts[2]);
            let type_ = SurfaceType::STLowerWall;
            let control_sector = -1;

            self.surfaces.push(Surface { type_, type_index, vert_count, start_vert_index, plane, control_sector, b_sky: false })
        }

        sides.v1_bottom = sides.back.unwrap().v1_bottom_back;
        sides.v2_bottom = sides.back.unwrap().v2_bottom_back;
    }

    fn create_uvs(&mut self) {

        for i in 0..self.surfaces.len() {
            let s = &self.surfaces[i];
            let vert_count = s.vert_count;
            let pos = s.start_vert_index;
            let verts = &self.vertices;

            for j in 0..vert_count {
                self.uv_index.push(j as i32);
            }

            if s.type_ == SurfaceType::STFloor || s.type_ == SurfaceType::STCeiling {
                for j in 2..vert_count as u32 {
                    if !Self::is_degenerate(&verts[pos as usize], &verts[(pos + j - 1) as usize], &verts[(pos + j) as usize]) {
                        self.elements.push(pos);
                        self.elements.push(pos + j - 1);
                        self.elements.push(pos + j);
                        self.mesh_surfaces.push(i as i32);
                    }
                }
            }
            else if s.type_ == SurfaceType::STMiddleWall || s.type_ == SurfaceType::STLowerWall || s.type_ == SurfaceType::STUpperWall {
                if !Self::is_degenerate(&verts[pos as usize], &verts[(pos + 1)as usize], &verts[(pos + 2) as usize]) {
                    self.elements.push(pos + 0);
                    self.elements.push(pos + 1);
                    self.elements.push(pos + 2);
                    self.mesh_surfaces.push(i as i32);
                }
                if !Self::is_degenerate(&verts[(pos + 1)as usize], &verts[(pos + 2) as usize], &verts[(pos + 3) as usize]) {
                    self.elements.push(pos + 3);
                    self.elements.push(pos + 2);
                    self.elements.push(pos + 1);
                    self.mesh_surfaces.push(i as i32);
                }
            }
        }
    }

    
    //Functions for checking the surfaces/sector
    fn is_top_side_sky(front_sector: &Sector, back_sector: &Sector, side: &Side) -> bool {
        Self::is_sky_sector(front_sector) && Self::is_sky_sector(back_sector)
    }
    
    fn is_top_side_visible(side: &Side, tex_man: &TextureManager) -> bool {
        let tex = tex_man.get_game_texture(side.get_texture(Sides::Top.bits() as usize), true);
        tex.is_some() && tex.unwrap().is_valid()
    }
    
    fn is_bottom_side_visible(side: &Side, tex_man: &TextureManager) -> bool {
        let tex = tex_man.get_game_texture(side.get_texture(Sides::Bottom.bits() as usize), true);
        tex.is_some() && tex.unwrap().is_valid()
    }
    
    fn is_sky_sector(sector: &Sector) -> bool {

        //TODO get sky_flat_num from somewehere? is global value in gzdoom
        let temp_sky_num = TextureID{tex_num: 0}; //TODO ^
        sector.get_texture(SectorE::Ceiling as usize) == temp_sky_num
    }
    
    fn is_control_sector(sector: SectorIndex) -> bool {false}

    
    fn to_plane(p0: &Vector3<f32>, p1: &Vector3<f32>, p2: &Vector3<f32>) -> SectorPlane {
        let p1sp0 = p1.sub(p0);
        let p2sp1 = p2.sub(p1);
        let cross = p1sp0.cross(&p2sp1);
        let n = cross.unit();
        let d = p0.dot(&n);
        let mut p = SectorPlane::new();
        // println!("to plane");
        // println!("p1sp0: {:?}, p2sp1: {:?}, cross: {:?}, n: {:?}, d: {:?}", p1sp0, p2sp1, cross, n, d);
        p.set(n.x as f64, n.y as f64, n.z as f64, d as f64);
        p
    }

    fn to_f32_vector2(v: &Vector2<f64>) -> Vector2<f32> {
        Vector2::<f32>::new_params(v.x as f32, v.y as f32)
    }
    
    fn to_f32_vector3(v: &Vector3<f64>) -> Vector3<f32> {
        Vector3::<f32>::new_params(v.x as f32, v.y as f32, v.z as f32)
    }

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
        let limit: f32 = 1.0e-6;
        return crosslengthsqr <= limit;
    }
}