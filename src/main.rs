#[cfg(feature = "ssr")]
use alex_hou_2024_test_16::config::AppEnv;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), alex_hou_2024_test_16::config::ConfigError> {
    let config = AppEnv::load()?;

    eprintln!(
        "Environment configuration loaded for '{}' with site root '{}' on {}.",
        config.output_name,
        config.site_root.display(),
        config.site_addr
    );

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
