#![recursion_limit = "256"]

use cfg_if::cfg_if;

// boilerplate to run in different modes
cfg_if! {
    if #[cfg(feature = "ssr")] {

        // use axum::routing::post;
        // use leptos_components::darkmode::update_theme;

        // use tower_http::services::ServeDir;
        // use axum::{
            // response::{Response, IntoResponse},
            // routing::get,
            // extract::{Request, State},
        // };
        // use axum_extra::extract::cookie::CookieJar;
        // use overview::app::*;

        // use leptos_axum::LeptosRoutes;
        // use leptos::prelude::{LeptosOptions, provide_context};
        // use axum::extract::FromRef;
        // use overview::fallback::file_and_error_handler;



        // #[derive(FromRef, Debug, Clone)]
        // pub struct AppState {
        //     pub leptos_options: LeptosOptions,
        // }

        #[tokio::main]
        async fn main() {
            use axum::Router;
            use leptos::logging::log;
            use leptos::prelude::*;
            use leptos_axum::{LeptosRoutes, generate_route_list};
            use overview::app::*;

            let conf = get_configuration(None).unwrap();
            let addr = conf.leptos_options.site_addr;
            let leptos_options = conf.leptos_options;
            // let site_root = &leptos_options.site_root;
            // let pkg_dir = &leptos_options.site_pkg_dir;

            // The URL path of the generated JS/WASM bundle from cargo-leptos
            // let bundle_path = format!("/{site_root}/{pkg_dir}");
            // The filesystem path of the generated JS/WASM bundle from cargo-leptos
            // let bundle_filepath = format!("./{site_root}/{pkg_dir}");
            // let cargo_leptos_service = ServeDir::new(&bundle_filepath);

            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(App);

            // let app = Router::new()
            //     .nest_service(
            //         &["/", &leptos_options.site_pkg_dir].concat(),
            //         cargo_leptos_service.clone(),
            //     )
            //     // .route("/api/update_theme", post(update_theme))
            //     .leptos_routes_with_handler(routes, get(leptos_routes_handler))
            //     .fallback(file_and_error_handler)
            //     .with_state(leptos_options);
            let app = Router::new()
                .leptos_routes(&leptos_options, routes, {
                    let leptos_options = leptos_options.clone();
                    move || overview::app::shell(leptos_options.clone())
                })
                .fallback(leptos_axum::file_and_error_handler(shell))
                .with_state(leptos_options);

            // run our app with hyper
            // `axum::Server` is a re-export of `hyper::Server`
            log!("listening on http://{}", &addr);
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }

        // #[cfg(feature = "ssr")]
        // #[axum::debug_handler]
        // async fn leptos_routes_handler(
        //     State(leptos_options): State<LeptosOptions>,
        //     cookies: CookieJar,
        //     request: Request
        // ) -> Response {
        //     let handler = leptos_axum::render_app_to_stream_with_context(
        //         move || {
        //             provide_context(cookies.clone());
        //         },
        //         move || {
        //             overview::app::shell(leptos_options.clone())
        //         }
        //     );
        //     handler(request).await.into_response()
        // }
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
