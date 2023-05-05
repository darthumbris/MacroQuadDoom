use std::cmp::Ordering;

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
        // info.push(WADLevelScriptInfo { number, offset: offset_, arg_count });
    }

    let string_count = read_int(lump, &mut offset).unwrap();

    let mut string_table: Vec<i32> = vec![];

    for _i in 0..string_count {
        string_table.push(read_int(lump, &mut offset).unwrap());
    }

    

   
    let behavior: WADLevelBehavior = WADLevelBehavior{ marker, format, data: lump.to_owned(), data_size, chunks, should_localize};
    Some(behavior)
}

fn find_chunk(data: &Vec<u8>, id: u32) ->Option<Vec<u8>> {

    let mut offset: usize = 0;
    while offset < data.len() {
        if read_uint(data, &mut offset).unwrap() == id {
            return Some(data[offset..data.len()].to_vec());
        }
        let temp_offset = read_uint(data, &mut offset).unwrap() + 8;
        offset = temp_offset as usize;
    }
    None
}

fn find_script(number: i16, scripts: Vec<WADLevelScriptInfoMemory>) -> Result<usize, usize> {
    let script = WADLevelScriptInfoMemory {
        number: number as i32,
        address: 0,
        type_: 0,
        arg_count: 0,
        var_count: 0,
        flags: 0,
        local_arrays: None,
        profile_data: None
    };
    return scripts.binary_search(&script)
}

fn sort_scripts(a: &WADLevelScriptInfoMemory, b: &WADLevelScriptInfoMemory) -> Ordering {
    return a.number.cmp(&b.number)
}

//  fn(&'a behavior::WADLevelScriptInfoMemory, &'b behavior::WADLevelScriptInfoMemory)

fn load_directory(data: &Vec<u8>, format: Acs) {

    let mut offset = 0;
    let mut script_count = 0;
    let mut scripts: Vec<WADLevelScriptInfoMemory> = vec![];
    match format {
        Acs::AcsOld => {
            let inf_offset = read_int(data, &mut offset).unwrap();
            offset = inf_offset as usize;
            script_count = read_int(data, &mut offset).unwrap();
            if script_count != 0 {
                scripts.reserve(script_count as usize);

                for _i in 0..script_count {
                    let ptr2: WADLevelScriptInfoMemory;

                    let number = read_int(data, &mut offset).unwrap();
                    let type_ = (number / 1000) as u8;
                    let address = read_uint(data, &mut offset).unwrap();
                    let arg_count = read_int(data, &mut offset).unwrap() as u8;

                    ptr2 = WADLevelScriptInfoMemory {
                        number,
                        address,
                        type_,
                        arg_count,
                        var_count: 0,
                        flags: 0,
                        profile_data: None,
                        local_arrays: None
                    };
                    scripts.push(ptr2);
                }
            }
        },
        Acs::AcsEnhanced | Acs::AcsLittleEnhanced => {
            let chunk = find_chunk(data, 0x53505452);
            if chunk.is_none() {
                // no scripts
            }
            else if read_uint(data, &mut offset).unwrap() != 0x41435300 { //ACS\0
                script_count = read_int(data, &mut offset).unwrap() / 12;

                scripts.reserve(script_count as usize);
                offset += 4; //? maybe 8?
                for _i in 0..script_count {
                    let ptr2: WADLevelScriptInfoMemory;

                    let number = read_int(data, &mut offset).unwrap();
                    let type_ = read_ushort(data, &mut offset).unwrap() as u8;
                    let address = read_uint(data, &mut offset).unwrap();
                    let arg_count = read_uint(data, &mut offset).unwrap() as u8;

                    ptr2 = WADLevelScriptInfoMemory {
                        number,
                        address,
                        type_,
                        arg_count,
                        var_count: 0,
                        flags: 0,
                        profile_data: None,
                        local_arrays: None
                    };
                    scripts.push(ptr2);
                }
            }
            else {
                script_count = read_int(data, &mut offset).unwrap() / 8;

                scripts.reserve(script_count as usize);
                offset += 4; //? maybe 8?
                for _i in 0..script_count {
                    let ptr2: WADLevelScriptInfoMemory;

                    let number = i32::from(read_short(data, &mut offset).unwrap());
                    let type_ = read_u8(data, &mut offset).unwrap();
                    let arg_count = read_u8(data, &mut offset).unwrap();
                    let address = read_uint(data, &mut offset).unwrap();

                    ptr2 = WADLevelScriptInfoMemory {
                        number,
                        address,
                        type_,
                        arg_count,
                        var_count: 20,
                        flags: 0,
                        profile_data: None,
                        local_arrays: None
                    };
                    scripts.push(ptr2);
                }
            }
        }
        _ => {}
    }
    if script_count > 1 {
        scripts.sort_by(sort_scripts);

        if format == Acs::AcsOld {
            for i in 0..(script_count - 1) as usize {
                if scripts[i].number == scripts[i + 1].number {
                    println!("{} has a duplicate", scripts[i].number);
                    if scripts[i].type_ == 0 { //Script closed
                        scripts.swap(i, i + 1);
                    }
                }
            }
        }
    }

    if format == Acs::AcsOld { return }

    let mut chunk = find_chunk(data, 0x53464C47); //SFLG
    if chunk.is_some() {
        offset = 0;
        let max = read_int(&chunk.unwrap(), &mut offset).unwrap() / 4;
        for _i in (0..max).rev() {
            let number = read_short(&chunk.unwrap(), &mut offset).unwrap();
            let index = find_script(number, scripts);
            if index.is_ok() {
                scripts[index.unwrap()].flags = read_ushort(&chunk.unwrap(), &mut offset).unwrap();
            }
            else {offset += 2}
        }
    }

    chunk = find_chunk(data, 0x53564354); //SVCT
    if chunk.is_some() {
        offset = 0;
        let max = read_int(&chunk.unwrap(), &mut offset).unwrap() / 4;
        for i in (0..max).rev() {
            let number = read_short(&chunk.unwrap(), &mut offset).unwrap();
            let index = find_script(number, scripts);
            if index.is_ok() {
                scripts[index.unwrap()].var_count = read_ushort(&chunk.unwrap(), &mut offset).unwrap();
            }
            else {offset += 2}
        }
    }
    chunk = find_chunk(data, 0x53415259);
    // while chunk.is_some() { //SVCT
    //     offset = 4;
    //     let size = read_int(chunk, &mut offset).unwrap();
    //     if size >= 6 {
    //         let script_num = read_int(chunk, &mut offset).unwrap();
    //     }

    //     // chunk = next_chunk(); //TODO finish the parsing of the behavior
    // }

}