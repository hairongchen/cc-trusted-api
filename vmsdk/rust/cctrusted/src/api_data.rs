pub enum CcType {
    PLAIN = -1,
    TDX = 0,
    SEV = 1,
    CCA = 2,
    TPM = 3,
}

// input of API get_cc_report()
// this struct is used in vTPM and other TEE scenarios
// e.g.: vTPM may need report based on selective PCRs
pub struct ExtraArgs {}

// return of API get_cc_report()
pub struct CcReport {
    pub cc_report: Vec<u8>,
    pub cc_type: CcType
}

// return of API parse_cc_report() in TDX case
pub struct CcTdxReport {
    pub name: String,
    pub var: u8
}

// return of API parse_cc_report() in TPM case
pub struct CcTpmReport {}

/***
    trait to be implemented for cc report parsing.

    the cooresponding implementation of parse_cc_report will be called according to 
    intented return format and the return of the trait function depends on 
    the type of cc report, e.g.: TdxQuote, TpmQuote and etc.

    TDX quote parsing Example:
    if following is provided:
    let tdx_quote: TdxQuote = parse_cc_report(cc_report_str);
    then this implementation in api.rs will be called:
    fn parse_cc_report(report: Vec<u8>) -> Result<TdxQuote, anyhow::Error>;
*/
pub trait ParseCcReport<T> {
    fn parse_cc_report(_report: Vec<u8>) -> Result<T, anyhow::Error>;
}

// return structure for get_default_algorithm
pub struct Algorithm {
    pub algo_id: u8,
    pub algo_id_str: String,
}
