#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use site::api;
    use site::api::SSRState;
    use site::app::{App, shell};

    let ssr_state = SSRState::new();
    let ssr_state_leptos = ssr_state.clone();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let frontend_routes = generate_route_list(App);
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            frontend_routes,
            move || provide_context(ssr_state_leptos.clone()), // why do we have to clone twice ?
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);
    let app = app.nest("/api", api::routes(ssr_state).await);

    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
