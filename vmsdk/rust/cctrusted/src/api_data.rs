use cctrusted_base::cc_type::CcType;

pub enum LocalTeeType {
    PLAIN = -1,
    TDX = 0,
    SEV = 1,
    CCA = 2,
    TPM = 3,
}

// this struct is used in vTPM and other TEE scenarios
// e.g.: vTPM may need report based on selective PCRs
pub struct ExtraArgs {}

pub struct CcReport {
    pub cc_report: Vec<u8>,
    pub cc_type: CcType
}

pub struct TdxQuote{
    pub name: String,
    pub var1: u8
}

#[allow(dead_code)]
pub struct TpmQuote{
    pub name: String,
    pub var1: u8,
    pub var2: u8
}

pub struct ParsedCcReport {}

pub trait ParseCcReport<T> {
    fn parse_cc_report(_report: Vec<u8>) -> T;
}

// return structure for get_default_algorithm
pub struct Algorithm {
    pub algo_id: u8,
    pub algo_id_str: String,
}
