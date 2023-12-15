use std::path::Path;
use std::collections::HashMap;

// supported TEE types
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum TeeType {
    PLAIN = -1,
    TDX = 0,
    SEV = 1,
    CCA = 2,
    TPM = 3,
}

// TEE type to type name string mapping
lazy_static! {
    pub static  ref TEE_NAME_MAP: HashMap<TeeType, String> = {
        let mut map:HashMap<TeeType, String> = HashMap::new();
        map.insert(TeeType::PLAIN, "PLAIN".to_string());
        map.insert(TeeType::TDX, "TDX".to_string());
        map.insert(TeeType::SEV, "SEV".to_string());
        map.insert(TeeType::CCA, "CCA".to_string());
        map.insert(TeeType::TPM, "TPM".to_string());
        map
    };
}

// public known device node path
pub const TEE_TPM_PATH: &str = "/dev/tpm0";
pub const TEE_TDX_1_0_PATH: &str = "/dev/tdx-guest";
pub const TEE_TDX_1_5_PATH: &str = "/dev/tdx_guest";
pub const TEE_SEV_PATH: &str = "/dev/sev-guest";
pub const TEE_CCA_PATH: &str = "";

// holds the TEE type info
pub struct CcType {
    pub tee_type: TeeType,
    pub tee_type_str: String
}

// detect the TEE type
pub fn detect_cc_type() -> CcType {
    let mut tee_type = TeeType::PLAIN;
    if Path::new(TEE_TPM_PATH).exists() {
        tee_type = TeeType::TPM;
    } else if Path::new(TEE_TDX_1_0_PATH).exists()
        || Path::new(TEE_TDX_1_5_PATH).exists()
    {
        tee_type = TeeType::TDX;
    } else if Path::new(TEE_SEV_PATH).exists() {
        tee_type = TeeType::SEV;
    } else {
        // TODO add support for CCA and etc.
    }

    return CcType { tee_type: tee_type.clone(), tee_type_str: TEE_NAME_MAP.get(&tee_type).unwrap().to_owned() }
}