
struct DeviceNode {
    device_path: string
}

struct CcReport {
    //TODO!
}

struct CcEventlogs {
    //TODO!
}

struct CcIMRs {
    //TODO!
}

// the interfaces a cvm should support
pub trait CVM {
    fn process_cc_report();
    fn process_eventlog();
    pub fn get_cc_report();
    pub fn get_cc_measurement();
    pub fn get_cc_eventlog();
    
    pub fn dump(&self) {
        //Dump confidential VM information
        println!("======================================")
        println!("CVM type = %s", self.cc_type.cc_type_str)
        println!("CVM version = %s", self.version)
        println!("======================================")
    }
}