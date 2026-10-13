//! Architecture-aware self-updater engine with Windows in-place rename swap.
#![allow(dead_code)]

use serde::Deserialize;
use std::env;
use std::fs::{self, File};
use std::io::copy;
use std::path::PathBuf;
use std::process::Command;

const GITHUB_REPO: &str = "Crlyzd/Inventory_Kamera";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubRelease {
    pub tag_name: String,
    pub name: Option<String>,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone)]
pub enum UpdateStatus {
    UpToDate,
    Available { version: String, download_url: String },
    Error(String),
}

/// Checks GitHub Releases for a newer version matching the CPU architecture.
pub fn check_for_updates() -> UpdateStatus {
    let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);
    let client = match reqwest::blocking::Client::builder()
        .user_agent("InventoryKamera-Updater/2.0")
        .build()
    {
        Ok(c) => c,
        Err(e) => return UpdateStatus::Error(format!("Failed to build HTTP client: {e}")),
    };

    let release: GithubRelease = match client.get(&url).send().and_then(|r| r.json()) {
        Ok(rel) => rel,
        Err(e) => return UpdateStatus::Error(format!("Release check failed: {e}")),
    };

    let remote_ver = release.tag_name.trim_start_matches('v');
    if remote_ver == CURRENT_VERSION {
        return UpdateStatus::UpToDate;
    }

    match find_matching_asset(&release.assets) {
        Some(asset) => UpdateStatus::Available {
            version: remote_ver.to_string(),
            download_url: asset.browser_download_url.clone(),
        },
        None => UpdateStatus::Error(format!("No matching binary found for architecture: {}", env::consts::ARCH)),
    }
}

/// Selects the release asset matching the current runtime CPU architecture.
pub fn find_matching_asset<'a>(assets: &'a [ReleaseAsset]) -> Option<&'a ReleaseAsset> {
    let arch = env::consts::ARCH;

    // Filter only .exe assets
    let exe_assets: Vec<&ReleaseAsset> = assets
        .iter()
        .filter(|a| a.name.ends_with(".exe"))
        .collect();

    // Specific architecture matching
    match arch {
        "x86_64" => exe_assets
            .iter()
            .find(|a| {
                let n = a.name.to_lowercase();
                n.contains("x86_64") || n.contains("x64") || n.contains("amd64")
            })
            .copied()
            .or_else(|| exe_assets.first().copied()),
        "aarch64" => exe_assets
            .iter()
            .find(|a| {
                let n = a.name.to_lowercase();
                n.contains("aarch64") || n.contains("arm64") || n.contains("arm")
            })
            .copied(),
        "x86" => exe_assets
            .iter()
            .find(|a| {
                let n = a.name.to_lowercase();
                (n.contains("x86") || n.contains("i686") || n.contains("win32")) && !n.contains("x86_64")
            })
            .copied(),
        _ => exe_assets.first().copied(),
    }
}

/// Executes in-place atomic replacement of the running executable.
pub fn apply_update(download_url: &str) -> Result<(), String> {
    let current_exe = env::current_exe().map_err(|e| format!("Cannot locate current exe: {e}"))?;
    let tmp_exe = current_exe.with_extension("tmp.exe");
    let old_exe = current_exe.with_extension("old");

    download_binary(download_url, &tmp_exe)?;
    swap_and_relaunch(&current_exe, &tmp_exe, &old_exe)
}

fn download_binary(url: &str, dest: &PathBuf) -> Result<(), String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("InventoryKamera-Updater/2.0")
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let mut response = client.get(url).send().map_err(|e| format!("Download failed: {e}"))?;
    let mut file = File::create(dest).map_err(|e| format!("Failed to create temp file: {e}"))?;
    copy(&mut response, &mut file).map_err(|e| format!("Failed to write downloaded binary: {e}"))?;
    Ok(())
}

fn swap_and_relaunch(current: &PathBuf, tmp: &PathBuf, old: &PathBuf) -> Result<(), String> {
    if old.exists() {
        let _ = fs::remove_file(old);
    }

    fs::rename(current, old).map_err(|e| format!("Failed to rename running exe to .old: {e}"))?;
    if let Err(e) = fs::rename(tmp, current) {
        let _ = fs::rename(old, current); // Revert on failure
        return Err(format!("Failed to promote new exe: {e}"));
    }

    Command::new(current)
        .args(env::args().skip(1))
        .spawn()
        .map_err(|e| format!("Failed to launch updated binary: {e}"))?;

    std::process::exit(0);
}
