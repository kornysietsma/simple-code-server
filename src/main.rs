#![feature(proc_macro_hygiene, decl_macro)]
use rocket::config::{Config, Environment};
use rocket::http::Method;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::AllowedOrigins;
use std::error::Error;
use std::path::PathBuf;
extern crate structopt;

extern crate rocket;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "simple-code-server",
    about = "A very simple server to allow local apps to access source code without CORS restrictions"
)]
struct Opt {
    /// Root directory to serve
    #[structopt(parse(from_os_str))]
    root: PathBuf,

    /// local address
    #[structopt(short = "h", long = "host", default_value = "localhost")]
    address: String,

    /// local port
    #[structopt(short = "p", long = "port", default_value = "8675")]
    port: u16,

    /// allowed origins regex, default allows 127.0.0.1, localhost and 0.0.0.0
    #[structopt(
        short = "o",
        long = "origins",
        default_value = r"^https?://((127\.0\.0\.1)|(localhost)|(0\.0\.0\.0))(:\d+)?"
    )]
    origins: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let allowed_origins = AllowedOrigins::some_regex(&[opt.origins]);

    let config = Config::build(Environment::Production)
        .address(opt.address)
        .port(opt.port)
        .finalize()?;

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;
    rocket::custom(config)
        .mount("/", StaticFiles::from(opt.root))
        .attach(cors)
        .launch();

    Ok(())
}
