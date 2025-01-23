use std::io::{Write, Read};
use std::net::{TcpStream, Shutdown};
use crate::connection_settings::ConnectionSettings;
use crate::tds_message::TdsMessage;
/**
 * OCDB Driver
 * 
 * First iteration lets just focus on MS SQL connections. 
 * This uses the Tabular Data Stream (TDS) protocol. 
 * 
 * https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-tds/893fcc7e-8a39-4b3c-815a-773b7b982c50
 */

struct Connector {
    database: String,
    settings: ConnectionSettings,
    stream: Option<TcpStream>,
    authenticated: bool
}

impl Connector {
    pub fn new(db_name: &str) -> Connector {
        let settings = ConnectionSettings::from_file();
        
        Connector {
            database: String::from(db_name),
            settings,
            stream: None,
            authenticated: false
        }
    }

    pub fn connect(&mut self) -> Result<bool, String> {
        let server = self.settings.get("server");
        let port = self.settings.get("port");

        let addr = format!("{server}:{port}");

        let stream = TcpStream::connect(addr);

        match stream {
            Ok(_) => self.save_connection(stream.unwrap()), // Connection successful
            Err(err) => Err(format!("Failed to connect: {}", err))
        }
    }

    fn save_connection(&mut self, stream: TcpStream) -> Result<bool, String> {
        self.stream = Some(stream);
        Ok(true)
    } 

    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    pub fn get_stream(&mut self) -> TcpStream {
        let stream: TcpStream = self.stream.take().expect("No active stream");
        stream
    } 

    pub fn authenticate(&mut self) -> Result<bool, String> {
        if !self.is_connected() {
            return Err(format!("Not connected to server. Please call connect first"));
        }
        
        let mut message: TdsMessage = TdsMessage::new();

        message.generate_prelogin();
        message.calc_length();

        let bytes:Vec<u8> = message.to_bytes();
        for byte in &bytes {
            print!("0x{:02X} ", byte);
        }
        println!();

        let mut stream: TcpStream = self.stream.take().ok_or("No active stream")?;

        let result = stream.write_all(&bytes).map_err(|e| format!("Failed to write to steam: {}", e));
        println!("result: {:?}", result);

        let mut read_buffer = [0u8; 1024];
        let size = stream.read(&mut read_buffer).map_err(|e| e.to_string())?;

        println!("response: {:?}, size: {size}", &read_buffer[..size]);
        
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connector_new_creates_instance() {
        let db_name: &str = "sample";
        let con: Connector = Connector::new(db_name);

        assert_eq!(con.database, db_name);

        let server = con.settings.get("server");
        assert_eq!(server, "localhost");
    }

    #[test]
    fn test_connector_get_stream_returns_stream() {
        let db_name: &str = "sample";
        let mut con: Connector = Connector::new(db_name);

        let _ = con.connect();

        let stream:TcpStream = con.get_stream();

        let _ = stream.shutdown(Shutdown::Both);
    }

    #[test]
    fn test_connector_connect_establishes_connection() {
        let db_name: &str = "sample";
        let mut con: Connector = Connector::new(db_name);

        let result = con.connect();

        let stream = con.get_stream();
        let _ = stream.shutdown(Shutdown::Both);

        match result {
            Ok(value) => assert_eq!(value, true),
            Err(err) => assert_eq!(err, "")
        };
    }

    #[test]
    fn test_connector_connect_fails_on_wrong_settings() {
        let settings: ConnectionSettings = ConnectionSettings::new("127.0.0.1", "8080", "sa", "pass");

        let mut con: Connector = Connector {
            database: String::from("sample"),
            settings,
            stream: None,
            authenticated: false
        };

        let result = con.connect();

        match result {
            Ok(value) => assert_eq!(value, false),
            Err(err) => assert!(err.contains("Failed to connect: No connection could be made because the target machine actively refused it. (os error 10061)"))
        }
    }

    #[test]
    fn test_connector_is_connected_success() {
        let db_name = "sample";
        let mut con: Connector = Connector::new(db_name);
        
        let _ = con.connect();

        assert!(con.is_connected());

        let stream = con.get_stream();
        let _ = stream.shutdown(Shutdown::Both);
    }

    #[test]
    fn test_connector_is_connected_fails() {
        let db_name = "sample";
        let con: Connector = Connector::new(db_name);

        assert!(!(con.is_connected()));
    }

    #[test]
    fn test_connector_can_authenticate() {
        let db_name = "sample";
        let mut con: Connector = Connector::new(db_name);

        let _ = con.connect();
        let result = con.authenticate();

        match result {
            Ok(value) => assert_eq!(value, true),
            Err(err) => assert_eq!(err, "")
        }
    }
}