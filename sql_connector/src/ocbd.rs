use std::net::TcpStream;

use crate::connection_settings::ConnectionSettings;

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
    stream: Option<TcpStream>
}

impl Connector {
    pub fn new(db_name: &str) -> Connector {
        let settings = ConnectionSettings::from_file();
        
        Connector {
            database: String::from(db_name),
            settings,
            stream: None
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
    fn test_connector_connect_establishes_connection() {
        let db_name: &str = "sample";
        let mut con: Connector = Connector::new(db_name);

        let result = con.connect();

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
            stream: None
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
    }

    #[test]
    fn test_connector_is_connected_fails() {
        let db_name = "sample";
        let con: Connector = Connector::new(db_name);

        assert!(!(con.is_connected()));
    }
}