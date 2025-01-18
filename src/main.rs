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
        #[clap(flatten)]
        action_args: ActionArgs
    },
    /// Deploys helm chart by its deployment file
    Deploy {
        #[clap(flatten)]
        action_args: ActionArgs
    },
    /// Cleans up any releases that are deployed but not defined
    Cleanup {
        namespace: Option<String>,
    },
}

#[derive(Debug, Args)]
struct ActionArgs {
    /// Namespace to deploy in. [Default: Current default namespace]
    #[arg(long, short = 'n')]
    namespace: Option<String>,

    /// Profile to deploy (e.g. dev/prod etc.)
    #[arg(long, short = 'p')]
    profile: Option<String>,
    
    /// Directory of deployment file
    dir: PathBuf,
}


fn main() {
    let app = App::parse();

    println!("{:?}", app);
}
