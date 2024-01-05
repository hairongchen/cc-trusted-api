use hashbrown::HashMap;

pub const TPM_ALG_ERROR: u8 = 0x0;
pub const TPM_ALG_RSA: u8 = 0x1;
pub const TPM_ALG_SHA1: u8 = 0x4;
pub const TPM_ALG_SHA256: u8 = 0xB;
pub const TPM_ALG_SHA384: u8 = 0xC;
pub const TPM_ALG_SHA512: u8 = 0xD;
pub const TPM_ALG_ECDSA: u8 = 0x18;

// hash algorithm ID to algorithm name string map
lazy_static! {
    pub static ref ALGO_NAME_MAP: HashMap<u8, String> = {
        let mut map: HashMap<u8, String> = HashMap::new();
        map.insert(TPM_ALG_ERROR, "TPM_ALG_ERROR".to_string());
        map.insert(TPM_ALG_RSA, "TPM_ALG_RSA".to_string());
        map.insert(TPM_ALG_SHA1, "TPM_ALG_SHA1".to_string());
        map.insert(TPM_ALG_SHA256, "TPM_ALG_SHA256".to_string());
        map.insert(TPM_ALG_SHA384, "TPM_ALG_SHA384".to_string());
        map.insert(TPM_ALG_SHA512, "TPM_ALG_SHA512".to_string());
        map.insert(TPM_ALG_ECDSA, "TPM_ALG_ECDSA".to_string());
        map
    };
}

// this trait retrieve tcg standard algorithm name in string
pub trait TcgAlgorithmRegistry {
    fn get_algorithm_id(&self) -> u8;
}

// digest format: (algo id, hash value)
#[allow(dead_code)]
pub struct TcgDigest {
    algo_id: u8,
    hash: Vec<u8>,
}

// this trait retrieve IMR's max index of a CVM and hash value
pub trait TcgIMR {
    fn max_index(&self) -> u8;
    fn get_index(&self) -> u8;
    fn get_hash(&self) -> Vec<&str>;
    fn is_valid(&self) -> bool;
}

pub const EV_PREBOOT_CERT: u32 = 0x0;
pub const EV_POST_CODE: u32 = 0x1;
pub const EV_UNUSED: u32 = 0x2;
pub const EV_NO_ACTION: u32 = 0x3;
pub const EV_SEPARATOR: u32 = 0x4;
pub const EV_ACTION: u32 = 0x5;
pub const EV_EVENT_TAG: u32 = 0x6;
pub const EV_S_CRTM_CONTENTS: u32 = 0x7;
pub const EV_S_CRTM_VERSION: u32 = 0x8;
pub const EV_CPU_MICROCODE: u32 = 0x9;
pub const EV_PLATFORM_CONFIG_FLAGS: u32 = 0xa;
pub const EV_TABLE_OF_DEVICES: u32 = 0xb;
pub const EV_COMPACT_HASH: u32 = 0xc;
pub const EV_IPL: u32 = 0xd;
pub const EV_IPL_PARTITION_DATA: u32 = 0xe;
pub const EV_NONHOST_CODE: u32 = 0xf;
pub const EV_NONHOST_CONFIG: u32 = 0x10;
pub const EV_NONHOST_INFO: u32 = 0x11;
pub const EV_OMIT_BOOT_DEVICE_EVENTS: u32 = 0x12;

pub const EV_EFI_EVENT_BASE: u32 = 0x80000000;
pub const EV_EFI_VARIABLE_DRIVER_CONFIG: u32 = EV_EFI_EVENT_BASE + 0x1;
pub const EV_EFI_VARIABLE_BOOT: u32 = EV_EFI_EVENT_BASE + 0x2;
pub const EV_EFI_BOOT_SERVICES_APPLICATION: u32 = EV_EFI_EVENT_BASE + 0x3;
pub const EV_EFI_BOOT_SERVICES_DRIVER: u32 = EV_EFI_EVENT_BASE + 0x4;
pub const EV_EFI_RUNTIME_SERVICES_DRIVER: u32 = EV_EFI_EVENT_BASE + 0x5;
pub const EV_EFI_GPT_EVENT: u32 = EV_EFI_EVENT_BASE + 0x6;
pub const EV_EFI_ACTION: u32 = EV_EFI_EVENT_BASE + 0x7;
pub const EV_EFI_PLATFORM_FIRMWARE_BLOB: u32 = EV_EFI_EVENT_BASE + 0x8;
pub const EV_EFI_HANDOFF_TABLES: u32 = EV_EFI_EVENT_BASE + 0x9;
pub const EV_EFI_VARIABLE_AUTHORITY: u32 = EV_EFI_EVENT_BASE + 0x10;

