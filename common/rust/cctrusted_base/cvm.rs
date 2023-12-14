mod cvm

struct DeviceNode {
    device_path: string
}

//struct ExtraArgs {}

// struct CcReport {
//     //TODO!
// }

struct CcEventlogs {
    //TODO!
}

// struct CcIMRs {
//     //TODO!
// }

// the interfaces a cvm should support
pub trait CVM {

    pub fn get_cc_report();
    pub fn get_cc_measurement();
    pub fn get_cc_eventlog();

    pub fn dump(&self) {
        //Dump confidential VM information
        println!("======================================")
        println!("CVM type = {}", self.cc_type.cc_type_str)
        println!("CVM version = {}", self.version)
        println!("======================================")
    }
}