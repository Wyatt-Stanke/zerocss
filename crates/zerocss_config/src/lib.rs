use config::{Config, Environment, File};

/// Get the configuration for the current environment.
/// 
/// ## Examples
/// ```no_run
/// use zerocss_config::get_config;
/// 
/// let config = get_config();
/// ```
pub fn get_config() -> Config {
    let config = Config::builder()
        .add_source(File::with_name("zero"))
        .add_source(Environment::with_prefix("ZERO"))
        .build()
        .expect("Failed to load configuration");
    
    config
}