pub struct DeviceNode {
    pub device_path: String
}

pub struct CcEventlogs {
    //TODO!
}

// the interfaces a cvm should support
pub trait CVM {

    fn process_cc_report(&self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error>;
    fn process_cc_measurement();
    fn process_cc_eventlog();

    //Dump confidential VM information
    fn dump(&self);
}