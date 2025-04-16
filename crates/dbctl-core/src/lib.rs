pub mod config;
pub mod db;
pub mod docker;
pub mod error;
pub mod output;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::db::Database;
    use crate::docker::DockerEngine; // Ensure you're importing the correct modules

    #[derive(Clone)]
    struct MockDatabase {
        name: String,

        image: String,
        port: u16,
    }

    impl Database for MockDatabase {
        fn name(&self) -> &str {
            &self.name
        }

        fn image(&self) -> &str {
            &self.image
        }

        fn port(&self) -> u16 {
            self.port
        }

        fn env_vars(&self) -> Vec<(String, String)> {
            let mut env_vars = Vec::new();
            env_vars.push(("POSTGRES_PASSWORD".to_string(), "secret".to_string()));
            env_vars
        }

        fn connection_url(&self) -> String {
            format!("postgres://user:password@localhost:{}/dbname", self.port())
        }

        fn default() -> Self {
            MockDatabase {
                name: "default-db".to_string(),
                image: "postgres:latest".to_string(),

                port: 5432,
            }
        }
    }

    // Test Docker Engine creation
    #[tokio::test]
    async fn test_docker_engine_creation() {
        let engine = DockerEngine::new().await;
        assert!(engine.docker.ping().await.is_ok()); // Check if Docker is reachable
    }

    // Test starting a mock database container
    #[tokio::test]
    async fn test_start_container() {
        let engine = DockerEngine::new().await;
        let db = MockDatabase::default(); // Use default database setup
        let result = engine.start_container(db).await;

        assert!(result.is_ok()); // Check if container started successfully
    }
}
