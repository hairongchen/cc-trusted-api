pub struct DeviceNode {
    device_path: String
}

pub struct CcEventlogs {
    //TODO!
}

// the interfaces a cvm should support
pub trait CVM {

    fn process_cc_report(&self, nonce: String, data: String);
    fn process_cc_measurement();
    fn process_cc_eventlog();

    fn get_cc_report();
    fn get_cc_measurement();
    fn get_cc_eventlog();

    //Dump confidential VM information
    fn dump(&self) {
        println!("======================================");
        println!("CVM type = {}", self.cc_type.cc_type_str);
        println!("CVM version = {}", self.version);
        println!("======================================");
    }
}