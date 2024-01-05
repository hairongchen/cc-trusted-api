use crate::tcg::EventLogEntryType;
use anyhow::anyhow;

/***
    TcgEventLog struct.

    This class contains the event logs following TCG specification.

    Attributes:
        data: raw data containing all boot time event logs
        event_logs: all parsed event logs
        count: total number of event logs
 */
pub struct TcgEventLog {
    pub data: Vec<u8>,
    pub event_logs: Vec<EventLogEntryType>,
    pub count: u32
}

impl TcgEventLog {

    pub fn new() -> Result<TcgEventLog, anyhow::Error>{
        todo!()
    }

    pub fn show(&self) {
        todo!()
    }

    pub fn select(&self) Result<Vec<EventLogEntryType>, anyhow::Error>{
        todo!()
    }

    fn parse(&self) {
        todo!()
    }

    fn parse_spec_id_event_log(&self, data: Vec<u8>) {
        todo!()
    }

    fn parse_event_log(&self, data: Vec<u8>) {
        todo!()
    }
}

