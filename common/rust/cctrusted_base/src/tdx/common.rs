pub enum TdxVersion {
    TDX_1_0,
    TDX_1_5,
}

pub const TdxVersionMap: HashMap<&u8, &str> = [
    (TDX_1_0, "1.0"),
    (TDX_1_5, "1.5"),
].iter().cloned().collect();

pub const TdxDeviceNodeMap: HashMap<&TdxVersion, &str> = [
    (TDX_1_0, TEE_TDX_1_0_PATH),
    (TDX_1_5, TEE_TDX_1_5_PATH),
].iter().cloned().collect();

pub enum TdxOperation {
    TDX_GET_TD_REPORT = 1,
    TDX_1_0_GET_QUOTE = 2,
    TDX_1_5_GET_QUOTE = 4,
}

const REPORT_DATA_LEN: u32 = 64;
const TDX_REPORT_LEN: u32 = 1024;
const TDX_QUOTE_LEN: usize = 4 * 4096;

fn get_tdx_version() -> TdxVersion {
    if Path::new(TEE_TDX_1_0_PATH).exists() {
        TdxVersion::TDX_1_0
    } else if Path::new(TEE_TDX_1_5_PATH).exists() {
        TdxVersion::TDX_1_5
    } else {
        panic!("get_tdx_version: no TDX device found!");
    }
}
