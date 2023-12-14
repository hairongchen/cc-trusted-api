mod rtmr

pub struct TdxRTMR {
    index: u8,
    digests: HashMap<u8, &TcgDigest>
}

impl TcgIMR for TdxRTMR {
    pub fn max_index(&self) -> int{
        return 3
    }

    pub fn get_index(&self) -> int {
        //TODO!
    }

    pub fn get_hash(&self) -> Vec<&str> {
        //TODO!
    }

    pub fn is_valid(&self) -> bool {
         //TODO!       
    }

}