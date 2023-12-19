use crate::cc_type::*;
use crate::cvm::*;
use crate::tdx::rtmr::TdxRTMR;
use crate::tdx::common::*;
use crate::tdx::report::generate_tdx_report_data;
use crate::tcg::{TcgAlgorithmRegistry, ALGO_NAME_MAP};

use anyhow::*; 
use std::result::Result::Ok;

//struct TdReport {}

//struct TdxQuote {}
/*
    TdxVM is an abstraction of TDX running environment, it contains:
        cc_type: should always be TDX
        version: 1.0 or 1.5
        device_node: /dev/tdx-guest or /dev/tdx_guest
        algo_id: should be TPM_ALG_SHA384
        cc_report_raw: the raw tdx quote in byte vector, filled by get_cc_report()
        td_report_raw: the raw td report in byte vector, filled by get_cc_report()
        rtrms: TDX rtmr algorithm and hash, filled by get_cc_measurement()
*/
pub struct TdxVM {
    pub cc_type: CcType,
    pub version: TdxVersion,
    pub device_node: DeviceNode,
    pub algo_id: u8,
    pub cc_report_raw: Vec<u8>,
    //cc_report: Option<CcReport>,
    pub td_report_raw: Vec<u8>,
    //td_report: Option<TdReport>,
    pub rtrms: Vec<TdxRTMR>,
}

// implement the structure create function
impl TdxVM {
    pub fn new() -> TdxVM {
        let cc_type = CcType{tee_type: TeeType::TDX, tee_type_str: TEE_NAME_MAP.get(&TeeType::TDX).unwrap().to_owned()};

        let version = get_tdx_version();
        let device_node = DeviceNode {device_path: TDX_DEVICE_NODE_MAP.get(&version).unwrap().to_owned()};
        let algo_id = crate::tcg::TPM_ALG_SHA384;

        TdxVM {
            cc_type,
            version,
            device_node,
            algo_id,
            cc_report_raw: Vec::new(),
            td_report_raw: Vec::new(),
            rtrms: Vec::new()
        }
    }
}

// all TdxVM's interfaces should implement CVM trait
impl CVM for TdxVM {
    // retrieve TDX quote
    fn process_cc_report(&mut self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error>{
        let report_data = match generate_tdx_report_data(nonce, Some(data)) {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!(
                    "[get_cc_report] error generating TDX report data: {:?}",
                    e
                ))
            }
        };

        self.cc_report_raw = match self.get_tdx_quote(report_data) {
            Ok(q) => q,
            Err(e) => return Err(anyhow!(
                "[get_cc_report] error getting TDX quote: {:?}",
                e
            )),
        };

        Ok(self.cc_report_raw.clone())
    }

    // retrieve TDX RTMR
    fn process_cc_measurement() -> () {
        todo!()
    }

    // retrieve TDX CCEL and IMA eventlog
    fn process_cc_eventlog() -> () {
        todo!()
    }

    fn parse_cc_report(&self) {
        todo!()
    }

    fn parse_cc_measurement(&self){
        todo!()
    }
    fn parse_cc_eventlog(&self){
        todo!()
    }

    fn dump_cc_report(report: Vec<u8>){
        let mut index: usize = 0;
        let mut linestr = "".to_string();
        let mut printstr = "".to_string();

        let printable = vec![
            ' ','\t','\n','\r',
            'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
            'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
            '0','1','2','3','4','5','6','7','8','9',
            'a','b','c','d','e','f',
            'A','B','C','D','E','F',
            '#','$','%','&','\'','(',')','*','+',',','-','.','/',':',';','<','=','>','?','@','[','\\',']','^','_','`','{','|','}','~','"','!'
        ];

        while usize::from(index) < report.len() {
            if index % 16 == 0 {
                if printstr.len() != 0 {
                    println!("{} {}", linestr, printstr);
                    printstr = "".to_string();
                }
                linestr = format!("{:08X} ", ((index/16) as u16)*16);
            }

            let v = report[index];
            linestr.push_str(format!("{:02X} ", v).as_str());    
            match printable.iter().position(|&c| c == (v as char)) {
                Some(_) => {
                    if v < 0x9 || v > 0xD {
                        printstr.push_str(std::str::from_utf8(&[v]).unwrap());
                    }

                }
                None => printstr.push_str("."),
            }

            index += 1;
        }

        if index % 16 != 0 {
            let mut blank = "".to_string();
            for _ in 0..=(16 - index % 16) {
                blank.push_str("   ");
            }
            println!("{}{} {}", linestr, blank, printstr);
        } else if usize::from(index) == report.len() {
            println!("{} {}", linestr, printstr);
        }

    }

    fn dump_cc_measurement(&self){
        todo!()
    }
    fn dump_cc_eventlog(&self){
        todo!()
    }

    fn dump(&self) {
        println!("======================================");
        println!("CVM type = {}", self.cc_type.tee_type_str);
        println!("CVM version = {}", TDX_VERSION_MAP.get(&self.version).unwrap().to_owned());
        println!("======================================");
    }

}

impl TcgAlgorithmRegistry for TdxVM {
    fn get_algorithm_string(&self) -> String{
        ALGO_NAME_MAP.get(&self.algo_id).unwrap().to_owned()
    }
}