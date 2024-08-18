mod config;
mod vortex;

use vortex::convert_to_vortex::convert_to_vortex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_env();
    println!("Using model: {}", config.model_name);

    convert_to_vortex(&config.model_name, &config.model_path, &config.output_dir, config.vortex_version)?;
    
    Ok(())
}
