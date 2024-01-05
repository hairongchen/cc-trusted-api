use crate::tcg::EventLogEntryType;

/***
    TcgEventLog struct.

    This class contains the event logs following TCG specification.

    Attributes:
        data: raw data containing all boot time event logs
        event_logs: all parsed event logs
        count: total number of event logs
 */
pub struct TcgEventLog {
    data: Vec<u8>,
    event_logs: Vec<EventLogEntryType>,
    count: u32
}

