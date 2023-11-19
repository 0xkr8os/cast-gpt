use alloy_primitives::Address;
use anyhow::Result;
use cast_gpt::run;
use cast_gpt::settings::Settings;
use dotenv::dotenv;
use std::env;

/// The main entry point of the application.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let settings = Settings::new()?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide an address and prompt");
    }

    let address = args[1].parse::<Address>().expect("Failed to parse address");
    let prompt = args[2].clone();

    run(address, prompt, settings).await?;

    Ok(())
}
