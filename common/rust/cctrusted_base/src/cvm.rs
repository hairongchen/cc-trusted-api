// holds the device node info
pub struct DeviceNode {
    pub device_path: String,
}

pub struct CcEventlogs {
    //TODO
}

/***
    the interfaces a cvm should support:
        - dump: show basic CVM info like TEE type and version
        - process_cc_report: retrive CVM signed report
        - process_cc_measurement: retrive CVM measurement registers, e.g.: RTMRs, vTPM PCRs, etc.
        - process_cc_eventlog: retrive CVM eventlog, e.g.: CCEL, IMA log, ect.
        - parse_cc_report: parse CVM report
        - parse_cc_measurement: parse CVM measurement
        - parse_cc_eventlog: parse CVM eventlog
*/
pub trait CVM {
    fn process_cc_report(&mut self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error>;
    fn process_cc_measurement(&self);
    fn process_cc_eventlog(&self);

    // parse raw data to standard structure defined by TEE or TCG spec
    fn parse_cc_report(&self);
    fn parse_cc_measurement(&self);
    fn parse_cc_eventlog(&self);

    //Dump confidential VM information
    fn dump(&self);
}
