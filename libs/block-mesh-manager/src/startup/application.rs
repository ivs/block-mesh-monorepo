use crate::configuration::settings::Settings;
use crate::emails::email_client::EmailClient;
use crate::envars::app_env_var::AppEnvVar;
use crate::envars::env_var;
use crate::envars::get_env_var_or_panic::get_env_var_or_panic;
use crate::frontends::app::App;
use crate::middlewares::authentication::{authentication_layer, Backend};
use crate::routes;
use axum::routing::{get, post};
use axum::{Extension, Router};
use axum_login::login_required;
use leptos::leptos_config::get_config_from_env;
use leptos_axum::{generate_route_list, LeptosRoutes};
use reqwest::Client;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub struct Application {
    app: Router,
    listener: TcpListener,
}

pub struct AppState {
    pub pool: PgPool,
    pub email_client: Arc<EmailClient>,
    pub client: Client,
    pub tx: tokio::sync::mpsc::Sender<JoinHandle<()>>,
    pub flags: HashMap<String, bool>,
}

#[derive(Clone)]
pub struct ApplicationBaseUrl(pub String);

impl ApplicationBaseUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Application {
    pub async fn build(settings: Settings, app_state: Arc<AppState>, db_pool: PgPool) -> Self {
        let auth_layer = authentication_layer(&db_pool).await;

        let app_env = get_env_var_or_panic(AppEnvVar::AppEnvironment);
        let app_env = <env_var::EnvVar as AsRef<String>>::as_ref(&app_env);
        let cors = match app_env.as_str() {
            "local" => CorsLayer::very_permissive(),
            _ => CorsLayer::permissive(),
        };

        let auth_router = Router::new()
            .route("/logout", get(routes::logout::get::handler))
            .route("/view_task", get(routes::tasks::view_task::handler))
            .route(
                "/create_task",
                get(routes::tasks::create_task::handler)
                    .post(routes::tasks::create_task_post::handler),
            )
            .route(
                "/edit_invite_code",
                get(routes::invite_codes::edit_invite_code::handler)
                    .post(routes::invite_codes::edit_invite_code_post::handler),
            )
            .route("/tasks_table", get(routes::tasks::tasks_table::handler))
            .route(
                "/dashboard",
                get(routes::dashboard::get::handler).post(routes::dashboard::post::handler),
            );

        let api_router = Router::new()
            .route(
                "/report_uptime",
                post(routes::uptime_report::report_uptime::handler),
            )
            .route(
                "/submit_bandwidth",
                post(routes::bandwidth::submit_bandwidth::handler),
            )
            .route("/get_token", post(routes::api_token::get_token::handler))
            .route("/get_task", post(routes::tasks::get_task::handler))
            .route("/submit_task", post(routes::tasks::submit_task::handler))
            .route("/get_stats", post(routes::api_token::get_stats::handler))
            .route(
                "/get_latest_invite_code",
                post(routes::invite_codes::get_latest_invite_code::handler),
            )
            .route(
                "/create_task_with_token",
                post(routes::tasks::create_task_with_token::handler),
            );

        let un_auth_router = Router::new()
            .route("/map", get(routes::map::world_map::handler))
            .route(
                "/auth_status",
                get(routes::health_check::auth_status::handler),
            )
            .route("/rpc_dashboard", get(routes::rpc::rpc_dashboard::handler))
            .route("/rpc_api", get(routes::rpc::rpc_api::handler))
            .route(
                "/notification",
                get(routes::notification::notification_page::handler),
            )
            .route(
                "/email_confirm",
                get(routes::emails::email_confirm::handler),
            )
            .route(
                "/reset_password",
                get(routes::password::reset_password_form::handler)
                    .post(routes::password::reset_password_post::handler),
            )
            .route(
                "/resend_confirmation_email",
                get(routes::emails::resend_confirm_email_form::handler)
                    .post(routes::emails::resend_confirm_email_post::handler),
            )
            .route(
                "/new_password",
                get(routes::password::new_password_form::handler)
                    .post(routes::password::new_password_post::handler),
            )
            .route(
                "/",
                get(routes::login::login_form::handler).post(routes::login::login_post::handler),
            )
            .route("/error", get(routes::error::error_page::handler))
            .route(
                "/login",
                get(routes::login::login_form::handler).post(routes::login::login_post::handler),
            )
            .route(
                "/register_api",
                post(routes::register::register_api::handler),
            )
            .route(
                "/register",
                get(routes::register::register_form::handler)
                    .post(routes::register::register_post::handler),
            )
            .route(
                "/api/check_token",
                post(routes::api_token::check_token::handler),
            )
            .route(
                "/api/get_email_via_token",
                post(routes::api_token::get_email_via_token::handler),
            )
            .route("/health_check", get(routes::health_check::get::handler));
        let leptos_config = get_config_from_env().unwrap();
        let leptos_options = leptos_config.leptos_options;
        let routes = generate_route_list(App);

        let path = Path::new("")
            .join(leptos_options.site_root.clone())
            .join(leptos_options.site_pkg_dir.clone());
        let leptos_pkg: Router<()> = Router::new().nest_service(
            &format!("/{}", leptos_options.site_pkg_dir),
            ServeDir::new(path), // ServeDir::new(path).fallback(ServeFile::new(
                                 //     "libs/block-mesh-manager/js-src/connectors.js",
                                 // )),
        );

        let leptos_router: Router<()> = Router::new()
            .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
            .leptos_routes(&leptos_options, routes, App)
            .with_state(leptos_options);

        let application_base_url = ApplicationBaseUrl(settings.application.base_url.clone());
        let backend = Router::new()
            .nest("/", auth_router)
            .route_layer(login_required!(Backend, login_url = "/login"))
            .nest("/api", api_router)
            .nest("/", un_auth_router)
            .layer(Extension(application_base_url))
            .layer(Extension(db_pool.clone()))
            .layer(cors)
            .layer(auth_layer)
            .with_state(app_state.clone());

        let app = Router::new()
            .nest("/", leptos_router)
            .nest("/", backend)
            .nest("/", leptos_pkg);

        let listener = TcpListener::bind(settings.application.address())
            .await
            .unwrap();
        tracing::info!("Listening on {}", listener.local_addr().unwrap());
        Application { app, listener }
    }

    pub async fn run(self) -> std::io::Result<()> {
        axum::serve(
            self.listener,
            self.app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
    }

    pub fn address(&self) -> String {
        format!("{}", self.listener.local_addr().unwrap())
    }

    pub fn port(&self) -> u16 {
        self.listener.local_addr().unwrap().port()
    }
}
