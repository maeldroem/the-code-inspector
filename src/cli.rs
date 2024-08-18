use std::path::PathBuf;

/// The Code Inspector - Analyze your codebase and query it
/// 
/// Analyze your codebase to obtain more information about usages of different
/// code pieces in order to understand how each code piece interacts with the others.
/// 
/// Query your codebase to spot out code that is buried in folders, unused code,
/// or even code that is being used too much, and from where.
/// 
/// Graph out your codebase to get better insight on logic traveling all over the place
/// or simply to help visualize what goes where.
#[derive(clap::Parser)]
#[command(version, author)]
pub struct Args {
    /// Path to the configuration file
    /// 
    /// The configuration file defines how The Code Inspector will process to analyze
    /// the given project (e.g. progressive analysis cache? ignored folders/files?)
    #[arg(long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Analyze a project
    Analyze(AnalysisArgs),

    /// Query an analyzed project
    Query(QueryArgs),

    /// Generate an analyzed project
    Graph(GraphArgs),
}

#[derive(clap::Args)]
pub struct AnalysisArgs {
    /// Location of the project's folder
    path: PathBuf,
}

#[derive(clap::Args)]
pub struct QueryArgs {

}

#[derive(clap::Args)]
pub struct GraphArgs {

}