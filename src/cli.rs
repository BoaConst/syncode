use clap::{Command, Arg};

pub fn build_cli() -> Command<'static> {
    Command::new("dvcs")
        .version("1.0")
        .about("Distributed Version Control System")
        .author("Syncode")
        .subcommand(
            Command::new("init")
                .about("Initialize a new repository")
                .arg(Arg::new("directory")
                     .help("The directory to initialize the repository in")
                     .required(false)),
        )
        .subcommand(
            Command::new("add")
                .about("Add files to tracking")
                .arg(Arg::new("files")
                     .help("Files to add")
                     .required(true)
                     .multiple_values(true)),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove file from tracking")
                .arg(Arg::new("file")
                     .help("File to remove")
                     .required(true)),
        )
        .subcommand(
            Command::new("commit")
                .about("Commit changes")
                .arg(Arg::new("message")
                     .help("Commit message")
                     .required(true)),
        )
        .subcommand(
            Command::new("status")
                    .about("Show status of tracked and untracked files"))
        .subcommand(
            Command::new("log")
                .about("Show commit history"))
}
