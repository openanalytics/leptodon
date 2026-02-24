#![recursion_limit = "256"]

use cfg_if::cfg_if;

// boilerplate to run in different modes
cfg_if! {
    if #[cfg(feature = "ssr")] {

        #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
        async fn main() {
            use axum::Router;
            use leptos::logging::log;
            use leptos::prelude::*;
            use leptos_axum::{LeptosRoutes, generate_route_list};
            use tower_http::compression::CompressionLayer;
            use overview::app::*;

            let conf = get_configuration(None).unwrap();
            let addr = conf.leptos_options.site_addr;
            let leptos_options = conf.leptos_options;

            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(App);

            let app = Router::new()
                .leptos_routes(&leptos_options, routes, {
                    let leptos_options = leptos_options.clone();
                    move || overview::app::shell(leptos_options.clone())
                })
                .fallback(leptos_axum::file_and_error_handler(shell))
                .with_state(leptos_options)
                .layer(CompressionLayer::new());

            // run our app with hyper
            // `axum::Server` is a re-export of `hyper::Server`
            log!("listening on http://{}", &addr);
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }

    } else {
        pub fn main() {
            // no client-side main function
            // unless we want this to work with e.g., Trunk for pure client-side testing
            // see lib.rs for hydration function instead

            use leptos::mount::mount_to_body;
            use overview_lib::app::App;

            mount_to_body(App)
        }
    }
}
