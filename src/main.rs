use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use axum::{
            routing::{post, get },
            Router,
        };
        use scribro::app::*;
        use scribro::fileserv::file_and_error_handler;
        use leptos_axum::{generate_route_list, LeptosRoutes};
        use scribro::components::PostContent::PerformMarkdownCodeToHtml;

        #[tokio::main]
        async fn main() {
            simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

            _ = PerformMarkdownCodeToHtml::register();

            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

            let app = Router::new()
                .route("/api/*fn_name", get(leptos_axum::handle_server_fns))
                .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
                .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
                .fallback(file_and_error_handler)
                .with_state(leptos_options);

            log!("listening on http://{}", &addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }
}
