pub struct TdsMessage {
    header: TdsHeader,
    body: Vec<u8>
}
impl TdsMessage {
    pub fn new() -> TdsMessage {
        let body: Vec<u8> = Vec::new();

        TdsMessage {
            header: TdsHeader::new(ClientMessageType::PreLogin, MessageStatus::EndOfMessage),
            body
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&self.header.to_byte_array());
        buffer.extend_from_slice(&self.body);
        buffer
    }

    pub fn calc_length(&mut self) {
        let length: u16 = 0x08 + self.body.len() as u16;
        self.header.length = length;
    }

    pub fn generate_prelogin(&mut self) {
        let version = SqlVersion::SqlServer2022.value();
        let encryption = EncryptionOptions::NoEncryption.value();
        let mars = MarsOptions::NoMars.value();
        let fed_auth = FedAuthOptions::No.value();
        let terminator = StaticValues::Terminator.value();

        let mut body: Vec<u8> = Vec::new();

        let mut offset_start = (5)+1; 

        body = TdsMessage::add_preflight(body, offset_start, 6, PreLoginOptionToken::Version);
        // offset_start += 6;
        // body = TdsMessage::add_preflight(body, offset_start, 1, PreLoginOptionToken::Encryption);
        // offset_start += 1;
        // body = TdsMessage::add_preflight(body, offset_start, 1, PreLoginOptionToken::Mars);
        // offset_start += 1;
        // body = TdsMessage::add_preflight(body, offset_start, 1, PreLoginOptionToken::FedAuthRequired);
        
        body.push(terminator);
        
        body.extend_from_slice(&version);
        // body.push(encryption);
        // body.push(mars);
        // body.push(fed_auth);

        self.body = body;
    }

    fn add_preflight(mut body: Vec<u8>, offset_start: u16, length: u16, option: PreLoginOptionToken) -> Vec<u8> {
        body.push(option.value()); //Option
        body.push((offset_start >> 8) as u8); //offset msb
        body.push((0xff & offset_start) as u8);
        body.push((length >> 8) as u8); //length msb
        body.push((0xff & length) as u8);

        body
    }
}

enum StaticValues {
    Terminator
}
impl StaticValues {
    fn value(&self) -> u8 {
        match self {
            StaticValues::Terminator => 0xff
        }
    }
}

enum PreLoginOptionToken {
    Version,
    Encryption,
    InStopT,
    ThreadId,
    Mars,
    TraceId,
    FedAuthRequired,
    NonceOpt,
    Terminator
}
impl PreLoginOptionToken {
    fn value(&self) -> u8 {
        match self {
            PreLoginOptionToken::Version => 0x00,
            PreLoginOptionToken::Encryption => 0x01,
            PreLoginOptionToken::InStopT => 0x02,
            PreLoginOptionToken::ThreadId => 0x03,
            PreLoginOptionToken::Mars => 0x04,
            PreLoginOptionToken::TraceId => 0x05,
            PreLoginOptionToken::FedAuthRequired => 0x06,
            PreLoginOptionToken::NonceOpt => 0x07,
            PreLoginOptionToken::Terminator => 0x08,
        }
    }
}

enum SqlVersion {
    SqlServer2022
}
impl SqlVersion {
    fn value(&self) -> [u8; 6] {
        match self {
            SqlVersion::SqlServer2022 => [0x10, 0x00, 0x7f, 0x10, 0x00, 0x00]
        }
    }
}
enum EncryptionOptions {
    NoEncryption,
    EncryptionEnabled,
    EncryptionRequested,
    EncryptionEnabledRequested
}
impl EncryptionOptions {
    fn value(&self) -> u8 {
        match self {
            EncryptionOptions::NoEncryption => 0x00,
            EncryptionOptions::EncryptionEnabled => 0x01,
            EncryptionOptions::EncryptionRequested => 0x02,
            EncryptionOptions::EncryptionEnabledRequested => 0x03
        }
    }
}
enum MarsOptions {
    NoMars,
    MarsRequested,
    MarsSupported,
    MarsRequestedSupportd
}
impl MarsOptions {
    fn value(&self) -> u8 {
        match self {
            MarsOptions::NoMars => 0x00,
            MarsOptions::MarsRequested => 0x01,
            MarsOptions::MarsSupported => 0x02,
            MarsOptions::MarsRequestedSupportd => 0x03
        }
    }
}
enum FedAuthOptions {
    Yes,
    No
}
impl FedAuthOptions {
    fn value(&self) -> u8 {
        match self {
            FedAuthOptions::Yes => 0x01,
            FedAuthOptions::No => 0x00 
        }
    }
}

struct TdsHeader {
    message_type: u8,
    status: u8,
    length: u16,
    spid: u16,
    packet_id: u8,
    window: u8
}
impl TdsHeader {
    pub fn new(message_type: ClientMessageType, status: MessageStatus) -> TdsHeader {
        TdsHeader {
            message_type: message_type.value(),
            status: status.value(), 
            length: 0x200,
            spid: 0x0000,
            packet_id: 0x00,
            window: 0x00
        }
    }

