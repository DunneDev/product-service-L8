use env_logger::Env;
use product_service::{configuration::Settings, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("I updated the code");
    let settings = Settings::new().set_wasm_rules_engine(false);
    env_logger::init_from_env(Env::default().default_filter_or(&settings.log_level));
    run(settings)?.await
}
