use std::sync::Arc;

use axum::{extract::State, response::Html};

use crate::config::AppConfig;

pub fn render_template(
    state: State<Arc<AppConfig>>,
    template_name: &str,
    context: minijinja::Value
) -> Html<String>{
    let template = state
        .template_env
        .get_template(template_name)
        .unwrap();
    let result = template
        .render(context)
        .unwrap();

    Html(result)
}
