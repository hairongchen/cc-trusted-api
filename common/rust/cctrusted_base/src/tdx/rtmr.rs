use crate::tcg::*;
use anyhow::anyhow;

pub struct TdxRTMR {
    index: u8,
    digest: (u8, TcgDigest),
}

impl TdxRTMR{
    pub fn new(index: u8, algo_id: u8, digest: [u8;48]) -> Result<TdxRTMR, anyhow::Error> {

        match TdxRTMR::is_valid_index(index){
            Ok(_) => (),
            Err(e) => return Err(anyhow!("error creating TdxRTMR {:?}", e)),
        };

        match TdxRTMR::is_valid_algo(algo_id){
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

}

impl TcgIMR for TdxRTMR {

    fn max_index() -> u8 {
        return 3;
    }

    fn get_index(&self) -> u8 {
        return self.index;
    }

    fn get_tcg_digest(&self, _algo_id: u8) -> TcgDigest{
        self.digest.1.clone()
    }

    fn is_valid_index(index: u8) -> Result<bool, anyhow::Error> {
        if index > TdxRTMR::max_index() {
            return Err(anyhow!(
                "[valid_algo_id] invalid RTMR index: {}",
                index
            ));
        }

        Ok(true)
    }

    fn is_valid_algo(algo_id: u8) -> Result<bool, anyhow::Error> {

        match ALGO_NAME_MAP.get(&algo_id) {
            Some(_) => Ok(true),
            None => return Err(anyhow!("[valid_algo] invalid algo id: {}", algo_id)),
        }
    }
}