lazy_static! {
    pub static ref TCG_EVENT_TYPE_NAME_MAP: HashMap<u32, String> = {
        let mut map: HashMap<u32, String> = HashMap::new();
        map.insert(EV_PREBOOT_CERT, "EV_PREBOOT_CERT".to_string());
        map.insert(EV_POST_CODE, "EV_POST_CODE".to_string());
        map.insert(EV_UNUSED, "EV_UNUSED".to_string());
        map.insert(EV_NO_ACTION, "EV_NO_ACTION".to_string());
        map.insert(EV_SEPARATOR, "EV_SEPARATOR".to_string());
        map.insert(EV_ACTION, "EV_ACTION".to_string());
        map.insert(EV_EVENT_TAG, "EV_EVENT_TAG".to_string());
        map.insert(EV_S_CRTM_CONTENTS, "EV_S_CRTM_CONTENTS".to_string());
        map.insert(EV_S_CRTM_VERSION, "EV_S_CRTM_VERSION".to_string());
        map.insert(EV_CPU_MICROCODE, "EV_CPU_MICROCODE".to_string());
        map.insert(EV_PLATFORM_CONFIG_FLAGS, "EV_PLATFORM_CONFIG_FLAGS".to_string());
        map.insert(EV_TABLE_OF_DEVICES, "EV_TABLE_OF_DEVICES".to_string());
        map.insert(EV_COMPACT_HASH, "EV_COMPACT_HASH".to_string());
        map.insert(EV_IPL, "EV_IPL".to_string());
        map.insert(EV_IPL_PARTITION_DATA, "EV_IPL_PARTITION_DATA".to_string());
        map.insert(EV_NONHOST_CODE, "EV_NONHOST_CODE".to_string());
        map.insert(EV_NONHOST_CONFIG, "EV_NONHOST_CONFIG".to_string());
        map.insert(EV_NONHOST_INFO, "EV_NONHOST_INFO".to_string());
        map.insert(EV_OMIT_BOOT_DEVICE_EVENTS, "EV_OMIT_BOOT_DEVICE_EVENTS".to_string());
        map.insert(EV_EFI_EVENT_BASE, "EV_EFI_EVENT_BASE".to_string());
        map.insert(EV_EFI_VARIABLE_DRIVER_CONFIG, "EV_EFI_VARIABLE_DRIVER_CONFIG".to_string());
        map.insert(EV_EFI_VARIABLE_BOOT, "EV_EFI_VARIABLE_BOOT".to_string());
        map.insert(EV_EFI_BOOT_SERVICES_APPLICATION, "EV_EFI_BOOT_SERVICES_APPLICATION".to_string());
        map.insert(EV_EFI_BOOT_SERVICES_DRIVER, "EV_EFI_BOOT_SERVICES_DRIVER".to_string());
        map.insert(EV_EFI_RUNTIME_SERVICES_DRIVER, "EV_EFI_RUNTIME_SERVICES_DRIVER".to_string());
        map.insert(EV_EFI_GPT_EVENT, "EV_EFI_GPT_EVENT".to_string());
        map.insert(EV_EFI_ACTION, "EV_EFI_ACTION".to_string());
        map.insert(EV_EFI_PLATFORM_FIRMWARE_BLOB, "EV_EFI_PLATFORM_FIRMWARE_BLOB".to_string());
        map.insert(EV_EFI_HANDOFF_TABLES, "EV_EFI_HANDOFF_TABLES".to_string());
        map.insert(EV_EFI_VARIABLE_AUTHORITY, "EV_EFI_VARIABLE_AUTHORITY".to_string());
        map
    };
}

pub struct TcgEventType {}

impl TcgEventType {
    //pub get_event_type_string
}