
use std::ops::IndexMut;
use std::ops::Index;
use std::cmp::Ordering;
use std::rc::Rc;
// pub mod parse_behavior;
pub use crate::parser::*;
use crate::level::LevelLocals;

#[derive(PartialEq)]
pub enum Acs{
    AcsOld,
    AcsEnhanced,
    AcsLittleEnhanced,
    AcsUnkown
}

pub struct WADLevelBehavior {
    pub marker: i32,
    pub format: Acs,
    pub data: Vec<u8>,
    pub data_size: i32,
    pub chunks: Vec<u8>,
    pub should_localize: bool,
}

impl WADLevelBehavior {
    pub fn new() -> WADLevelBehavior {
        WADLevelBehavior { marker: 0, format: Acs::AcsUnkown, data: vec![], data_size: 0, chunks: vec![], should_localize: false }
    }
}

enum _WADLevelScriptInfo {
    Hexen(WADLevelScriptInfoHexen),
    ZDoomOld(WADLevelScriptInfoZdoomOld),
    ZDoomNew(WADLevelScriptInfoZdoomNew),
    Memory(WADLevelScriptInfoMemory)
}

// pub struct WADLevelScriptInfo {
    
// }

//Hexen version
#[derive(Clone, Copy)]
pub struct WADLevelScriptInfoHexen {
    pub number: u32,
    pub address: u32, //offset
    pub arg_count: u32
    //type is numer / 1000
}

//old zdoom version
#[derive(Clone, Copy)]
pub struct WADLevelScriptInfoZdoomOld {
    pub number: i32,
    pub type_: u16,
    pub address: u32, 
    pub arg_count: u32
}

//new zdoom version
#[derive(Clone, Copy)]
pub struct WADLevelScriptInfoZdoomNew {
    pub number: i16,
    pub type_: u8,
    pub arg_count: u8,
    pub address: u32 
}

//in Memory version
#[derive(Clone, Eq)]
pub struct WADLevelScriptInfoMemory {
    pub number: i32,
    pub address: u32,
    pub type_: u8,
    pub arg_count: u8,
    pub var_count: u16,
    pub flags: u16,
    pub local_arrays: Option<AcsLocalArrays>,
    pub profile_data: Option<AcsProfileInfo>
}

impl PartialOrd for WADLevelScriptInfoMemory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WADLevelScriptInfoMemory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialEq for WADLevelScriptInfoMemory {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AcsProfileInfo {
    total_instr: u64,
    num_runs: u32,
    min_instr_per_run: u32,
    max_instr_per_run: u32
}


impl AcsProfileInfo {
    pub fn add_run(&mut self, num_instr: u32) {
        self.total_instr += u64::from(num_instr);
        self.num_runs += 1;
        if self.num_runs < self.min_instr_per_run {
            self.min_instr_per_run = num_instr;
        }
        if self.num_runs > self.max_instr_per_run {
            self.max_instr_per_run = num_instr;
        }
    }

    pub fn reset(&mut self) {
        self.total_instr = 0;
        self.num_runs = 0;
        self.min_instr_per_run = 0;
        self.max_instr_per_run = 0;
    }

    pub fn new() -> Self {
        Self {
            total_instr: 0,
            num_runs: 0,
            min_instr_per_run: 0,
            max_instr_per_run: 0
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct AcsLocalArrays {
    pub count : u32,
    pub info: Vec<AcsLocalArrayInfo>

    //destructor
}

impl AcsLocalArrays {
    pub fn new() -> Self {
        Self {
            count: 0,
            info: vec![]
        }
    }

    //destructor?

    pub fn set(&self, locals: &mut AcsLocalVariables, array_num: i32, array_entry: i32, value: i32) {
        if (array_num as u32) < self.count && (array_entry as u32) < self.info[array_num as usize].size {
            locals[self.info[array_num as usize].offset as usize + array_entry as usize] = value;
        }
    }

    pub fn get(&self, locals: &AcsLocalVariables, array_num: i32, array_entry: i32) -> i32 {
        if (array_num as u32) < self.count && (array_entry as u32) < self.info[array_num as usize].size {
            return locals[self.info[array_num as usize].offset as usize + array_entry as usize];
        }
        0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AcsLocalArrayInfo {
    pub size: u32,
    pub offset: i32
}

pub struct AcsLocalVariables {
    pub memory: Vec<i32>,
    pub count: usize
}

impl Index<usize> for AcsLocalVariables {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        //check here for overflow?
        &self.memory[index]
    }
}

impl IndexMut<usize> for AcsLocalVariables {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        //check here for overflow?
        &mut self.memory[index]
    }
}

impl AcsLocalVariables {

    //constructor

    fn _reset(&mut self, memory: Vec<i32>, count: usize) {
        self.memory = memory;
        self.count = count;
    }

    fn _get_pointer(&self) -> &Vec<i32> {
        &self.memory
    }
}

struct _ZDoomBehaviour {
    // pub map_vars,
    level: Option<Box<LevelLocals>>,
    data: Option<Vec<u8>>,
    chunks: Option<Vec<u8>>,
    scripts: Option<Vec<WADLevelScriptInfoMemory>>,
    functions: Option<Vec<_ScriptFunction>>,
    function_profile_data: Option<AcsProfileInfo>,
    array_store: Option<_ArrayInfo>,
    arrays: Option<Vec<_ArrayInfo>>,
    format: Acs,
    lump_num: i32,
    data_size: i32,
    script_count: i32,
    function_count: i32,
    array_count: i32,
    total_array_count: i32,
    string_table: u32,
    library_id: u32,
    should_localize: bool,
    map_var_store: [i32;128],
    imports: Option<Vec<Rc<_ZDoomBehaviour>>>,
    module_name: [u8;9],
    jump_points: Option<Vec<i32>>,

}

impl _ZDoomBehaviour {
    fn _new() -> Self {
        Self { 
            script_count: 0, 
            function_count: 0, 
            array_count: 0, 
            total_array_count: 0, 
            scripts: None, 
            functions: None, 
            arrays: None, 
            array_store: None, 
            chunks: None, 
            data: None, 
            format: Acs::AcsUnkown, 
            lump_num: -1, 
            map_var_store: [0;128], 
            module_name: [0; 9], 
            function_profile_data: None,

            // map_vars: (), 
            level: None,
            data_size: 0, 
            string_table: 0, 
            library_id: 0, 
            should_localize: false, 
            imports: None, 
            jump_points: None 
        }
    }
}

struct _ArrayInfo {
    size: u32,
    elements: Vec<i32>
}

struct _ScriptFunction {
    arg_count: u8,
    has_return_value: u8,
    import_num: u8,
    local_count: i32,
    address: u32,
    local_arrays: AcsLocalArrays
}

struct _BehaviorContainer {}
