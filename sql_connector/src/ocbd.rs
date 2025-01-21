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
    settings: ConnectionSettings
}

impl Connector {
    fn new(db_name: &str) -> Connector {
        let settings = ConnectionSettings::from_file();
        
        Connector {
            database: String::from(db_name),
            settings: settings
        }
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
}