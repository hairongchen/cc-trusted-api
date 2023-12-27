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

impl CcType {
    // a function to detect the TEE type
    pub fn new() -> CcType {
        let mut tee_type = TeeType::PLAIN;
        if Path::new(TEE_TPM_PATH).exists() {
            tee_type = TeeType::TPM;
        } else if Path::new(TEE_TDX_1_0_PATH).exists() || Path::new(TEE_TDX_1_5_PATH).exists() {
            tee_type = TeeType::TDX;
        } else if Path::new(TEE_SEV_PATH).exists() {
            tee_type = TeeType::SEV;
        } else {
            // TODO add support for CCA and etc.
        }

        CcType {
            tee_type: tee_type.clone(),
            tee_type_str: TEE_NAME_MAP.get(&tee_type).unwrap().to_owned(),
        }
    }
}

// used for return of Boxed trait object in build_cvm()
pub trait BuildCVM: CVM + TcgAlgorithmRegistry {}

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

    /***
        retrive CVM eventlogs

        Args:
            start and count of eventlogs

        Returns:
            array of eventlogs
    */
    fn process_cc_eventlog(&self);

    /***
        retrive CVM type

        Args:
            None

        Returns:
            CcType of CVM
    */
    fn get_cc_type(&self) -> CcType;

    //Dump confidential CVM information
    fn dump(&self);
}
