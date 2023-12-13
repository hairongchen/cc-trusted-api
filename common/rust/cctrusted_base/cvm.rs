
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

struct Version {
    version_str: String
}

pub trait CVM {
    fn process_cc_report();
    fn process_eventlog();
    pub fn get_cc_report();
    pub fn get_cc_measurement();
    pub fn get_cc_eventlog();
    pub fn dump();
}