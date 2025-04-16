pub trait Database {
    fn name(&self) -> &str;
    fn image(&self) -> &str;
    fn port(&self) -> u16;
    fn env_vars(&self) -> Vec<(String, String)>;
    fn connection_url(&self) -> String;

    fn default() -> Self
    where
        Self: Sized;
}