    fn update_message_type(&mut self, message_type: ClientMessageType) {
        self.message_type = message_type.value();
    }

    fn update_status(&mut self, status: MessageStatus) {
        self.status = status.value();
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

pub enum ClientMessageType {
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
            ClientMessageType::PreLogin => 0x12,
            ClientMessageType::Tds7Login => 0x10,
            ClientMessageType::SspiLogin => 0x11,
            ClientMessageType::FederatedAuthToken => 0x8,
            ClientMessageType::SqlBatch => 0x1,
            ClientMessageType::BulkLoad => 0x7,
            ClientMessageType::Rpc => 0x3,
            ClientMessageType::Attention => 0x6,
            ClientMessageType::TransactionManagerRequest => 0xE
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messagestatus_values() {
        let normal = MessageStatus::Normal.value();
        let eom = MessageStatus::EndOfMessage.value();
        let ignore = MessageStatus::Ignore.value();
        let resetconn = MessageStatus::ResetConnection.value();
        let resetconnskiptran = MessageStatus::ResetConnectionSkipTran.value();

        assert_eq!(normal, 0x00);
        assert_eq!(eom, 0x01);
        assert_eq!(ignore, 0x02);
        assert_eq!(resetconn, 0x08);
        assert_eq!(resetconnskiptran, 0x10);
    }

    #[test]
    fn test_clientmessagetype_values() {
        let prelogin = ClientMessageType::PreLogin.value();
        let tsdlogin = ClientMessageType::Tds7Login.value();
        let sspilogin = ClientMessageType::SspiLogin.value();
        let fedauth = ClientMessageType::FederatedAuthToken.value();
        let sqlbatch = ClientMessageType::SqlBatch.value();
        let bulkload = ClientMessageType::BulkLoad.value();
        let rpc = ClientMessageType::Rpc.value();
        let attn = ClientMessageType::Attention.value();
        let tmr = ClientMessageType::TransactionManagerRequest.value();

        assert_eq!(prelogin, 18);
        assert_eq!(tsdlogin, 16); 
        assert_eq!(sspilogin, 17); 
        assert_eq!(fedauth, 8); 
        assert_eq!(sqlbatch, 1); 
        assert_eq!(bulkload, 7); 
        assert_eq!(rpc, 3); 
        assert_eq!(attn, 6); 
        assert_eq!(tmr, 14); 
    }

    #[test]
    fn test_tdsheader_new_creates_instance() {
        let header = TdsHeader::new(ClientMessageType::PreLogin, MessageStatus::EndOfMessage);

        assert_eq!(header.message_type, ClientMessageType::PreLogin.value());
        assert_eq!(header.status, MessageStatus::Normal.value());
        assert_eq!(header.length, 512);
        assert_eq!(header.spid, 0x0000);
        assert_eq!(header.packet_id, 1);
        assert_eq!(header.window, 0x00);
    }

    #[test]
    fn test_tdsheader_update_message_type_updates() {
        let mut header: TdsHeader = TdsHeader::new(ClientMessageType::Tds7Login, MessageStatus::EndOfMessage);

        header.update_message_type(ClientMessageType::PreLogin);

        assert_eq!(header.message_type, ClientMessageType::PreLogin.value());
    }

    #[test]
    fn test_tdsheader_update_status_updates() {
        let mut header: TdsHeader = TdsHeader::new(ClientMessageType::Tds7Login, MessageStatus::EndOfMessage);

        header.update_status(MessageStatus::Normal);

        assert_eq!(header.status, MessageStatus::Normal.value());
    }

    #[test]
    fn test_tdsheader_tobytearray_returns_array() {
        let header = TdsHeader::new(ClientMessageType::PreLogin, MessageStatus::EndOfMessage);
        let bytes = header.to_byte_array();

        assert_eq!(bytes[0], header.message_type);
        assert_eq!(bytes[1], header.status);
        assert_eq!(bytes[2], (header.length >> 8) as u8);
        assert_eq!(bytes[3], (header.length & 0xff) as u8);
        assert_eq!(bytes[4], (header.spid >> 8) as u8);
        assert_eq!(bytes[5], (header.spid & 0xff) as u8);
        assert_eq!(bytes[6], header.packet_id);
        assert_eq!(bytes[7], header.window);
    }

    #[test]
    fn test_tdsmessage_new_creates_instance() {
        let message = TdsMessage::new();

        assert_eq!(message.header.message_type, ClientMessageType::PreLogin.value());
        assert_eq!(message.body[0], 1);
    }

    #[test]
    fn test_tdsmessage_generate_prelogin_generates_body() {
        let mut message = TdsMessage::new();

        message.generate_prelogin();
    }

    #[test]
    fn test_tdsmessage_tobytes_creates_bytes() {
        let message = TdsMessage::new();

        let bytes = message.to_bytes();
        assert_eq!(bytes.len(), 9);
    }
}