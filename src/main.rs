pub mod tmdb;

use clap::Parser;
use log::{info, trace};
use std::path::PathBuf;

const PEOPLE_DIR: &str = "data/metadata/People";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Parameter {
    /// Jellyfin's config path
    #[arg(value_name="CONFIG PATH", value_parser=check_config_root)]
    config_root: PathBuf,
    /// Root of movie and nfo files.
    #[arg(value_name = "MEDIA PATH", value_parser=check_media_root)]
    media_root: PathBuf,

    /// Include adult content
    #[arg(short)]
    adult: bool,
}
fn check_media_root(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.is_dir() {
        return Err(format!("'{}' is not a directory.", s));
    }
    Ok(path)
}
fn check_config_root(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.join(PEOPLE_DIR).is_dir() {
        return Err(format!(
            "'{}' does not have the subdirectory '{}'.",
            s, PEOPLE_DIR
        ));
    }
    Ok(path)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env)
        .format_timestamp(None)
        //.filter(Some("naca_download"), LevelFilter::Trace)
        .init();

    let params = Parameter::parse();
    println!("parameters: {:?}", params);
    walk_dirs(&params.media_root)?;

    // let client_builder = crate::tmdb::ClientBuilder::new();
    // let configuration_client = client_builder.configuration_client();
    // let configuration = configuration_client.configuration()?;
    // if let Some(images) = configuration.images {
    //     let base_url = images.secure_base_url;
    //     println!("base_url: {:?}", base_url);
    //     if let Some(s) = images.poster_sizes {
    //         println!("{:?}", s);
    //     }
    // }

    // let search_client = client_builder.search_client();
    // let peoples = search_client.search_people(None, "Nancy A", None, Some(true), None)?;
    // if let Some(v) = peoples.results {
    //     for po in v.iter() {
    //         println!(
    //             "adult: {:?}, id: {:?}, name: {:?}, profile: {:?}",
    //             po.adult, po.id, po.name, po.profile_path
    //         );
    //     }
    // }
    Ok(())
}

pub fn walk_dirs(dir: &PathBuf) -> Result<(), std::io::Error> {
    trace!("walk_dirs({:?})", dir);
    if dir.is_dir() {
        for entry in dir.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                walk_dirs(&path)?;
            } else {
                if let Some(ext) = path.extension() {
                    if ext.eq_ignore_ascii_case("nfo") {
                        info!("{:?}", path);
                    }
                }
            }
        }
    }
    Ok(())
}
