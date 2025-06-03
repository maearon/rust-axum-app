use askama::Template;

#[derive(Template)]
#[template(path = "static_page.html")]
pub struct StaticPageTemplate {
    pub title: String,
    pub content: String,
    pub page: String,
}

#[derive(Template)]
#[template(path = "users/signup.html")]
pub struct SignupTemplate {
    pub title: String,
    pub errors: Vec<String>,
}

#[derive(Template)]
#[template(path = "users/login.html")]
pub struct LoginTemplate {
    pub title: String,
    pub errors: Vec<String>,
}
