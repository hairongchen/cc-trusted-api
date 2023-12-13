
struct TeeDeviceNode {
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
    pub fn process_cc_report();
    pub fn process_eventlog();
    pub fn dump();
}