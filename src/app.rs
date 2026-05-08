use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let stylesheet = format!("/pkg/{}.css", env!("CARGO_PKG_NAME"));

    view! {
        <Stylesheet id="leptos" href=stylesheet />
        <Title text="Alex-Hou-2024-test-16" />

        <Router>
            <main class="app-shell">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section>
            <h1>"Alex-Hou-2024-test-16"</h1>
            <p>"Minimal Axum + Leptos server bootstrap is running."</p>
        </section>
    }
}
