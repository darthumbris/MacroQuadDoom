
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

pub struct WADLevelScriptInfo {
    number: i32,
    offset: i32, //Address?
    arg_count: i32
}