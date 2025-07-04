use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(env)]
    pub database_url: String,
}
