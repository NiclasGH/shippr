use std::{error::Error, path::PathBuf};

use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version)]
/// A simple binary to manage your helmcharts.
struct App {
    /// Sets verbosity level. Can be used multiple times for more detail [e.g. -vvvv]. 
    /// [Default: ERROR logs]
    #[arg(
        global = true,
        action = ArgAction::Count,
        long,
        short,
    )]
    verbose: u8,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Configures the cluster
    Cluster {
        /// Name of the cluster/context to use
        name: String
    },
    /// Verifies that the chart can be deployed
    Check {
        /// Profile to deploy (e.g. dev/prod etc.)
        #[arg(long, short = 'p')]
        profile: Option<String>,

        #[command(flatten)]
        args: ActionArgs
    },
    /// Deploys helm chart by its deployment file
    Deploy {
        /// Profile to deploy (e.g. dev/prod etc.)
        #[arg(long, short = 'p')]
        profile: Option<String>,

        #[command(flatten)]
        args: ActionArgs
    },
    /// Cleans up any releases that are deployed but not defined
    Cleanup {
        #[command(flatten)]
        args: ActionArgs,

        /// Namespace to cleanup.
        #[arg(long, short = 'n')]
        namespace: String,
    },
}

#[derive(Debug, Args)]
struct ActionArgs {

    /// Will not verify for deployments. Good for CI/CDs
    #[arg(long, short = 'y', action = ArgAction::SetTrue)]
    no_verify: bool,
    
    /// Directory of deployment file
    dir: PathBuf,
}


fn main() -> Result<(), Box<dyn Error>> {
    let app = App::parse();

    let log_level = match app.verbose {
        0 => tracing::Level::ERROR,
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3.. => tracing::Level::DEBUG,
    };
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(log_level)
        .with_file(false)
        .without_time()
        .init();

    match app.command {
        Command::Check { profile, args } => shippr::actions::check(profile, args.dir)?,
        Command::Cleanup { namespace, args } => shippr::actions::cleanup(namespace, args.dir),
        Command::Cluster { name } => shippr::actions::set_cluster(&name)?,
        Command::Deploy { profile, args } => shippr::actions::deploy(profile, args.dir)?,
    }

    Ok(())
}
