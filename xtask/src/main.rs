use clap::CommandFactory;
use cli_xtask::{
    config::{ConfigBuilder, DistConfigBuilder},
    workspace, Result, Xtask,
};

fn main() -> Result<()> {
    let workspace = workspace::current();
    let (dist, package) = DistConfigBuilder::from_root_package(workspace)?;
    let command = ja::Cli::command();
    let target = package
        .target_by_name("ja", "bin")?
        .command(command) // this allows us to use the clap to generate a manpage
        .build()?;
    let package = package.targets(vec![target]).build()?;
    let dist = dist.package(package).build()?;
    <Xtask>::main_with_config(|| ConfigBuilder::new().dist(dist).build())
}
