use std::{error::Error, path::PathBuf, process};

use clap::{ArgAction, Args, Parser, Subcommand};
use tracing::error;

#[derive(Debug, Parser)]
#[clap(version)]
/// A simple binary to manage your helmcharts.
struct App {
    // Yes this could be a SetTrue, but I liked the solution, but the logging isnt complex enough
    // to make it multi-level
    /// Enables verbose logging.
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
    /// Initialized deployment configuration in current directory
    Init {
        /// Name of resource to be deployed
        name: String,
    },
    /// Configures the cluster
    Cluster {
        #[command(subcommand)]
        cluster_command: ClusterCommand,
    },
    /// Verifies that the chart can be deployed
    Check {
        /// Profile to deploy (e.g. dev/prod etc.)
        #[arg(long, short = 'p')]
        profile: Option<String>,

        #[command(flatten)]
        args: ActionArgs,
    },
    /// Deploys helm chart by its deployment file
    Deploy {
        /// Profile to deploy (e.g. dev/prod etc.)
        #[arg(long, short = 'p')]
        profile: Option<String>,

        #[command(flatten)]
        args: ActionArgs,
    },
    Undeploy {
        #[command(flatten)]
        args: ActionArgs,
    },
    /// Cleans up any releases that are deployed but not defined.
    ///
    /// For this it uses the given namespace and the context directory.
    /// It reads the folder names of the context directories
    /// and compares them with the release-names in the given namespace.
    Cleanup {
        #[command(flatten)]
        args: ActionArgs,

        /// Cleanup all namespaces
        #[arg(long, short = 'A', action = ArgAction::SetTrue)]
        all_namespaces: bool,

        /// Namespace to cleanup.
        #[arg(long, short = 'n')]
        namespace: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum ClusterCommand {
    /// List available clusters
    List,
    /// Set the currently active cluster
    Set { name: String },
}

#[derive(Debug, Args)]
struct ActionArgs {
    /// Will not verify for deployments. Good for CI/CDs
    #[arg(long, short = 'y', action = ArgAction::SetTrue)]
    no_verify: bool,

    /// Context directory. Should be the location of the deployment.yaml for deployments
    /// and the namespace directory for cleanups
    dir: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::parse();

    setup_logger(&app);

    command_available("kubectl")?;
    command_available("helm")?;

    match app.command {
        Command::Init { name } => shippr::actions::initialize_configuration(name)?,

        Command::Check { profile, args } => shippr::actions::check(profile, args.dir)?,

        Command::Cleanup {
            namespace,
            all_namespaces,
            args,
        } => {
            if !all_namespaces && namespace.is_some() {
                shippr::actions::cleanup_namespace(namespace.unwrap(), args.dir, args.no_verify)?
            } else if all_namespaces {
                shippr::actions::cleanup_all_namespaces(args.dir, args.no_verify)?
            } else {
                return Err(shippr::Error::NoNamespacePassed.into());
            }
        }

        Command::Cluster { cluster_command } => match cluster_command {
            ClusterCommand::List => shippr::actions::list_clusters()?,
            ClusterCommand::Set { name } => shippr::actions::set_cluster(&name)?,
        },

        Command::Deploy { profile, args } => {
            shippr::actions::deploy(profile, args.dir, args.no_verify)?
        }

        Command::Undeploy { args } => shippr::actions::undeploy(args.dir, args.no_verify)?,
    }

    Ok(())
}

fn setup_logger(app: &App) {
    let log_level = if app.verbose == 0 {
        tracing::Level::ERROR
    } else if app.verbose == 1 {
        tracing::Level::INFO
    } else {
        tracing::Level::DEBUG
    };
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(log_level)
        .with_file(false)
        .without_time()
        .init();
}

fn command_available(command: &str) -> Result<(), Box<dyn Error>> {
    let output = process::Command::new(command)
        .arg("version")
        .output()
        .map_err(|_| shippr::Error::MissingTool(command.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        error!("{output:?}");
        Err(shippr::Error::FaultyTool(command.to_string()).into())
    }
}
