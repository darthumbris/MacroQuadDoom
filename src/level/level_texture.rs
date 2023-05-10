use std::collections::HashMap;

pub struct CanvasTextureInfo {

}

#[derive(PartialEq, Clone, Copy)]
pub struct TextureID {
    pub tex_num: i32
}

impl TextureID {
    pub fn exists(&self) ->bool {
        //TODO
        false
    }
}

#[derive(Clone, Copy)]
pub struct TextureManipulation {
    //TODO
}

#[derive(Default)]
pub struct MissingCount {
    pub count: i32
}

pub type MissingTextureTracker = HashMap<String, MissingCount>;

pub struct TextureManager {
    //TODO
}

impl TextureManager {
    pub fn check_for_texture(&self) -> TextureID {
        //TODO
        TextureID { tex_num: 0 }
    }

    pub fn get_default_texture(&self) -> TextureID {
        //TODO
        TextureID { tex_num: 0 }
    }
}

pub struct MapSideDef {
    pub top_texture: String,
    pub bottom_texture: String,
    pub middle_texture: String
}