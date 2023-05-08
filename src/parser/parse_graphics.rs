use crate::parser::*;

pub struct WADPaletteColor
{
    r: u8,
    g: u8,
    b: u8
}

pub struct WADSprite {
    width: u32,
    height: u32,
    left_offset: u32,
    top_offset: u32,
    posts: Vec<WADSpritePost>
}

pub struct WADSpritePost
{
    col: u8,
    row: u8,
    size: u8,
    pixels: Vec<u8>
}

pub fn read_pallete(wad_data: &Vec<u8>, offset: &mut usize, wad_parsed: &mut WADData) {
    println!("levels: {}", wad_parsed.levels.len());
    let i: usize = wad_parsed.levels.len();
    let index = wad_parsed.lump_map[i].get("PLAYPAL").unwrap().clone() as usize;
    let pallete_entry = wad_parsed.directory.get(index).unwrap();

    *offset = pallete_entry.offset as usize;
    while *offset < pallete_entry.offset  as usize + pallete_entry.size as usize {
        let mut palette_: Vec<WADPaletteColor> = Vec::with_capacity(256);

        for _i in 0 .. 256 {
            let color_: WADPaletteColor = WADPaletteColor { r: wad_data[*offset], g: wad_data[*offset + 1], b: wad_data[*offset + 2] };
            palette_.push(color_);
            *offset += 3;
        }

        wad_parsed.palletes.push(palette_);
    }
    println!("palletes: {}", wad_parsed.palletes.len());
}

pub fn read_colormap(wad_data: &Vec<u8>, offset: &mut usize, wad_parsed: &mut WADData) {
    let i: usize = wad_parsed.levels.len();
    let index = wad_parsed.lump_map[i].get("COLORMAP").unwrap().clone() as usize;
    let color_map = wad_parsed.directory.get(index).unwrap();

    *offset = color_map.offset as usize;
    while *offset < color_map.offset  as usize + color_map.size as usize {
        let mut colormap_: Vec<u8> = Vec::with_capacity(256);

        for _i in 0 .. 256 {
            colormap_.push(wad_data[*offset]);
            *offset += 1;
        }

        wad_parsed.color_maps.push(colormap_);
    }
    println!("color_maps: {}", wad_parsed.color_maps.len());
}

pub fn read_sprites(wad_data: &Vec<u8>, wad_parsed: &mut WADData, index: usize) {
    let sprite_lump = &wad_parsed.directory[index];
    let mut offset = sprite_lump.offset as usize;

    let width = u32::from(read_ushort(wad_data, &mut offset).unwrap());
    let height = u32::from(read_ushort(wad_data, &mut offset).unwrap());
    let left_offset = u32::from(read_ushort(wad_data, &mut offset).unwrap());
    let top_offset = u32::from(read_ushort(wad_data, &mut offset).unwrap());

    let mut sprite = WADSprite { width, height, left_offset, top_offset, posts: vec![] };

    let mut col_offsets = vec![];

    for _i in 0..sprite.width {
        col_offsets.push(read_uint(wad_data, &mut offset).unwrap());
    }

    for i in 0..sprite.width {
        offset = sprite_lump.offset as usize + col_offsets[i as usize] as usize;

        while wad_data[offset] != 0xff {
            let col = i as u8;
            let row = wad_data[offset];
            let size = wad_data[offset + 1];
            offset += 3; // 2 + 1 to skip the first unused byte
            let pixels = wad_data[offset..size as usize + offset].to_vec();
            let post: WADSpritePost = WADSpritePost { col, row, size, pixels};
            offset += post.size as usize;
            offset += 1; // to skip the last unused byte
            sprite.posts.push(post);
        }
    }
    // println!("Adding sprite :{}", sprite_lump.name);
    wad_parsed.sprites.push((sprite_lump.name.to_string(), sprite));
    
}

pub fn read_flats(wad_data: &Vec<u8>, wad_parsed: &mut WADData, index: usize) {
    let offset = wad_parsed.directory[index].offset as usize;
    let pixels = wad_data[offset..offset + 4096].to_vec();
    wad_parsed.flats = pixels;
}

pub fn is_doom_gfx(dv: &Vec<u8>,lump: WADEntry, offset: usize) ->bool {
    // first check the dimensions aren't ridiculous
    let mut temp_offset = offset;
    if read_ushort(dv, &mut temp_offset).unwrap() > 4096 {return false}
    if read_ushort(dv, &mut temp_offset).unwrap() > 4096 {return false}

    if read_short(dv, &mut temp_offset).unwrap().abs() > 2000 {return false}
    if read_short(dv, &mut temp_offset).unwrap().abs() > 2000 {return false}

    // then check it ends in 0xFF
    temp_offset = offset + lump.size as usize - 1;
    let res = dv[temp_offset];
    if res != 255 {
        // sometimes the graphics have up to 3 garbage 0x00 bytes at the end
        let mut found = false;
        for b in 1..=4 {
            if found == false {
                temp_offset = offset + lump.size as usize - b;
                let temp = dv[temp_offset];
                if temp == 255 {
                    found = true;
                } else if temp != 0 {
                    return false;
                }
            }
        }
        return found
    }
    // I think this is enough for now. If I get false positives, I'll look into more comprehensive checks.
    true
}