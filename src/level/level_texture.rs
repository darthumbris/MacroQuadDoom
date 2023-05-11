use std::collections::HashMap;
use bitflags::bitflags;

pub struct CanvasTextureInfo {

}

#[derive(PartialEq, Clone, Copy)]
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

pub struct TextureManager {
    //TODO
}

impl TextureManager {
    pub fn check_for_texture(&self, name: &String, tex_type: TextureType, flag: u32) -> TextureID {
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