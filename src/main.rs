use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version)]
/// A simple binary to manage your helmcharts.
struct App {
    #[clap(subcommand)]
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

        #[clap(flatten)]
        args: ActionArgs
    },
    /// Deploys helm chart by its deployment file
    Deploy {
        /// Profile to deploy (e.g. dev/prod etc.)
        #[arg(long, short = 'p')]
        profile: Option<String>,

        #[clap(flatten)]
        args: ActionArgs
    },
    /// Cleans up any releases that are deployed but not defined
    Cleanup {
        #[clap(flatten)]
        args: ActionArgs
    },
}

#[derive(Debug, Args)]
struct ActionArgs {
    /// Namespace to deploy in. [Default: Current default namespace]
    #[arg(long, short = 'n')]
    namespace: Option<String>,
    
    /// Directory of deployment file
    dir: PathBuf,
}


fn main() {
    let app = App::parse();

    match app.command {
        Command::Check { profile, args } => shippr::actions::check(profile, args.namespace, args.dir),
        Command::Cleanup { args } => shippr::actions::cleanup(args.namespace, args.dir),
        Command::Cluster { name } => shippr::actions::set_cluster(name),
        Command::Deploy { profile, args } => shippr::actions::deploy(profile, args.namespace, args.dir),
    }
}
