// src/main.rs
use std::process::Command;
use self_update::{backends::github::Release, backends::Download};

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Check for updates
    if let Ok(release) = check_for_update() {
        println!(
            "A new version is available! Current version: {}, Latest version: {}",
            CURRENT_VERSION, release.version
        );
        println!("Do you want to update? (y/n)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().eq_ignore_ascii_case("y") {
            // Perform the update process
            if let Err(err) = update_application(&release) {
                eprintln!("Failed to update: {:?}", err);
            }
        }
    }

    // The rest of your application logic goes here
}

fn check_for_update() -> Result<Release, Box<dyn std::error::Error>> {
    let target = "your_target";  // Replace with your target platform
    let repo_owner = "authtbh";
    let repo_name = "mew";

    let backend = self_update::backends::github::Updater::configure()
        .repo_owner(repo_owner)
        .repo_name(repo_name)
        .target(target)
        .bin_name("mew")
        .current_version(CURRENT_VERSION)
        .build()?;

    let release = backend.fetch_update()?;
    Ok(release)
}

fn update_application(release: &Release) -> Result<(), Box<dyn std::error::Error>> {
    let target = "your_target";  // Replace with your target platform

    // Download the updated binary
    let download = self_update::backends::Download::from_release(release, target)?;
    let new_version = download.version();

    // Replace the current binary with the updated one
    download.run()?;

    println!("Successfully updated to version {}", new_version);
    Ok(())
}
