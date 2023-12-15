pub struct TdxRTMR {
    index: u8,
    digests: HashMap<u8, &TcgDigest>
}

impl TcgIMR for TdxRTMR {
    pub fn max_index(&self) -> int{
        return 3
    }

    pub fn get_index(&self) -> int {
        todo!()
    }

    pub fn get_hash(&self) -> Vec<&str> {
        todo!()
    }

    pub fn is_valid(&self) -> bool {
        todo!()
    }

}