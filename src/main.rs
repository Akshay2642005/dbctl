use tokio::runtime::Runtime;

fn main() -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(dbctl_cli::run_cli_async())
}
