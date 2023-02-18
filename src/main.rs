use clap::crate_authors;
use clap::{Parser, Subcommand};
use std::io;
use tmuxane::*;

#[derive(Parser, Debug)]
#[clap(
    author=crate_authors!(),
    version=build::PKG_VERSION,
    long_version=build::CLAP_LONG_VERSION,
    about="The cross-shell prompt for astronauts. ☄🌌️",
    subcommand_required=true,
    arg_required_else_help=true,
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a pre-populated GitHub issue with information about your configuration
    Init,
}

fn main() {
    println!("Hello, world!");
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(e) => {
            // if the error is not printed to stderr, this means it was not really
            // an error but rather some information is going to be listed, therefore
            // we won't print the arguments passed
            let is_info_only = !e.use_stderr();
            // print the error and void panicking in case of stdout/stderr closing unexpectedly
            let _ = e.print();
            // if there was no mistake by the user and we're only going to display information,
            // we won't put arguments or exit with non-zero code
            let exit_code = if is_info_only {
                0
            } else {
                // print the arguments
                // avoid panicking in case of stderr closing
                let mut stderr = io::stderr();
                use io::Write;
                let _ = writeln!(
                    stderr,
                    "\nNOTE:\n    passed arguments: {:?}",
                    // collect into a vec to format args as a slice
                    std::env::args().skip(1).collect::<Vec<_>>()
                );
                // clap exits with status 2 on error:
                //  https://docs.rs/clap/latest/clap/struct.Error.html#method.exit
                2
            };

            std::process::exit(exit_code);
        }
    };

    match args.command {
        Commands::Init => {
            println!("Init called.")
        }
    }
}
