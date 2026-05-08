#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    eprintln!("SSR server bootstrap is implemented in issue #3.");
}

#[cfg(not(feature = "ssr"))]
fn main() {}
