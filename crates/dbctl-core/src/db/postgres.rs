use super::traits::Database;

pub struct Postgres {
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: u16,
    pub db_name: String,
    pub host: String,
}

impl Database for Postgres {
    fn name(&self) -> &str {
        &self.name
    }

    fn image(&self) -> &str {
        "postgres:15"
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn env_vars(&self) -> Vec<(String, String)> {
        vec![
            ("POSTGRES_USER".into(), self.user.clone()),
            ("POSTGRES_PASSWORD".into(), self.password.clone()),
            ("POSTGRES_DB".into(), self.db_name.clone()),
        ]
    }

    fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }

    fn default() -> Self {
        Postgres {
            name: "pg-default".to_string(),
            user: "admin".to_string(),
            password: "secret".to_string(),

            port: 5432,
            db_name: "mypg".to_string(),
            host: "localhost".to_string(),
        }
    }
}
