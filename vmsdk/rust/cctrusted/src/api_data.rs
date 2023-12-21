use cctrusted_base::binary_blob::dump_data;
use cctrusted_base::cc_type::CcType;

// this struct is used in vTPM and other TEE scenarios
// e.g.: vTPM may need report based on selective PCRs
pub struct ExtraArgs {}

pub struct CcReport {
    cc_report: Vec<u8>,
    cc_type: CcType
}

impl CcReport {
    pub fn dump_cc_report(&self){
        dump_data(self.cc_report);
    }
}

// return structure for get_default_algorithm
pub struct Algorithm {
    pub algo_id: u8,
    pub algo_id_str: String,
}
