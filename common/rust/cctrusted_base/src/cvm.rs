use crate::tcg::TcgDigest;

// holds the device node info
pub struct DeviceNode {
    pub device_path: String,
}

pub struct CcEventlogs {
    //TODO
}

// the interfaces a cvm should implement
pub trait CVM {

    /***
        retrive CVM signed report

        Args:
            nonce (String): against replay attacks
            data (String): user data

        Returns:
            the cc report byte array or error information
    */    
    fn process_cc_report(&mut self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error>;

    /***
        retrive CVM measurement registers, e.g.: RTMRs, vTPM PCRs, etc.

        Args:
            index (u8): the index of measurement register,
            algo_id (u8): the alrogithms ID

        Returns:
            TcgDigest struct
    */   
    fn process_cc_measurement(&self, index: u8, algo_id: u8) -> TcgDigest;
    
    //TODO!
    fn process_cc_eventlog(&self);

    // parse raw data to standard structure defined by TEE or TCG spec
    fn parse_cc_report(&self);
    fn parse_cc_measurement(&self);
    fn parse_cc_eventlog(&self);

    //Dump confidential VM information
    fn dump(&self);
}
