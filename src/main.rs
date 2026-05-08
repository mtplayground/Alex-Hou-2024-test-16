#[cfg(feature = "ssr")]
use alex_hou_2024_test_16::{
    app::{shell, App},
    config::{AppEnv, ConfigError},
};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), ServerStartError> {
    use axum::Router;
    use leptos::logging::log;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let config = AppEnv::load()?;
    let leptos_options = config.leptos_options();
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}

#[cfg(feature = "ssr")]
#[derive(Debug)]
enum ServerStartError {
    Config(ConfigError),
    Io(std::io::Error),
}

#[cfg(feature = "ssr")]
impl std::fmt::Display for ServerStartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(source) => write!(f, "{source}"),
            Self::Io(source) => write!(f, "{source}"),
        }
    }
}

#[cfg(feature = "ssr")]
impl std::error::Error for ServerStartError {}

#[cfg(feature = "ssr")]
impl From<ConfigError> for ServerStartError {
    fn from(source: ConfigError) -> Self {
        Self::Config(source)
    }
}

#[cfg(feature = "ssr")]
impl From<std::io::Error> for ServerStartError {
    fn from(source: std::io::Error) -> Self {
        Self::Io(source)
    }
}
