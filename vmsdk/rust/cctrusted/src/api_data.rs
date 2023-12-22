use cctrusted_base::cc_type::CcType;
use cctrusted_base::tdx::quote::TdxQuote;

// input of API get_cc_report()
// this struct is used in vTPM and other TEE scenarios
// e.g.: vTPM may need report based on selective PCRs
pub struct ExtraArgs {}

// return of API get_cc_report()
pub struct CcReport {
    pub cc_report: Vec<u8>,
    pub cc_type: CcType
}

// return of API parse_cc_report()
pub struct TpmQuote{
    // TODO
}

// trait to be implemented for cc report parsing 
// return of the trait function depends on the type of cc report, e.g.:
// TdxQuote, TpmQuote and etc.
pub trait ParseCcReport<T> {
    fn parse_cc_report(_report: Vec<u8>) -> Result<T, anyhow::Error>;
}

// return structure for get_default_algorithm
pub struct Algorithm {
    pub algo_id: u8,
    pub algo_id_str: String,
}
