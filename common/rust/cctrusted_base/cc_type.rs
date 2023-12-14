mod cctype

#[derive(Debug, Clone)]
pub enum TeeType {
    NONE = -1,
    TDX = 0,
    SEV = 1,
    CCA = 2,
    TPM = 3,
}

pub const TeeNameMap: HashMap<&u8, &str> = [
    (NONE, "NONE"),
    (TDX, "TDX"),
    (SEV, "SEV"),
    (CCA, "CCA"),
    (TPM, "TPM"),
].iter().cloned().collect();

// public known device node path
const TEE_TPM_PATH: &str = "/dev/tpm0"
const TEE_TDX_1_0_PATH: &str = "/dev/tdx-guest"
const TEE_TDX_1_5_PATH: &str = "/dev/tdx_guest"
const TEE_SEV_PATH: &str = "/dev/sev-guest"
const TEE_CCA_PATH: &str = ""

// the TEE type
struct CcType {
    tee_type: TeeType
    tee_type_str: String
}

// detect the TEE running in
pub fn detect_cc_type() -> CcType {
    let tee_type = TeeType::NONE
    if Path::new(TEE_TPM_PATH).exists() {
        tee_type = TeeType::TPM
    } else if Path::new(TEE_TDX_1_0_PATH).exists()
        || Path::new(TEE_TDX_1_5_PATH).exists()
    {
        tee_type = TeeType::TDX
    } else if Path::new(TEE_SEV_PATH).exists() {
        tee_type = TeeType::SEV
    } // TODO! add support for CCA

    return CcType { tee_type, tee_type_str: TeeNameMap.get(tee_type) }
}