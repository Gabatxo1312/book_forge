use std::{env, sync::Arc};

use dotenvy::dotenv;
use minijinja::Environment;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub template_env: Environment<'static>
}

impl AppConfig {
    pub fn from_env() -> Arc<AppConfig> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");

        let mut env = minijinja::Environment::new();
        minijinja_embed::load_templates!(&mut env);

        Arc::new(Self {
            database_url: database_url.clone(),
            template_env: env
        })
    }
}

