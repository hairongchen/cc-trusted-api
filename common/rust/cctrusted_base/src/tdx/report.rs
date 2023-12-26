#![allow(non_camel_case_types)]
use anyhow::*;
use nix::*;
use sha2::{Digest, Sha512};
use std::fs::File;
use std::os::unix::io::AsRawFd;
use core::ptr;
use core::result::Result;
use core::result::Result::Ok;

use crate::tdx::common::*;
use crate::tdx::tdvm::TdxVM;

#[repr(C)]
#[allow(private_in_public)]
struct tdx_1_0_report_req {
    subtype: u8,     // Subtype of TDREPORT: fixed as 0 by TDX Module specification
    reportdata: u64, // User-defined REPORTDATA to be included into TDREPORT
    rpd_len: u32,    // Length of the REPORTDATA: fixed as 64 bytes by the TDX Module specification
    tdreport: u64,   // TDREPORT output from TDCALL[TDG.MR.REPORT]
    tdr_len: u32,    // Length of the TDREPORT: fixed as 1024 bytes by the TDX Module specification
}

#[repr(C)]
#[allow(private_in_public)]
struct tdx_1_5_report_req {
    reportdata: [u8; REPORT_DATA_LEN as usize], // User buffer with REPORTDATA to be included into TDREPORT
    tdreport: [u8; TDX_REPORT_LEN as usize], // User buffer to store TDREPORT output from TDCALL[TDG.MR.REPORT]
}

impl Tdx{
    pub fn generate_tdx_report_data(
        nonce: String,
        data: Option<String>,
    ) -> Result<String, anyhow::Error> {
        let nonce_decoded = match base64::decode(nonce) {
            Ok(v) => v,
            Err(e) => {
                return Err(anyhow!(
                    "[generate_tdx_report_data] nonce is not base64 encoded: {:?}",
                    e
                ))
            }
        };
        let mut hasher = Sha512::new();
        hasher.update(nonce_decoded);
        let _ret = match data {
            Some(_encoded_data) => {
                if _encoded_data.is_empty() {
                    hasher.update("")
                } else {
                    let decoded_data = match base64::decode(_encoded_data) {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(anyhow!(
                                "[generate_tdx_report_data] user data is not base64 encoded: {:?}",
                                e
                            ))
                        }
                    };
                    hasher.update(decoded_data)
                }
            }
            None => hasher.update(""),
        };
        let hash_array: [u8; 64] = hasher
            .finalize()
            .as_slice()
            .try_into()
            .expect("[generate_tdx_report_data] Wrong length of report data");
        Ok(base64::encode(hash_array))
    }
    
    pub fn prepare_tdx_1_0_report_request(
        report_data: String,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let report_data_bytes = match base64::decode(report_data) {
            Ok(v) => v,
            Err(e) => return Err(anyhow!("report data is not base64 encoded: {:?}", e)),
        };
    
        //prepare get TDX report request data
        let mut report_data_array: [u8; REPORT_DATA_LEN as usize] = [0; REPORT_DATA_LEN as usize];
        report_data_array.copy_from_slice(&report_data_bytes[0..]);
        let td_report: [u8; TDX_REPORT_LEN as usize] = [0; TDX_REPORT_LEN as usize];
    
        //build the request
        tdx_1_0_report_req {
            subtype: 0 as u8,
            reportdata: ptr::addr_of!(report_data_array) as u64,
            rpd_len: REPORT_DATA_LEN,
            tdreport: ptr::addr_of!(td_report) as u64,
            tdr_len: TDX_REPORT_LEN,
        }
    }

    pub fn prepare_tdx_1_5_report_request(
        report_data: String,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let report_data_bytes = match base64::decode(report_data) {
            Ok(v) => v,
            Err(e) => return Err(anyhow!("report data is not base64 encoded: {:?}", e)),
        };
    
        //prepare get TDX report request data
        let mut request = tdx_1_5_report_req {
            reportdata: [0; REPORT_DATA_LEN as usize],
            tdreport: [0; TDX_REPORT_LEN as usize],
        };
        request.reportdata.copy_from_slice(&report_data_bytes[0..]);
    
        request
    }
    
}