use std::path::Path;
use std::collections::HashMap;
use TeeType::*;
use crate::tdx::common::TdxVersion::*;

#[derive(Eq, Hash, PartialEq)]
pub enum TeeType {
    PLAIN = -1,
    TDX = 0,
    SEV = 1,
    CCA = 2,
    TPM = 3,
}

pub const TeeNameMap: HashMap<TeeType, &str> = [
    (PLAIN, "PLAIN"),
    (TDX, "TDX"),
    (SEV, "SEV"),
    (CCA, "CCA"),
    (TPM, "TPM"),
].iter().cloned().collect();

// public known device node path
pub const TEE_TPM_PATH: &str = "/dev/tpm0";
pub const TEE_TDX_1_0_PATH: &str = "/dev/tdx-guest";
pub const TEE_TDX_1_5_PATH: &str = "/dev/tdx_guest";
pub const TEE_SEV_PATH: &str = "/dev/sev-guest";
pub const TEE_CCA_PATH: &str = "";

// the TEE type
pub struct CcType {
    tee_type: TeeType,
    tee_type_str: String
}

// detect the TEE running in
pub fn detect_cc_type() -> CcType {
    let tee_type = TeeType::PLAIN;
    if Path::new(TEE_TPM_PATH).exists() {
        tee_type = TeeType::TPM;
    } else if Path::new(TEE_TDX_1_0_PATH).exists()
        || Path::new(TEE_TDX_1_5_PATH).exists()
    {
        tee_type = TeeType::TDX;
    } else if Path::new(TEE_SEV_PATH).exists() {
        tee_type = TeeType::SEV;
    } else {
        // TODO! add support for CCA
    }

    return CcType { tee_type, tee_type_str: TeeNameMap.get(&tee_type) }
}