pub use crate::parser::*;

use crate::behavior::*;

pub fn parse_behavior(lump: &Vec<u8>) -> Option<WADLevelBehavior> {
    let mut offset: usize = 0;
    let mut data_size = lump.len() as i32;
    let marker = read_int(lump, &mut offset).unwrap();
    let info_offset = read_int(lump, &mut offset).unwrap(); //dirof?

    if marker != 0x41435300 && marker != 0x41435345 && marker != 0x41435365 || data_size < 32 {
        return None;
    }

    let mut format: Acs;
    let mut chunks = lump.clone();
    let mut should_localize = false;


    match marker {
        0x41435300 => format = Acs::AcsOld, //ACS\0
        0x41435345 => format = Acs::AcsEnhanced, //ACSE
        0x41435365 => format = Acs::AcsLittleEnhanced, //ACSe
        _ => return None
    }

    offset = info_offset as usize - 1;
    let pretag = read_int(lump, &mut offset).unwrap();
    offset = info_offset as usize;
    let script_count = read_int(lump, &mut offset).unwrap(); //DataSize if ACSOLD?

    if format == Acs::AcsOld {
        if info_offset >= 6 * 4 && (pretag == 0x41435345 || pretag == 0x41435365) {

            if pretag == 0x41435365 {format = Acs::AcsLittleEnhanced;}
            else {format = Acs::AcsEnhanced;}

            data_size = info_offset - 8;
            let mut temp_offset = info_offset as usize - 2;
            let start = read_int(lump, &mut temp_offset).unwrap() as usize;
            chunks = lump[start..lump.len()].to_vec();
        }
        should_localize = false;
    }
    else {
        chunks = lump[info_offset as usize..lump.len()].to_vec();
    }

    //Load Scriptdirectory

    let mut info: Vec<WADLevelScriptInfo> = vec![];

    for _i in 0..script_count {
        let number = read_int(lump, &mut offset).unwrap();
        let offset_ = read_int(lump, &mut offset).unwrap();
        let arg_count = read_int(lump, &mut offset).unwrap();
        info.push(WADLevelScriptInfo { number, offset: offset_, arg_count });
    }

    let string_count = read_int(lump, &mut offset).unwrap();

    let mut string_table: Vec<i32> = vec![];

    for _i in 0..string_count {
        string_table.push(read_int(lump, &mut offset).unwrap());
    }

    

   
    let behavior: WADLevelBehavior = WADLevelBehavior{ marker, format, data: lump.to_owned(), data_size, chunks, should_localize};
    Some(behavior)
}

fn load_directory(data: &Vec<u8>, format: Acs) {

    let mut offset = 0;
    let mut script_count = 0;
    match format {
        Acs::AcsOld => {
            let inf_offset = read_int(data, &mut offset).unwrap();
            offset = inf_offset as usize;
            script_count = read_int(data, &mut offset).unwrap();
            if script_count != 0 {

            }
        },
        Acs::AcsEnhanced | Acs::AcsLittleEnhanced => {

        }
        _ => {}
    }
}