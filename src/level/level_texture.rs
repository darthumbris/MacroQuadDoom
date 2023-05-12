use std::collections::HashMap;
use bitflags::bitflags;

use crate::game::Game;

#[derive(Default)]
pub struct CanvasTextureInfo {

}

#[derive(PartialEq, Clone, Copy, Default)]
pub struct TextureID {
    pub tex_num: i32
}

impl TextureID {
    pub fn new() -> TextureID {
        TextureID { tex_num: 0 }
    }

    pub fn exists(&self) ->bool {
        //TODO
        false
    }

    pub fn get_index(&self) -> i32 {
        self.tex_num
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

pub struct FakeColorMap {
    pub name: String
}

pub struct TextureDescriptor {
    paletted: i32,
    front_sky_layer: i32,
    raw_texture: i32,
    hash_next: i32,
    flags: u64,
    texture: GameTexture
}

pub struct GameTexture {

}

impl GameTexture {
    pub fn is_valid(&self) -> bool {
        //TODO
        false
    }
}

pub struct TextureManager {
    //TODO
    textures: Vec<TextureDescriptor>,
    translation: Vec<i32>
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager { textures: vec![], translation: vec![] }
    }


    pub fn check_for_texture(&self, name: &String, tex_type: TextureType, flag: u32) -> TextureID {
        //TODO
        TextureID { tex_num: 0 }
    }

    pub fn get_default_texture(&self) -> TextureID {
        //TODO
        TextureID { tex_num: 0 }
    }

    pub fn get_game_texture(&self, tex_num: TextureID, animate: bool) -> Option<&GameTexture> {
        Self::internal_get_texture(&self, tex_num.get_index(), animate)
    }

    fn internal_get_texture(&self, tex_num: i32, animate: bool) -> Option<&GameTexture> {
        let tex_num: i32 = Self::resolve_texture_index(self, tex_num, animate);
        if tex_num == -1 {return None}
        Some(&self.textures[tex_num as usize].texture)
    }

    fn resolve_texture_index(&self, tex_num: i32, animate: bool) -> i32 {
        if tex_num as usize >= self.textures.len() {return -1}
        let mut tex_num = tex_num;
        if animate {tex_num = self.translation[tex_num as usize]}
        tex_num
    }
}

pub struct MapSideDef {
    pub top_texture: String,
    pub bottom_texture: String,
    pub middle_texture: String
}

pub enum TextureType {
    Any,
	Wall,
	Flat,
	Sprite,
	WallPatch,
	Build,		// no longer used but needs to remain for ZScript
	SkinSprite,
	Decal,
	MiscPatch,
	FontChar,
	Override,	// For patches between TX_START/TX_END
	Autopage,	// Automap background - used to enable the use of FAutomapTexture
	SkinGraphic,
	Null,
	FirstDefined,
	Special,
	SWCanvas,
}

bitflags! {
    pub struct TexManFlags: u32 {
        const TryAny = 1;
        const Overridable = 2;
        const ReturnFirst = 4;
        const AllowsSkins = 8;
        const ShortNameOnly = 16;
        const DontCreate = 32;
        const Localize = 64;
        const ForceLookUp = 128;
        const NoAlias = 256;
        const ReturnAll = 512;
    }
}