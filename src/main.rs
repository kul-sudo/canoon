use std::{
    env::args,
    fs::{create_dir, create_dir_all, read_to_string, remove_file, write},
    path::Path,
    process::{exit, Command},
};

use dialoguer::Select;
use home::home_dir;
use is_root::is_root;

static CURRENT_COMMIT_DIR: &str = "/var/lib/cano/current_commit.txt";

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

fn install(latest_commit_hash: &str, cano_cache_dir: &str) {
    create_dir_all(cano_cache_dir).unwrap();
    let cano_cloned_dir = &format!("{}/Cano", cano_cache_dir);
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/CobbCoding1/Cano")
        .arg(cano_cloned_dir)
        .output()
        .unwrap();

    Command::new("make")
        .current_dir(cano_cloned_dir)
        .output()
        .unwrap();
    move_file!(format!("{}/build/cano", cano_cloned_dir), "/usr/bin/");
    create_dir("/var/lib/cano/").unwrap();
    write(CURRENT_COMMIT_DIR, latest_commit_hash).unwrap();
    force_delete!(cano_cache_dir);
}

fn uninstall() {
    remove_file("/usr/bin/cano").unwrap();
    force_delete!("/var/lib/cano");
}

fn update(cano_installed: bool, latest_commit_hash: &str, cano_cache_dir: &str) {
    if cano_installed {
        if let Ok(current_local_commit) = read_to_string(CURRENT_COMMIT_DIR) {
            if current_local_commit == latest_commit_hash {
                println!("Cano's up-to-date :O");
            } else {
                println!("An update's available! Installing it...");
                uninstall();
                install(latest_commit_hash, cano_cache_dir);
                println!("Successfully installed.");
            }
        }
    } else {
        println!("Cano isn't installed.");
    }
}

fn main() {
    if !is_root() {
        println!("The installer must be run as root.");
        exit(0);
    }

    if Command::new("git").output().is_err() {
        println!("You must have git installed.");
        exit(0);
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

    let mut args = args().collect::<Vec<_>>();
    args.remove(0);

    let options = ["install", "uninstall", "update"];

    let cano_cache_dir = format!("{}/.cache/canoon", home_dir().unwrap().to_string_lossy());

    match options[Select::new()
        .with_prompt("What do you choose?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap()]
    {
        "install" => {
            if cano_installed {
                println!("Cano is already installed.");
            } else {
                println!("Installing Cano...");
                install(&latest_commit_hash, &cano_cache_dir);
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
            update(cano_installed, &latest_commit_hash, &cano_cache_dir);
        }
        _ => unreachable!(),
    }
}
