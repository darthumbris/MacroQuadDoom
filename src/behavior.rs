
use std::ops::Index;
pub mod parse_behavior;
pub use crate::parser::*;

#[derive(PartialEq)]
enum Acs{
    AcsOld,
    AcsEnhanced,
    AcsLittleEnhanced
}

pub struct WADLevelBehavior {
    marker: i32,
    format: Acs,
    data: Vec<u8>,
    data_size: i32,
    chunks: Vec<u8>,
    should_localize: bool,
}

//Hexen version
pub struct WADLevelScriptInfo {
    number: u32,
    address: u32, //offset
    arg_count: u32
    //type is numer / 1000
}

//old zdoom version
pub struct WADLevelScriptInfoZdoomOld {
    number: i32,
    type_: u16,
    address: u32, 
    arg_count: u32
}

//new zdoom version
pub struct WADLevelScriptInfoZdoomNew {
    number: i16,
    type_: u8,
    arg_count: u8,
    address: u32 
}

//in Memory version
pub struct WADLevelScriptInfoMemory {
    number: i32,
    address: u32,
    type_: u8,
    arg_count: u8,
    var_count: u16,
    flags: u16,
    local_arrays: AcsLocalArrays,
    profile_data: AcsProfileInfo
}

struct AcsProfileInfo {
    total_instr: u128,
    num_runs: u32,
    min_instr_per_run: u32,
    max_instr_per_run: u32
}

impl AcsProfileInfo {
    pub fn add_run(&self, num_instr: u32) {
        self.total_instr += u128::from(num_instr);
        self.num_runs += 1;
        if self.num_runs < self.min_instr_per_run {
            self.min_instr_per_run = num_instr;
        }
        if self.num_runs > self.max_instr_per_run {
            self.max_instr_per_run = num_instr;
        }
    }

    pub fn reset(&self) {
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

struct AcsLocalArrays {
    count : u32,
    info: Option<Vec<AcsLocalArrayInfo>>

    //constructor

    //destructor

    // set fn

    // get fn
}

impl AcsLocalArrays {
    pub fn new() -> Self {
        Self {
            count: 0,
            info: None
        }
    }

    //destructor?

    pub fn set(&self, locals: &AcsLocalVariables, array_num: i32, array_entry: i32, value: i32) {
        if (array_num as u32) < self.count && (array_entry as u32) < self.info.unwrap()[array_num as usize].size {
            locals[self.info.unwrap()[array_num as usize].offset as usize + array_entry as usize] = value;
        }
    }

    pub fn get(&self, locals: &AcsLocalVariables, array_num: i32, array_entry: i32) -> i32 {
        if (array_num as u32) < self.count && (array_entry as u32) < self.info.unwrap()[array_num as usize].size {
            return locals[self.info.unwrap()[array_num as usize].offset as usize + array_entry as usize];
        }
        0
    }
}

struct AcsLocalArrayInfo {
    size: u32,
    offset: i32
}

struct AcsLocalVariables {
    memory: Vec<i32>,
    count: usize
}

impl Index<usize> for AcsLocalVariables {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        //check here for overflow?
        &self.memory[index]
    }
}

impl AcsLocalVariables {

    //constructor

    fn reset(&self, memory: Vec<i32>, count: usize) {
        self.memory = memory;
        self.count = count;
    }

    fn getPointer(&self) -> &Vec<i32> {
        &self.memory
    }
}