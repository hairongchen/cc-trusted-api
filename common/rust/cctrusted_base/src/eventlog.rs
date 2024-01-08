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

    pub fn new(data: Vec<u8>) -> TcgEventLog {
        TcgEventLog{
            data: data,
            event_logs: Vec::new(),
            count: 0
        }
    }

    pub fn show(&self) {
        todo!()
    }

    /***
        Collect selected event logs according to user input.

        Args:
            start: index of the first event log to collect
            count: total number of event logs to collect
    */
    pub fn select(&self) -> Result<Vec<EventLogEntryType>, anyhow::Error>{
        todo!()
    }

    /***
        Parse event log data into TCG compatible forms.

        Go through all event log data and parse the contents accordingly
        Save the parsed event logs into TcgEventLog.
    */
    fn parse(&mut self) Result<(), anyhow::Error>{
        if self.data.len() == 0 {
            return Err(anyhow!("[parse] no eventlog data provided"));
        }

        let mut index = 0;
        while index < self.data.len() {
            let start = index;
            let imr = self.data[index..index+4];
            let event_type = self.data[index+4..index+8];
            index = index + 8;

            if imr == 0xFFFFFFFF {
                break;
            }

            if event_type = EV_NO_ACTION {
                let (spec_id_event, event_len) = parse_spec_id_event_log(self.data[start..]);
                index = start + event_len;
                self.eventlog.push(spec_id_event);
                self.count = self.count + 1;
            } else {
                let (event_log, event_len) = parse_event_log(self.data[start..]);
                index = start + event_len;
                self.eventlog.push(event_log);
                self.count = self.count + 1;
            }
        }
    }

    /***
        Parse TCG specification Id event according to TCG spec at
        https://trustedcomputinggroup.org/wp-content/uploads/TCG_PCClientSpecPlat_TPM_2p0_1p04_pub.pdf.

        Event Structure:
        typedef tdTCG_PCClientPCREvent {
            2735 UINT32 pcrIndex;
            UINT32 eventType;
            BYTE digest[20];
            UINT32 eventDataSize;
            BYTE event[eventDataSize]; //This is actually a TCG_EfiSpecIDEventStruct
        } TCG_PCClientPCREvent;

        Args:
            data: event log data in bytes

        Returns:
            A TcgPcClientImrEvent containing the Specification ID version event
            An int specifying the event size
    */
    fn parse_spec_id_event_log(&self, data: Vec<u8>) -> (spec_id_event: TcgPcClientImrEvent, event_len: u32) {
        todo!()
    }

    /***
        Parse TCG event log body as single event log entry (TcgImrEventLogEntry) defined at
        https://trustedcomputinggroup.org/wp-content/uploads/TCG_PCClientSpecPlat_TPM_2p0_1p04_pub.pdf

        typedef struct tdTCG_PCR_EVENT2{
            UINT32 pcrIndex;
            UINT32 eventType;
            TPML_DIGEST_VALUES digests;
            UINT32 eventSize;
            BYTE event[eventSize];
        } TCG_PCR_EVENT2;

        Args:
            data: event log data in bytes

        Returns:
            A TcgImrEvent containing the event information
            An int specifying the event size
    */
    fn parse_event_log(&self, data: Vec<u8>)-> (spec_id_event: TcgImrEvent, event_len: u32) {
        todo!()
    }
}

