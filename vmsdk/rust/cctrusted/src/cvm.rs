use anyhow::*;
use cctrusted_base::tcg::{TcgDigest,TcgAlgorithmRegistry};
use cctrusted_base::cc_type::CcType;
use cctrusted_base::cc_type::TeeType;
use crate::tdvm::TdxVM;

// holds the device node info
pub struct DeviceNode {
    pub device_path: String,
}

pub struct CcEventlogs {}

// used for return of Boxed trait object in build_cvm()
pub trait BuildCVM: CVM + TcgAlgorithmRegistry {}

pub fn build_cvm() -> Result<Box<dyn BuildCVM>, anyhow::Error> {
    // instance a CVM according to detected TEE type
    match CcType::new().tee_type {
        TeeType::TDX => Ok(Box::new(TdxVM::new())),
        TeeType::SEV => todo!(),
        TeeType::CCA => todo!(),
        TeeType::TPM => todo!(),
        TeeType::PLAIN => return Err(anyhow!("[build_cvm] Error: not in any TEE!")),
    }
}

// the interfaces a CVM should implement
pub trait CVM {
    /***
        retrive CVM signed report

        Args:
            nonce (String): against replay attacks
            data (String): user data

        Returns:
            the cc report byte array or error information
    */
    fn process_cc_report(&mut self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error>;

    /***
        retrive CVM measurement registers, e.g.: RTMRs, vTPM PCRs, etc.

        Args:
            index (u8): the index of measurement register,
            algo_id (u8): the alrogithms ID

        Returns:
            TcgDigest struct
    */
    fn process_cc_measurement(&self, _index: u8, _algo_id: u8) -> TcgDigest;

    //TODO!
    fn process_cc_eventlog(&self);

    fn get_cc_type(&self) -> CcType;

    //Dump confidential CVM information
    fn dump(&self);
}

// TdxVM implements the interfaces defined in CVM trait
impl CVM for TdxVM {
    // retrieve TDX quote
    fn process_cc_report(&mut self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error> {

        let tdreport = match self.get_td_report(nonce, data) {
            Ok(r) => r,
            Err(e) => return Err(anyhow!("[process_cc_report] error getting TD report: {:?}", e)),
        };

        let report_data_array: [u8; TDX_REPORT_LEN as usize] = match tdreport.try_into() {
            Ok(r) => r,
            Err(e) => return Err(anyhow!("[get_tdx_quote] Wrong TDX report format: {:?}", e)),
        };
    
        //build QGS request message
        let qgs_msg = Tdx::generate_qgs_quote_msg(report_data_array);
    
        //build quote generation request header
        let mut quote_header = tdx_quote_hdr {
            version: 1,
            status: 0,
            in_len: (mem::size_of_val(&qgs_msg) + 4) as u32,
            out_len: 0,
            data_len_be_bytes: (1048 as u32).to_be_bytes(),
            data: [0; TDX_QUOTE_LEN as usize],
        };
    
        let qgs_msg_bytes = unsafe {
            let ptr = &qgs_msg as *const qgs_msg_get_quote_req as *const u8;
            core::slice::from_raw_parts(ptr, mem::size_of::<qgs_msg_get_quote_req>())
        };
        quote_header.data[0..(16 + 8 + TDX_REPORT_LEN) as usize]
            .copy_from_slice(&qgs_msg_bytes[0..((16 + 8 + TDX_REPORT_LEN) as usize)]);
    
        let tdx_quote_request = tdx_quote_req {
            buf: ptr::addr_of!(quote_header) as u64,
            len: TDX_QUOTE_LEN as u64,
        };

        let device_node = match File::options()
        .read(true)
        .write(true)
        .open(self.device_node.device_path.clone())
        {
            Err(e) => {
                return Err(anyhow!(
                    "[get_td_report] Fail to open {}: {:?}",
                    self.device_node.device_path,
                    e
                ))
            }
            Ok(fd) => fd,
        };

        //build the operator code and apply the ioctl command
        match self.version {
            TdxVersion::TDX_1_0 => {
                ioctl_read!(
                    get_quote_1_0_ioctl,
                    b'T',
                    TdxOperation::TDX_1_0_GET_QUOTE,
                    u64
                );
                match unsafe {
                    get_quote_1_0_ioctl(device_node.as_raw_fd(), ptr::addr_of!(tdx_quote_request) as *mut u64)
                } {
                    Err(e) => {
                        return Err(anyhow!("[get_tdx_quote] Fail to get TDX quote: {:?}", e))
                    }
                    Ok(_r) => _r,
                };
            }
            TdxVersion::TDX_1_5 => {
                ioctl_read!(
                    get_quote_1_5_ioctl,
                    b'T',
                    TdxOperation::TDX_1_5_GET_QUOTE,
                    tdx_quote_req
                );
                match unsafe {
                    get_quote_1_5_ioctl(
                        device_node.as_raw_fd(),
                        ptr::addr_of!(tdx_quote_request) as *mut tdx_quote_req,
                    )
                } {
                    Err(e) => {
                        return Err(anyhow!("[get_tdx_quote] Fail to get TDX quote: {:?}", e))
                    }
                    Ok(_r) => _r,
                };
            }
        };
    
        //inspect the response and retrive quote data
        let out_len = quote_header.out_len;
        let qgs_msg_resp_size =
            unsafe { core::mem::transmute::<[u8; 4], u32>(quote_header.data_len_be_bytes) }.to_be();
    
        let qgs_msg_resp = unsafe {
            let raw_ptr = ptr::addr_of!(quote_header.data) as *mut qgs_msg_get_quote_resp;
            raw_ptr.as_mut().unwrap() as &mut qgs_msg_get_quote_resp
        };
    
        if out_len - qgs_msg_resp_size != 4 {
            return Err(anyhow!(
                "[get_tdx_quote] Fail to get TDX quote: wrong TDX quote size!"
            ));
        }
    
        if qgs_msg_resp.header.major_version != 1
            || qgs_msg_resp.header.minor_version != 0
            || qgs_msg_resp.header.msg_type != 1
            || qgs_msg_resp.header.error_code != 0
        {
            return Err(anyhow!(
                "[get_tdx_quote] Fail to get TDX quote: QGS response error!"
            ));
        }
    
        Ok(qgs_msg_resp.id_quote[0..(qgs_msg_resp.quote_size as usize)].to_vec())
    }

    // retrieve TDX RTMR
    fn process_cc_measurement(&self, _index: u8, _algo_id: u8) -> TcgDigest {
        todo!()
    }

    // retrieve TDX CCEL and IMA eventlog
    fn process_cc_eventlog(&self) -> () {
        todo!()
    }

    fn get_cc_type(&self) -> CcType {
        return self.cc_type.clone();
    }

    fn dump(&self) {
        info!("======================================");
        info!("CVM type = {}", self.cc_type.tee_type_str);
        info!(
            "CVM version = {}",
            TDX_VERSION_MAP.get(&self.version).unwrap().to_owned()
        );
        info!("======================================");
    }
}