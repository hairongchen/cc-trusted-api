use mod cc_type
use mod tcg
use mod common
use mod cvm
use mod quote
use mod report

struct TdReport {
    todo!()
}

struct TdxQuote {
    todo!()
}

/*
    TdxVM is an abstraction of TDX running environment, it contains:
        cc_type: should always be TDX
        version: 1.0 or 1.5
        device_node: /dev/tdx-guest or /dev/tdx_guest
        algo_id: should be TPM_ALG_SHA384
        cc_report_raw: the raw tdx quote in byte vector, filled by get_cc_report()
        cc_report: the parsed tdx quote, filled by get_cc_report()
        td_report_raw: the raw td report in byte vector, filled by get_cc_report()
        td_report: the parsed tdreport, filled by get_cc_report()
        rtrms: TDX rtmr algorithm and hash, filled by get_cc_measurement()
*/
pub struct TdxVM {
    cc_type: CcType
    version: TdxVersion
    device_node: DeviceNode
    algo_id: u8
    cc_report_raw: Option<Vec<u8>>
    cc_report: Option<CcReport>
    td_report_raw: Option<Vec<u8>>
    td_report: Option<TdReport>
    rtrms: Option<Vec!<TdxRTMR>>
}

// implement the structure create function
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
    // retrieve TDX quote
    pub fn get_cc_report(&self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error>{
        let report_data = match generate_tdx_report_data(nonce, data) {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!(
                    "[get_cc_report] error generating TDX report data: {:?}",
                    e
                ))
            }
        };

        let self.cc_report_raw = match get_tdx_quote(&self, report_data) {
            Ok(q) => q,
            Err(e) => return Err(anyhow!(
                "[get_cc_report] error getting TDX quote: {:?}",
                e
            )),
        }
    }

    // retrieve TDX RTMR
    pub fn get_cc_measurement();
    // retrieve TDX CCEL and IMA eventlog
    pub fn get_cc_eventlog();

}