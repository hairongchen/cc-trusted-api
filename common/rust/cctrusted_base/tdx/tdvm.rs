
use mod cctype
use mod tcg

struct TdReport {
    //TODO!
}

pub struct TdxVM {
use mod cctype
    cc_type: CcType
    version: TdxVersion
    device_node: DeviceNode
    algo_id: u8
    // cc_report: CcReport
    // tdx_report: TdReport
    // tdx_rtrms: Vec!<TdxRTMR>
}

impl TdxVM {
    pub fn new() -> Result<TdxVM, anyhow::Errort> {
        let cc_type = cctype::detect_cc_type();
        if cc_type.tee_type != TeeType::TDX {
            panic!("Not in TDX enviroment")
        }

        let version = get_tdx_version();

        let device_node = DeviceNode {device_path: TdxDeviceNodeMap.get(version)};

        let algo_id = tcg::TPM_ALG_SHA384;

        TdxVM {
            cc_type,
            version,
            device_node,
            algo_id
        }
    }
}

// all TdxVM's interfaces should implement CVM trait
impl CVM for TdxVM {
    pub fn get_cc_report();
    pub fn get_cc_measurement();
    pub fn get_cc_eventlog();

}