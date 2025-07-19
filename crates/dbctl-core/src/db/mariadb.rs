use super::traits::Database;

#[derive(Clone)]
pub struct MariaDB {
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: u16,
    pub db_name: String,
    pub host: String,
    pub root_password: String,
}

impl Database for MariaDB {
    fn name(&self) -> &str {
        &self.name
    }

    fn image(&self) -> &str {
        "mariadb:10.11"
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn env_vars(&self) -> Vec<(String, String)> {
        vec![
            ("MARIADB_USER".into(), self.user.clone()),
            ("MARIADB_PASSWORD".into(), self.password.clone()),
            ("MARIADB_DATABASE".into(), self.db_name.clone()),
            ("MARIADB_ROOT_PASSWORD".into(), self.root_password.clone()),
        ]
    }

    fn connection_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }

    fn default() -> Self {
        MariaDB {
            name: "mariadb-default".to_string(),
            user: "admin".to_string(),
            password: "secret".to_string(),
            port: 3306,
            db_name: "mydb".to_string(),
            host: "localhost".to_string(),
            root_password: "rootsecret".to_string(),
        }
    }
}