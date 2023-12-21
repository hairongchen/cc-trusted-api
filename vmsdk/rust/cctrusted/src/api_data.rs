use cctrusted_base::cc_type::CcType;

// this struct is used in vTPM and other TEE scenarios
// e.g.: vTPM may need report based on selective PCRs
pub struct ExtraArgs {}

pub struct CcReport {
    pub cc_report: Vec<u8>,
    pub cc_type: CcType
}

pub struct TdxQuote{
    name: String,
    var1: u8
}

pub struct TpmQuote{
    name: String,
    var1: u8,
    var2: u8
}

pub struct ParsedCcReport {}

pub trait ParseCcReport<T> {
    fn parse_cc_report(_report: Vec<u8>) -> T;
}

impl ParseCcReport<TdxQuote> for ParsedCcReport{
    fn parse_cc_report(_report: Vec<u8>) -> TdxQuote{
        TdxQuote{
            name: "TDX".to_string(),
            var1: 1
        }
    }
}

impl ParseCcReport<TpmQuote> for ParsedCcReport{
    fn parse_cc_report(_report: Vec<u8>) -> TpmQuote{
        TpmQuote{
            name: "TPM".to_string(),
            var1: 0,
            var2: 2
        }
    }
} 

// return structure for get_default_algorithm
pub struct Algorithm {
    pub algo_id: u8,
    pub algo_id_str: String,
}
