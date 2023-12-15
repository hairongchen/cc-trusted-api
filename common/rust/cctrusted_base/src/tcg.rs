use std::collections::HashMap;

const TPM_ALG_ERROR: u8 = 0x0
const TPM_ALG_RSA: u8 = 0x1
const TPM_ALG_TDES: u8 = 0x3
const TPM_ALG_SHA256: u8 = 0xB
const TPM_ALG_SHA384: u8 = 0xC
const TPM_ALG_SHA512: u8 = 0xD

pub const AlgoNameMap: HashMap<&u8, &str> = [
    (TPM_ALG_ERROR, "TPM_ALG_RSA"),
    (TPM_ALG_TDES, "TPM_ALG_TDES"),
    (TPM_ALG_SHA256, "TPM_ALG_SHA256"),
    (TPM_ALG_SHA384, "TPM_ALG_SHA384"),
    (TPM_ALG_SHA512, "TPM_ALG_SHA512")
].iter().cloned().collect();

// this trait retrieve tcg standard algorithm name in string
pub trait TcgAlgorithmRegistry {
    pub fn get_algorithm_string(alg_id: u8) -> Result<String>{
        //TODO!
    }
}

// digest format: (algo id, hash value)
pub struct TcgDigest {
    algo_id: u8,
    hash: Vec<u8>
}

// this trait retrieve IMR's max index of a TEE and hash value 
pub trait TcgIMR {
    pub fn max_index(&self);
    pub fn get_index(&self);
    pub fn get_hash(&self);
    pub fn is_valid(&self);
}