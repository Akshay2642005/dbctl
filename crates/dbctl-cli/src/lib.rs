use clap::{Parser, Subcommand};
use dbctl_core::db::{Database, MariaDB, Postgres, Redis};
use dbctl_core::docker::DockerEngine;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new database container
    Create {
        #[arg(value_enum)]
        db_type: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        user: Option<String>,
        #[arg(short = 'P', long)]
        password: Option<String>,
        #[arg(short = 'p', long)]
        port: Option<u16>,
        #[arg(short, long)]
        db_name: Option<String>,
        #[arg(short, long)]
        from_file: Option<PathBuf>,
    },
    /// List running database containers
    List {},
    /// View logs for a container
    Logs { container_id: String },
    /// Remove a database container
    Remove { container_id: String },
}

async fn create_postgres(
    name: Option<String>,
    user: Option<String>,
    password: Option<String>,
    port: Option<u16>,
    db_name: Option<String>,
) -> anyhow::Result<()> {
    let mut pg = Postgres::default();
    if let Some(name) = name {
        pg.name = name;
    }
    if let Some(user) = user {
        pg.user = user;
    }
    if let Some(password) = password {
        pg.password = password;
    }
    if let Some(port) = port {
        pg.port = port;
    }
    if let Some(db_name) = db_name {
        pg.db_name = db_name;
    }

    let engine = DockerEngine::new().await;
    let container_id = engine.start_container(pg.clone()).await?;

    println!("✅ Database '{}' started in Docker", pg.name);
    println!("🔗 URL: {}", pg.connection_url());
    println!("🆔 Container ID: {}", &container_id[..12]);

    Ok(())
}

async fn create_redis(
    name: Option<String>,
    password: Option<String>,
    port: Option<u16>,
) -> anyhow::Result<()> {
    let mut redis = Redis::default();
    if let Some(name) = name {
        redis.name = name;
    }
    if let Some(password) = password {
        redis.password = Some(password);
    }
    if let Some(port) = port {
        redis.port = port;
    }

    let engine = DockerEngine::new().await;
    let container_id = engine.start_container(redis.clone()).await?;

    println!("✅ Redis '{}' started in Docker", redis.name);
    println!("🔗 URL: {}", redis.connection_url());
    println!("🆔 Container ID: {}", &container_id[..12]);

    Ok(())
}

async fn create_mariadb(
    name: Option<String>,
    user: Option<String>,
    password: Option<String>,
    port: Option<u16>,
    db_name: Option<String>,
) -> anyhow::Result<()> {
    let mut mariadb = MariaDB::default();
    if let Some(name) = name {
        mariadb.name = name;
    }
    if let Some(user) = user {
        mariadb.user = user;
    }
    if let Some(password) = password {
        mariadb.password = password;
    }
    if let Some(port) = port {
        mariadb.port = port;
    }
    if let Some(db_name) = db_name {
        mariadb.db_name = db_name;
    }

    let engine = DockerEngine::new().await;
    let container_id = engine.start_container(mariadb.clone()).await?;

    println!("✅ MariaDB '{}' started in Docker", mariadb.name);
    println!("🔗 URL: {}", mariadb.connection_url());
    println!("🆔 Container ID: {}", &container_id[..12]);

    Ok(())
}

async fn list_containers() -> anyhow::Result<()> {
    let engine = DockerEngine::new().await;
    let containers = engine.docker.list_containers::<String>(None).await?;

    println!("🐳 Running Database Containers:");
    println!(
        "{:<15} {:<20} {:<15} {:<10}",
        "CONTAINER ID", "NAME", "IMAGE", "STATUS"
    );

    for container in containers {
        if let (Some(id), Some(names), Some(image), Some(status)) = (
            container.id,
            container.names,
            container.image,
            container.status,
        ) {
            if names.iter().any(|name| name.contains("dbctl")) {
                let name = names.first().unwrap_or(&String::new()).replace("/", "");
                println!(
                    "{:<15} {:<20} {:<15} {:<10}",
                    &id[..12],
                    name,
                    image,
                    status
                );
            }
        }
    }

    Ok(())
}

async fn view_logs(container_id: &str) -> anyhow::Result<()> {
    let engine = DockerEngine::new().await;
    let logs = engine.container_logs(container_id).await?;

    println!("📋 Logs for container {}:", container_id);
    for line in logs {
        println!("{}", line);
    }

    Ok(())
}

async fn remove_container(container_id: &str) -> anyhow::Result<()> {
    let engine = DockerEngine::new().await;
    engine.stop_container(container_id).await?;

    println!("✅ Container {} stopped and removed", container_id);

    Ok(())
}

pub async fn run_cli_async() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create {
            db_type,
            name,
            user,
            password,
            port,
            db_name,
            from_file,
        }) => {
            if let Some(_path) = from_file {
                println!("Loading from file not yet implemented");
                return Ok(());
            }

            match db_type.to_lowercase().as_str() {
                "postgres" => {
                    create_postgres(
                        name.clone(),
                        user.clone(),
                        password.clone(),
                        *port,
                        db_name.clone(),
                    )
                    .await?
                }
                "redis" => create_redis(name.clone(), password.clone(), *port).await?,
                "mariadb" => {
                    create_mariadb(
                        name.clone(),
                        user.clone(),
                        password.clone(),
                        *port,
                        db_name.clone(),
                    )
                    .await?
                }
                _ => {
                    println!("Unsupported database type: {}", db_type);
                }
            }
        }
        Some(Commands::List {}) => list_containers().await?,
        Some(Commands::Logs { container_id }) => view_logs(&container_id).await?,
        Some(Commands::Remove { container_id }) => remove_container(&container_id).await?,
        None => {
            println!("No command specified. Use --help for available commands.");
        }
    }

    Ok(())
}
