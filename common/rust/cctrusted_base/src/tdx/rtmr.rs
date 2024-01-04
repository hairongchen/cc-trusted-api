use crate::tcg::*;

pub struct TdxRTMR {
    index: u8,
    digest: (u8, TcgDigest),
}

impl TdxRTMR{
    fn new(index: u8, digest: [u8;48]) -> TdxRTMR {
        let tcg_digest = TcgDigest {
            algo_id: TPM_ALG_SHA384,
            hash: digest.to_vec()
        };

        TdxRTMR {
            index: index,
            digest: (algo_id, tcg_digest)
        }
    }
}

impl TcgIMR for TdxRTMR {

    fn max_index() -> u8 {
        return 3;
    }

    fn get_index(&self) -> u8 {
        return self.index;
    }

    fn get_hash(&self) -> Vec<&str> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
