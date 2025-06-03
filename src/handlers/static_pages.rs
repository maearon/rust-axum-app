use axum::{extract::State, response::Html};
use crate::{app::AppState, templates::StaticPageTemplate};

pub async fn home(State(_state): State<AppState>) -> Html<String> {
    let template = StaticPageTemplate {
        title: "Home".to_string(),
        content: "Welcome to the Sample App".to_string(),
        page: "home".to_string(),
    };
    Html(template.render().unwrap())
}

pub async fn about(State(_state): State<AppState>) -> Html<String> {
    let template = StaticPageTemplate {
        title: "About".to_string(),
        content: "This is the About page for the Sample App".to_string(),
        page: "about".to_string(),
    };
    Html(template.render().unwrap())
}

pub async fn help(State(_state): State<AppState>) -> Html<String> {
    let template = StaticPageTemplate {
        title: "Help".to_string(),
        content: "Get help on the Sample App".to_string(),
        page: "help".to_string(),
    };
    Html(template.render().unwrap())
}

pub async fn contact(State(_state): State<AppState>) -> Html<String> {
    let template = StaticPageTemplate {
        title: "Contact".to_string(),
        content: "Contact the Sample App".to_string(),
        page: "contact".to_string(),
    };
    Html(template.render().unwrap())
}
