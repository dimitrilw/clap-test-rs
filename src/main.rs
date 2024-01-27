// taken from the Cookbook's git_derive example, then pared down for testing
// https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html

use std::ffi::OsString;
use std::process;

use clap::{Parser, Subcommand};

/// A test CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "ctr")]
#[command(about = "A test CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Clones repos
    #[command(arg_required_else_help = true)]
    Clone {
        /// The remote to clone
        remote: String,
    },

    // more commands here

    #[command(external_subcommand)]
    External(Vec<OsString>),
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Clone { remote } => {
            println!("from ctr binary :: Cloning {remote}");
        }
        // more commands here
        Commands::External(args) => {
            let external_executable = format!("ctr-{}", &args[0].to_str().unwrap());
            let ext_exe_args = &args[1..];

            let which_ext_proc = process::Command::new("which")
                .arg(&external_executable)
                .output()
                .expect("failed to execute which");

            // if the external command is not found, exit with an error...
            if !which_ext_proc.status.success() {
                let m = format!(
                    "from ctr binary :: ctr: '{}' is not a ctr command. See 'ctr --help'.",
                    &args[0].to_str().unwrap(),
                );
                eprintln!("{}", m.as_str());
                process::exit(1);
            }
            // ...else, continue

            let m = format!(
                "failed to execute external process '{}' with args '{:?}'",
                &external_executable,
                &ext_exe_args,
            );
            let fail_msg= m.as_str();

            println!(
                "from ctr binary :: Calling out to {:?} with {:?}", 
                &external_executable, 
                &ext_exe_args,
            );

            process::Command::new(external_executable)
                .args(ext_exe_args)
                .status()
                .expect(fail_msg);
        }
    }
}