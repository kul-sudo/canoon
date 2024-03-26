use std::{
    env::args,
    fs::{create_dir, read_to_string, remove_file, write},
    path::Path,
    process::{exit, Command},
};

static CURRENT_COMMIT_DIR: &str = "/var/lib/cano/current_commit.txt";

enum Commands {
    Install,
    Uninstall,
    Update,
}

macro_rules! move_file {
    ($from:expr, $to:expr) => {
        Command::new("cp").arg($from).arg($to).output().unwrap();
    };
}

macro_rules! force_delete {
    ($path:expr) => {
        Command::new("rm").arg("-rf").arg($path).output().unwrap();
    };
}

fn install(latest_commit_hash: &str) {
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/CobbCoding1/Cano")
        .output()
        .unwrap();

    Command::new("make").current_dir("./Cano").output().unwrap();
    move_file!("./Cano/build/cano", "/usr/bin/");
    create_dir("/var/lib/cano/").unwrap();
    write(CURRENT_COMMIT_DIR, latest_commit_hash).unwrap();
    force_delete!("./Cano");
}

fn uninstall() {
    remove_file("/usr/bin/cano").unwrap();
    force_delete!("/var/lib/cano/");
}

fn main() {
    let mut args = args().collect::<Vec<_>>();
    args.remove(0);
    if args.len() != 1 {
        println!("The utility accepts one of these arguments: install, uninstall, update.");
        exit(0)
    }

    // Get the latest commit of the main branch
    let latest_commit_hash = String::from_utf8(
        Command::new("git")
            .arg("ls-remote")
            .arg("https://github.com/CobbCoding1/Cano.git")
            .arg("main")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .split('\t')
    .collect::<Vec<_>>()
    .first()
    .unwrap()
    .to_string();

    let cano_installed = Path::new("/usr/bin/cano").is_file();

    match args.first().unwrap().as_str() {
        "install" => {
            if cano_installed {
                println!(
                    "Cano is already installed. If you're trying to update, use 'canoon update'."
                );
            } else {
                println!("Installing Cano...");
                install(&latest_commit_hash);
                println!("Successfully installed.");
            }
        }
        "uninstall" => {
            if cano_installed {
                println!("Uninstalling Cano...");
                uninstall();
                println!("Successfully uninstalled.");
            } else {
                println!("Cano isn't installed.");
            }
        }
        "update" => {
            if cano_installed {
                if let Ok(current_local_commit) = read_to_string(CURRENT_COMMIT_DIR) {
                    if current_local_commit == latest_commit_hash {
                        println!("Cano's up-to-date :O");
                    } else {
                        println!("An update's available! Installing it...");
                        uninstall();
                        install(&latest_commit_hash);
                        println!("Successfully installed.");
                    }
                }
            } else {
                println!("Cano isn't installed.");
            }
        }
        _ => {
            println!("An unavailable command has been used!");
        }
    }
}
