pub struct TdsMessage {
    header: TdsHeader,
    body: Vec<u8>
}

struct TdsHeader {
    message_type: u8,
    status: u8,
    length: u16,
    spid: u16,
    packet_id: u8,
    window: u8
}

enum ClientMessageType {
    PreLogin,
    Tds7Login,
    SspiLogin,
    FederatedAuthToken,
    SqlBatch,
    BulkLoad,
    Rpc,
    Attention,
    TransactionManagerRequest
}
impl ClientMessageType {
    fn value(&self) -> u8 {
        match self {
            ClientMessageType::PreLogin => 18,
            ClientMessageType::Tds7Login => 16,
            ClientMessageType::SspiLogin => 17,
            ClientMessageType::FederatedAuthToken => 8,
            ClientMessageType::SqlBatch => 1,
            ClientMessageType::BulkLoad => 7,
            ClientMessageType::Rpc => 3,
            ClientMessageType::Attention => 6,
            ClientMessageType::TransactionManagerRequest => 14
        }
    }
}

enum MessageStatus {
    Normal,
    EndOfMessage,
    Ignore,
    ResetConnection,
    ResetConnectionSkipTran
}
impl MessageStatus {
    fn value(&self) -> u8 {
        match self {
            MessageStatus::Normal => 0x00,
            MessageStatus::EndOfMessage => 0x01,
            MessageStatus::Ignore => 0x02,
            MessageStatus::ResetConnection => 0x08,
            MessageStatus::ResetConnectionSkipTran => 0x10
        }
    }
}


impl TdsHeader {
    pub fn new(message_type: ClientMessageType, status: MessageStatus, length: u16, spid: u16, packet_id: u8) -> TdsHeader {
        TdsHeader {
            message_type: message_type.value(),
            status: status.value(), 
            length,
            spid,
            packet_id,
            window: 0x00
        }
    }

    fn to_byte_array(&self) -> [u8;8] {
        let mut header: [u8; 8] = [0; 8];
        header[0] = self.message_type;
        header[1] = self.status;
        header[2] = (self.length >> 8) as u8;
        header[3] = (self.length & 0xff) as u8;
        header[4] = (self.spid >> 8) as u8;
        header[5] = (self.spid & 0xff) as u8;
        header[6] = self.packet_id;
        header[7] = self.window;

        header
    }
}