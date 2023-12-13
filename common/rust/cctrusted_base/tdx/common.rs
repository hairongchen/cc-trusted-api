pub enum TdxVersion {
    TDX_1_0,
    TDX_1_5,
}

pub enum TdxOperation {
    TDX_GET_TD_REPORT = 1,
    TDX_1_0_GET_QUOTE = 2,
    TDX_1_5_GET_QUOTE = 4,
}

const REPORT_DATA_LEN: u32 = 64;
const TDX_REPORT_LEN: u32 = 1024;
const TDX_QUOTE_LEN: usize = 4 * 4096;

