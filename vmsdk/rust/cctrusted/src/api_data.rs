// this struct is used in vTPM and other TEE scenarios
// e.g.: vTPM may need report based on selective PCRs
pub struct ExtraArgs {}

// return structure for get_default_algorithm
pub struct Algo {
    pub algo_id: u8,
    pub algo_id_str: String,
}