use alloy_primitives::Address;
use settings::Settings;

pub mod settings;

pub fn run(address: Address, prompt: String, settings: Settings){
  println!("address: {:?}", address);
  println!("prompt: {:?}", prompt);
  println!("settings: {:?}", settings);
}
