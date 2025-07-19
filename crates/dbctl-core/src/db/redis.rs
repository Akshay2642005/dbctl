use super::traits::Database;

#[derive(Clone)]
pub struct Redis {
    pub name: String,
    pub password: Option<String>,
    pub port: u16,
    pub host: String,
}

impl Database for Redis {
    fn name(&self) -> &str {
        &self.name
    }

    fn image(&self) -> &str {
        "redis:7"
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn env_vars(&self) -> Vec<(String, String)> {
        let mut env_vars = Vec::new();
        
        if let Some(password) = &self.password {
            env_vars.push(("REDIS_PASSWORD".into(), password.clone()));
        }
        
        env_vars
    }

    fn connection_url(&self) -> String {
        match &self.password {
            Some(password) => format!(
                "redis://default:{}@{}:{}",
                password, self.host, self.port
            ),
            None => format!(
                "redis://{}:{}",
                self.host, self.port
            ),
        }
    }

    fn default() -> Self {
        Redis {
            name: "redis-default".to_string(),
            password: Some("redispassword".to_string()),
            port: 6379,
            host: "localhost".to_string(),
        }
    }
}