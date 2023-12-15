use std::collections::HashMap;

pub const TPM_ALG_ERROR: u8 = 0x0;
pub const TPM_ALG_RSA: u8 = 0x1;
pub const TPM_ALG_TDES: u8 = 0x3;
pub const TPM_ALG_SHA256: u8 = 0xB;
pub const TPM_ALG_SHA384: u8 = 0xC;
pub const TPM_ALG_SHA512: u8 = 0xD;

pub const AlgoNameMap: HashMap<&u8, &str> = [
    (TPM_ALG_ERROR, "TPM_ALG_RSA"),
    (TPM_ALG_TDES, "TPM_ALG_TDES"),
    (TPM_ALG_SHA256, "TPM_ALG_SHA256"),
    (TPM_ALG_SHA384, "TPM_ALG_SHA384"),
    (TPM_ALG_SHA512, "TPM_ALG_SHA512")
].iter().cloned().collect();

// this trait retrieve tcg standard algorithm name in string
pub trait TcgAlgorithmRegistry {
    fn get_algorithm_string(alg_id: u8) -> String{
        todo!()
    }
}

// digest format: (algo id, hash value)
pub struct TcgDigest {
    algo_id: u8,
    hash: Vec<u8>
}

// this trait retrieve IMR's max index of a TEE and hash value 
pub trait TcgIMR {
    fn max_index(&self);
    fn get_index(&self);
    fn get_hash(&self);
    fn is_valid(&self);
}