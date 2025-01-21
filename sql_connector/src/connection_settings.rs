use std::collections::HashMap;
use config::Config;
use std::fs;

pub struct ConnectionSettings {
    server: String,
    port: String,
    user: String,
    password: String,
    from_file: bool,
}

impl ConnectionSettings {
    pub fn new(server: &str, port: &str, user: &str, pass: &str) -> ConnectionSettings {
        ConnectionSettings {
            server: String::from(server),
            port: String::from(port),
            user: String::from(user),
            password: String::from(pass),
            from_file: false
        }
    }

    pub fn from_file() -> ConnectionSettings {
        let settings: HashMap<String, String> = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .unwrap()
            .try_deserialize::<HashMap<String, HashMap<String, String>>>()
            .unwrap()["connection_settings"].clone();

        return ConnectionSettings {
            server: settings["server"].to_string(),
            port: settings["port"].to_string(),
            user: settings["user"].to_string(),
            password: settings["password"].to_string(),
            from_file: true
        }
    }

    pub fn get(&self, field: &str) -> &str {
        match self.get_result(field) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Error in getting field {field}: {err}");
                ""
            }
        }        
    }

    fn get_result(&self, field_string: &str) -> Result<&str, String> {
        match field_string {
            "server" => Ok(&self.server),
            "user" => Ok(&self.user),
            "password" => Ok(&self.password),
            "port" => Ok(&self.port),
            _ => Err(format!("invalid field name to get '{}'", field_string))
        }
    }

    pub fn update(&mut self, field_string: &str, value: &str) -> Result<(), String>{
        match field_string {
            "server" => {
                self.server = String::from(value);
            },
            "user" => {
                self.user = String::from(value);
            }
            "password" => {
                self.password = String::from(value);
            }
            "port" => {
                self.port = String::from(value);
            }
            _ => ()
        };

        if self.from_file {
            self.save_config()?;
        }

        Ok(())
    }

    fn save_config(&self) -> Result<(), String> {
        //rebuild the serializable data structure. 
        let mut settings_map = HashMap::new();
        settings_map.insert("server", &self.server);
        settings_map.insert("port", &self.port);
        settings_map.insert("user", &self.user);
        settings_map.insert("password", &self.password);

        let mut config_data = HashMap::new();
        config_data.insert("connection_settings", settings_map);

        let serialized_config = toml::to_string(&config_data)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write("config.toml", serialized_config)
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connectionsettings_get_gets_field() {
        let settings: ConnectionSettings = ConnectionSettings {
            server: String::from("localhost"),
            port: String::from("1433"),
            user: String::from("sa"),
            password: String::from("SomeTestPass123!"),
            from_file: false

        };

        let server: &str = settings.get("server");
        let port: &str = settings.get("port");
        let user: &str = settings.get("user");
        let password: &str = settings.get("password");

        assert_eq!(server, String::from("localhost"));
        assert_eq!(port, "1433");
        assert_eq!(user, String::from("sa"));
        assert_eq!(password, String::from("SomeTestPass123!"));
    }

    #[test]
    fn test_connectionsettings_update_updates_field_value() {
        let mut settings: ConnectionSettings = ConnectionSettings {
            server: String::from("localhost"),
            port: String::from("1433"),
            user: String::from("sa"),
            password: String::from("SomeTestPass123!"),
            from_file: false
        };

        let new_server_value: &str = "https://localhost";
        let new_port_value: &str = "8080";
        let new_user_value: &str = "user";
        let new_pass_value: &str = "password";

        let _ = settings.update("server", new_server_value);
        let _ = settings.update("port", new_port_value);
        let _ = settings.update("user", new_user_value);
        let _ = settings.update("password", new_pass_value);

        assert_eq!(settings.server, new_server_value);
        assert_eq!(settings.port, new_port_value);
        assert_eq!(settings.user, new_user_value);
        assert_eq!(settings.password, new_pass_value);
    }

    #[test]
    fn test_connectionsettings_new_creates_instance() {
        let settings = ConnectionSettings::new(
            "localhost",
            "1433",
            "sa",
            "SomePassword123!",
        );

        assert_eq!(settings.server, "localhost");
        assert_eq!(settings.port, "1433");
        assert_eq!(settings.user, "sa");
        assert_eq!(settings.password, "SomePassword123!");
    }

    #[test]
    fn test_connectionsettings_fromfile_creates_instance() {
        //Update to use temp file at some point

        let settings: ConnectionSettings = ConnectionSettings::from_file();

        assert_eq!(settings.server, "localhost");
        assert_eq!(settings.port, "1433");
        assert_eq!(settings.user, "sa");
        assert_eq!(settings.password, "SomeTestPass123!");
    }

    #[test]
    fn test_connectionsettings_update_updates_file() {
        //Update to use temp file at some point
        
        let mut settings: ConnectionSettings = ConnectionSettings::from_file();
        let update_value = "https://localhost";

        let _ = settings.update("server", &update_value);
        assert_eq!(settings.server, update_value);

        let mut settings2: ConnectionSettings = ConnectionSettings::from_file();
        assert_eq!(settings2.server, update_value);

        let _ = settings2.update("server", "localhost");
    }
}
