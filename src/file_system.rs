pub struct FileSystem {
    //TODO
}

impl FileSystem {
    pub fn check_num_for_name(&self, _name: &String) -> i32 {
        //TODO
        -1
    }

    pub fn file_length(&self, _lump: i32) -> i32 {
        //TODO
        -1
    }

    pub fn open_file_reader(&self) {
        //TODO   
    }
}

pub fn make_id(a: char, b: char, c: char, d: char) -> u32 {
    u32::from_le_bytes([a as u8,b as u8,c as u8,d as u8])
}