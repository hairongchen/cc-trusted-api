#![warn(dead_code)]

use std::collections::HashMap;
use crate::tcg::*;

#![warn(non_upper_case_globals)]
pub struct TdxRTMR {
    index: u8,
    digests: HashMap<u8, TcgDigest>
}

impl TcgIMR for TdxRTMR {
    fn max_index(&self) -> u8{
        return 3
    }

    fn get_index(&self) -> u8 {
        todo!()
    }

    fn get_hash(&self) -> Vec<&str> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }

}