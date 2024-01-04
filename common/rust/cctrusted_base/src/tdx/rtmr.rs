use crate::tcg::*;
use anyhow::anyhow;

pub struct TdxRTMR {
    index: u8,
    digest: (u8, TcgDigest),
}

impl TdxRTMR{
    pub fn new(index: u8, algo_id: u8, digest: [u8;48]) -> Result<TdxRTMR, anyhow::Error> {

        match TdxRTMR::valid_index(index){
            Ok(_) => (),
            Err(e) => return Err(anyhow!("error creating TdxRTMR {:?}", e)),
        };

        match TdxRTMR::valid_algo(algo_id){
            Ok(_) => (),
            Err(e) => return Err(anyhow!("error creating TdxRTMR {:?}", e)),
        };

        let tcg_digest = TcgDigest {
            algo_id: algo_id,
            hash: digest.to_vec()
        };

        Ok(TdxRTMR {
            index: index,
            digest: (algo_id, tcg_digest)
        })
    }

    pub fn valid_index(index: u8) -> Result<bool, anyhow::Error> {
        if index < 0 || index > TdxRTMR::max_index() {
            return Err(anyhow!(
                "[valid_algo_id] invalid RTMR index: {}",
                index
            ));
        }

        Ok(true)
    }

    pub fn valid_algo(algo_id: u8) -> Result<bool, anyhow::Error> {

        match ALGO_NAME_MAP.get(&algo_id) {
            Some(_) => Ok(true),
            None => return Err(anyhow!("[valid_algo] invalid algo id: {}", algo_id)),
        };
    }
}

impl TcgIMR for TdxRTMR {

    fn max_index() -> u8 {
        return 3;
    }

    fn get_index(&self) -> u8 {
        return self.index;
    }

    fn get_tcg_digest(&self, _algo_id: u8) -> TcgDigest{
        self.digest.1
    }

    fn get_hash(&self) -> Vec<&str> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
