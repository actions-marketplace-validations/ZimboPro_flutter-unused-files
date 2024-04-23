use std::env;
use std::fs::write;
use std::process::exit;

extern crate glob;

use std::path::PathBuf;

use clap::Parser;
use glob::glob;
use indicatif::ProgressBar;

#[derive(Debug, Parser, Clone)]
struct CLI {
    /// If output should be a warning
    #[arg(short, long)]
    warning: bool,
    /// If assets should be ignored
    #[arg(short, long)]
    assets: bool,
}

fn write_info(github_output_path: String, message: String) {
    println!("INFO: {}", message);
    write(github_output_path, format!("info={message}")).unwrap();
}

fn write_warning(github_output_path: String, message: String) {
    println!("WARNING: {}", message);
    write(github_output_path, format!("warning={message}")).unwrap();
}

fn write_error(github_output_path: String, message: String) {
    eprintln!("Error: {}", message);
    write(github_output_path, format!("error={message}")).unwrap();
}

fn main() {
    let args = CLI::parse();
    let valid = find_unreferenced_asset_files(args);

    if !valid {
        exit(1);
    } else {
        write_info(
            env::var("GITHUB_OUTPUT").unwrap(),
            "No unreferenced files found".to_string(),
        );
    }
}

fn find_unreferenced_asset_files(args: CLI) -> bool {
    let assets: Vec<PathBuf> = if args.assets {
        Vec::new()
    } else {
        let assets = glob("assets/**/*").expect("Failed to read glob pattern");
        assets.flatten().map(|x| x).collect()
    };
    let dart = glob("lib/**/*.dart").expect("Failed to read glob pattern");
    let dart: Vec<PathBuf> = dart.flatten().map(|x| x).collect();
    let mut asset_files = Vec::new();
    for asset in assets.iter() {
        asset_files.push(asset.file_name().unwrap().to_owned());
    }

    let mut referenced_asset_files = std::collections::HashSet::new();
    let mut referenced_dart_files = std::collections::HashSet::new();

    let bar = ProgressBar::new(dart.len() as u64);
    for file in dart.clone() {
        bar.inc(1);
        let contents = std::fs::read_to_string(&file).expect("Failed to read file");
        for asset in asset_files.iter() {
            if contents.contains(asset.to_str().unwrap()) {
                referenced_asset_files.insert(asset);
            }
        }
        for dart_file in dart.clone() {
            let name = dart_file.file_name().unwrap().to_str().unwrap();
            if file != dart_file
                && (contents.contains(name) || contents.contains(name.replace(" ", "%20").as_str()))
            {
                referenced_dart_files.insert(dart_file);
            }
        }
    }
    bar.finish();

    let mut unreferenced_files = Vec::new();
    for asset in assets {
        let file_name = asset.file_name().unwrap();
        if !referenced_asset_files.contains(&file_name.to_os_string()) {
            unreferenced_files.push(asset);
        }
    }

    let mut unreferenced_dart_files = Vec::new();
    for file in dart {
        if !referenced_dart_files.contains(&file) {
            unreferenced_dart_files.push(file);
        }
    }
    let valid = unreferenced_files.is_empty() && unreferenced_dart_files.is_empty();
    for asset in unreferenced_files.iter().enumerate() {
        let message = format!("{}. Unreferenced asset: {:?}", asset.0 + 1, asset.1);
        if args.warning {
            write_warning(env::var("GITHUB_OUTPUT").unwrap(), message);
        } else {
            write_error(env::var("GITHUB_OUTPUT").unwrap(), message)
        }
    }
    println!();
    for file in unreferenced_dart_files.iter().enumerate() {
        let message = format!("{} Unreferenced file: {:?}", file.0 + 1, file.1);
        if args.warning {
            write_warning(env::var("GITHUB_OUTPUT").unwrap(), message);
        } else {
            write_error(env::var("GITHUB_OUTPUT").unwrap(), message)
        }
    }
    valid
}
