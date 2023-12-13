
struct TdReport {
    //TODO!
}

pub struct tdx_vm {
    tdx_version: Version
    device_node: TeeDeviceNode
    cc_type: CcType
    rtmrs: Imrs
    default_algo_id: AlgoId
    cc_report: CcReport
    tdx_report: TdReport
    tdx_rtrms: Vec!<TdxRTMR>
}

impl CVM for tdx_vm {

    pub fn process_cc_report(){
        //TODO!
    }

    pub fn process_eventlog(){
        //TODO!
    }

}