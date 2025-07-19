pub mod config;
pub mod db;
pub mod docker;
pub mod error;
pub mod output;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::db::{Database, Postgres};
    use crate::docker::DockerEngine;

    /*
     * @Test the Postgres struct
     * @checking the default values
     * @checking the env vars
     * @checking the connection url
     */

    #[test]
    fn test_postgres_defaults() {
        let pg = Postgres::default();
        assert_eq!(pg.name(), "pg-default");
        assert_eq!(pg.image(), "postgres:15");
        assert_eq!(pg.port(), 5432);
        assert_eq!(
            pg.connection_url(),
            "postgres://admin:secret@localhost:5432/mypg"
        );
    }

    #[test]
    fn test_postgres_env_vars() {
        let pg = Postgres::default();

        let env_vars = pg.env_vars();

        assert!(env_vars.contains(&("POSTGRES_USER".into(), "admin".into())));
        assert!(env_vars.contains(&("POSTGRES_PASSWORD".into(), "secret".into())));
        assert!(env_vars.contains(&("POSTGRES_DB".into(), "mypg".into())));
    }

    /*
     * @Docker Engine Test(Unit Test)
     * @Checking if the docker engine is running
     * @Creating a moke postgres database
     */

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

    // @Test Docker Engine creation
    #[tokio::test]
    async fn test_docker_engine_creation() {
        let engine = DockerEngine::new().await;
        assert!(engine.docker.ping().await.is_ok()); // Check if Docker is reachable
    }

    // @Test starting a mock database container
    #[tokio::test]
    async fn test_start_container() {
        let engine = DockerEngine::new().await;
        let db = MockDatabase::default(); // Use default database setup
        let result = engine.start_container(db).await;

        assert!(result.is_ok());
    }
}
