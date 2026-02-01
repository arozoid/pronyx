use clap::{Arg, Command, ArgAction}; // App is dead, long live Command

use crate::errors::*;
use crate::filesystem::FileSystem;
use crate::utils::Config;

pub const DEFAULT_ROOTFS: &'static str = "/";
pub const DEFAULT_CWD: &'static str = "/";

pub fn get_args_parser() -> Command {
    Command::new("pronyx")
        .about("chroot, mount --bind, and binfmt_misc without privilege/setup.")
        .version(env!("CARGO_PKG_VERSION"))
        .trailing_var_arg(true)
        .arg(Arg::new("rootfs")
            .short('r') // now takes a char, not a str
            .long("rootfs")
            .help("Use *path* as the new guest root file-system.")
            .num_args(1)
            .default_value(DEFAULT_ROOTFS)
            // .value_parser(path_validator) // need to update validator signature
        )
        .arg(Arg::new("bind")
            .short('b')
            .long("bind")
            .help("Make host_path accessible in guest. Format: host_path:guest_path")
            .action(ArgAction::Append) // replaces .multiple(true)
            .num_args(1)
            // .value_parser(binding_validator)
        )
        .arg(Arg::new("cwd")
            .short('w')
            .long("cwd")
            .help("Set the initial working directory.")
            .num_args(1)
            .default_value(DEFAULT_CWD))
        .arg(Arg::new("root_id")
            .short('0')
            .help("Pretend to be root (uid 0, gid 0)")
            .num_args(0)
            .action(ArgAction::SetTrue),
            )
        .arg(Arg::new("link2symlink")
            .long("link2symlink")
            .help("Convert hard links to symbolic links")
            .num_args(0)
            .action(ArgAction::SetTrue),
            )
        .arg(Arg::new("command")
            .required(true)
            .num_args(1..)
            .help("The command to run within Pronyx")
        )
}

pub fn parse_config() -> Result<(FileSystem, Vec<String>, Config)> {
    let app = get_args_parser();
    let mut fs: FileSystem = FileSystem::new();

    let matches = app.get_matches();

    let config = Config {
        root_id: matches.get_flag("root_id"),
        link2symlink: matches.get_flag("link2symlink"),
        // ...
    };

    debug!("pronyx startup with args:\n{:#?}", matches);

    // option -r: use get_one::<String>
    // if let Some(root_id) = matches.get_one::<bool>("root_id") {
    //     fs.set_root(root_id)?;
    // }

    // option -r: use get_one::<String>
    if let Some(rootfs) = matches.get_one::<String>("rootfs") {
        fs.set_root(rootfs)?;
    }

    // option(s) -b: use get_many::<String>
    if let Some(bindings) = matches.get_many::<String>("bind") {
        for raw_binding_str in bindings {
            let parts: Vec<&str> = raw_binding_str.split_terminator(':').collect();
            // make sure we actually have two parts before indexing [1]
            if parts.len() == 2 {
                fs.add_binding(parts[0], parts[1])?;
            } else {
                // maybe default to root if only one part is provided? 
                // proot usually allows "host_path" to mean "host_path:host_path"
                fs.add_binding(parts[0], parts[0])?;
            }
        }
    }

    // option -w
    if let Some(cwd) = matches.get_one::<String>("cwd") {
        fs.set_cwd(cwd)?;
    }

    // command: collect into Vec<String>
    let command: Vec<String> = matches
        .get_many::<String>("command")
        .map(|vals| vals.map(|s| s.clone()).collect())
        .unwrap_or_else(|| vec!["/bin/sh".to_string()]);

    Ok((fs, command, config))
}
