mod cli;

use anyhow::Result;
use clap::Parser;
use regex::Regex;

use std::{
    io::BufWriter,
    fs::File,
    process::{Command, Stdio},
};

use std::env;
use std::io;
use std::fs;
use std::path::{PathBuf, Path};

use crate::cli::Opts;

use toml::Table;

use path_clean::PathClean;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct WallpaperEntry {
    name: String,
    ext: String,
    sha256: String,
    url: String,
}

struct WallpaperOpts {
    name: String,
    id : String,
}

#[derive(Serialize, Deserialize)]
struct WallhavenData {
    path: String,
    file_type: String,
}

#[derive(Serialize, Deserialize)]
struct WallhavenResponse {
    data : WallhavenData
}

fn get_wallpaper_entry(opts: &WallpaperOpts) -> Result<WallpaperEntry> {
    let url = format!("https://wallhaven.cc/api/v1/w/{}", opts.id);

    let body: String = ureq::get(&url)
        .call()?
        .into_string()?;
    let v: WallhavenResponse = serde_json::from_str(&body)?;

    let ext_regex = Regex::new(r"\.([0-9a-z]+)$")?;
    let ext_captures = ext_regex.captures_iter(&v.data.path);

    let mut ext : String = String::new();
    for cap in ext_captures {
        ext = cap[1].to_string();
        break;
    }

    let hash = nix_prefetch_url(&v.data.path, &format!("{}.{}", &opts.name, &ext));
    
    Ok(WallpaperEntry {
        name : opts.name.clone(),
        sha256 : hash?,
        url : v.data.path,
        ext: ext.to_string(),
    })
}

pub fn nix_prefetch_url(url: &String, name: &String) -> Result<String> {
    let cmdout_bytes = Command::new("nix-prefetch-url")
        .arg("--type")
        .arg("sha256")
        .arg("--name")
        .arg(name)
        .arg(url)
        .stderr(Stdio::inherit())
        .output()?
        .stdout;

    let mut str = String::from_utf8(cmdout_bytes)?;
    str.truncate(str.len() - 1);
    Ok(str)
}

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }.clean();

    Ok(absolute_path)
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(f) = opts.file {    
        let file_content = fs::read_to_string(f.clone())?;
        let toml_content = file_content.parse::<Table>()?;
        let toml_itr = toml_content.iter();
        let mut wallpapers: Vec<WallpaperEntry> = Vec::new();

        for (name, id) in toml_itr {
            if let Some(id) = id.as_str() {
                let wp_opts = WallpaperOpts {
                    name: name.to_string(),
                    id: id.to_string(),
                };
                let entry = get_wallpaper_entry(&wp_opts)?;
                wallpapers.push(entry);
            }
        }
        let file_basename = f.file_stem().unwrap().to_string_lossy();
        let json_file_path = absolute_path(&f)?.parent().unwrap().join(format!("{}.{}", file_basename, "json"));
        let json_file = File::create(json_file_path)?;
        let writer = BufWriter::new(json_file);   

        serde_json::to_writer_pretty(writer, &wallpapers)?;
    } else {
        let wp_opts = WallpaperOpts {
            name: opts.name,
            id: opts.id,
        };
        let entry = get_wallpaper_entry(&wp_opts)?;
        println!("{:?}", entry);
    }

    Ok(())
}
