use bollard::Docker;
use bollard::container::Config as ContainerConfig;
use bollard::container::CreateContainerOptions;
use bollard::container::StartContainerOptions;
use bollard::container::Stats;
use bollard::image::CreateImageOptions;
use bollard::models::*;
use futures_util::stream::TryStreamExt;
use std::collections::HashMap;
use std::default::Default;

use crate::db::Database;

pub struct DockerEngine {
    pub docker: Docker,
}

impl DockerEngine {
    pub async fn new() -> Self {
        let docker = Docker::connect_with_local_defaults().unwrap();
        DockerEngine { docker }
    }

    pub async fn start_container<D: Database>(&self, db: D) -> anyhow::Result<String> {
        self.docker
            .create_image(
                Some(CreateImageOptions {
                    from_image: db.image(),
                    ..Default::default()
                }),
                None,
                None,
            )
            .try_collect::<Vec<_>>()
            .await?;

        let env_vars: Vec<String> = db
            .env_vars()
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        let env: Vec<&str> = env_vars.iter().map(|s| s.as_str()).collect();

        let port_str = format!("{}/tcp", db.port());

        let config = ContainerConfig {
            image: Some(db.image()),
            env: Some(env),
            exposed_ports: Some({
                let mut map = HashMap::new();
                map.insert(port_str.as_str(), HashMap::new());
                map
            }),
            host_config: Some(HostConfig {
                port_bindings: Some({
                    let mut pb = HashMap::new();
                    pb.insert(
                        port_str.clone(),
                        Some(vec![PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
                            host_port: Some(db.port().to_string()),
                        }]),
                    );
                    pb
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        let container_name = format!("dbctl-{}", db.name());

        let created = self
            .docker
            .create_container(
                Some(CreateContainerOptions::<&str> {
                    name: &container_name,

                    platform: None,
                }),
                config,
            )
            .await?;

        self.docker
            .start_container(&created.id, None::<StartContainerOptions<String>>)
            .await?;

        Ok(created.id)
    }

    pub async fn stop_container(&self, container_id: &str) -> anyhow::Result<()> {
        self.docker.stop_container(container_id, None).await?;

        self.docker.remove_container(container_id, None).await?;

        Ok(())
    }

    pub async fn container_logs(&self, container_id: &str) -> anyhow::Result<Vec<String>> {
        let logs = self
            .docker
            .logs(
                container_id,
                Some(bollard::container::LogsOptions {
                    stdout: true,
                    stderr: true,
                    follow: false,
                    tail: "100".to_string(),
                    ..Default::default()
                }),
            )
            .try_collect::<Vec<_>>()
            .await?;

        let log_lines = logs.iter().map(|log| log.to_string()).collect();

        Ok(log_lines)
    }

    pub async fn container_stats(
        &self,
        container_id: &str,
    ) -> anyhow::Result<Stats> {
        let stats = self
            .docker
            .stats(
                container_id,
                Some(bollard::container::StatsOptions {
                    stream: false,
                    one_shot: true,
                }),
            )
            .try_collect::<Vec<_>>()
            .await?;

        if let Some(container_stats) = stats.first() {
            Ok(container_stats.clone())
        } else {
            anyhow::bail!("No stats available for container {}", container_id)
        }
    }

    pub async fn container_info(
        &self,
        container_id: &str,
    ) -> anyhow::Result<ContainerInspectResponse> {
        let info = self.docker.inspect_container(container_id, None).await?;
        Ok(info)
    }
}
