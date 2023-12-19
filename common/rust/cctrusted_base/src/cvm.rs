// holds the device node info
pub struct DeviceNode {
    pub device_path: String
}

pub struct CcEventlogs {
    //TODO
}

/*** 
    the interfaces a cvm should support:
        - dump: show basic CVM info like TEE type and version
        - process_cc_report: retrive and parse CVM report
        - process_cc_measurement: retrive and parse CVM measurement registers, e.g.: RTMRs, vTPM PCRs, etc.
        - process_cc_eventlog: retrive and parse CVM eventlog, e.g.: CCEL, IMA log, ect.
*/
pub trait CVM {
    fn process_cc_report(&mut self, nonce: &String, data: &String) -> Result<Vec<u8>, anyhow::Error>;
    fn process_cc_measurement();
    fn process_cc_eventlog();

    fn parse_cc_report(&self);
    fn parse_cc_measurement(&self);
    fn parse_cc_eventlog(&self);

    fn dump_cc_report(report: Vec<u8>);
    fn dump_cc_measurement(&self);
    fn dump_cc_eventlog(&self);

    //Dump confidential VM information
    fn dump(&self);
}