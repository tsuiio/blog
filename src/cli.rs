use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Parser, Serialize, Deserialize)]
#[command(author, version, about ,long_about = None)]
pub struct Cli {
    #[clap(short, long, default_value = "./blog.toml", env("TSUIIO_BLOG_CONFIG"))]
    pub conf_path: PathBuf,
    #[command(flatten)]
    pub config: Config,
}
