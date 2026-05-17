use clap::{Parser, Subcommand, Args, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "static-track")]
#[command(about = "A project tracking tool for game studios", long_about = None)]
#[command(author)]
struct Cli {
    /// Current project (optional)
    #[arg(short, long)]
    project: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Shows project details
    Proj {
        /// Shows all project details
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        all: bool,
    },

    /// Initialize a new project or show setup info
    Init {
        /// Name for the new project
        #[arg(short, long)]
        name: String, // required

        /// Path to project directory (default: current directory)
        #[arg(short, long, value_name = "PATH")]
        path: Option<PathBuf>,

        /// Skip interactive setup
        #[arg(long, action = clap::ArgAction::SetTrue)] // don't use short here as conflict with `name`. But this is a good idea
        non_interactive: bool,
    },

    /// Shows project lead(s)
    Lead,

    /// Make exportable files inside a project
    Make {
        /// Type of file to create
        #[command(subcommand)]
        file_type: MakeType,

        /// Output filename (default: generated based on type)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Status operations
    Statdo {
        /// Status to check/set
        #[command(subcommand)]
        status: Status,
    },

    /// Verifies project and command files for unauthorized changes
    Verify,

    /// List all projects currently signed up
    List {
        /// List all members currently signed up in a project
        #[command(subcommand)]
        members: ListMembers,
    },
}

#[derive(Subcommand)]
enum MakeType {
    /// Make a Markdown file
    Md,
    /// Make a PDF file
    Pdf,
}

#[derive(Subcommand, Debug)]
enum Status {
    /// Production status
    Prod,
    /// Prototype status
    Proto,
    /// Pre-production status
    Preprod,
    /// Testing status
    Testing,
    /// Assembly status
    Assembly,
}

#[derive(Subcommand)]
enum ListMembers {
    /// List all members currently signed up in a project
    /// Requires the project name to be specified via --project or -p
    All,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Proj { all } => {
            if *all {
                println!("Showing all project details...");
                // Implementation would fetch and display all project info
            } else {
                println!("Showing current project details...");
                // Implementation would fetch and display current project info
            }
        },

        Commands::Init { name, path, non_interactive } => if
        *non_interactive {
            println!("Non-interactive initialization requested");
            // Implementation would perform automated initialization
        } else if let Some(proj_name) = name {
            println!("Initializing project '{}' at specified path", proj_name);
            // Implementation would create project and append user as director
        } else {
            println!("Starting interactive project setup...");
            //TODO
        },

        Commands::Lead => {
            println!("Current project lead(s):");
            // Implementation would display project leads
        },

        Commands::Make { file_type, output } => {
            println!("Creating {} file...", match file_type {
                MakeType::Md => "Markdown",
                MakeType::Pdf => "PDF",
            });
            if let Some(out) = output {
                println!("Output filename: {}", out);
            }
            // Implementation would create the appropriate file
        },

        Commands::Statdo { status } => {
            println!("Checking status: {:?}", status);
            // Implementation would check/set the specified status
        },

        Commands::Verify => {
            println!("Verifying project and command files...");
            // Implementation would check for unauthorized changes
        },
        // this one is all scrwed up and junk
        Commands::List { members } => {
            match members {
                ListMembers::All => {
                    if let Some(proj_name) = &cli.project {
                        println!("Listing all members for project '{}'", proj_name);
                        // Implementation would list project members
                    } else {
                        println!("Error: Please specify a project using --project or -p");
                    }
                },
            }
        },
    }
}