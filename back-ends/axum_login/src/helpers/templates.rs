use tera::Context;
use std::collections::HashMap;
use std::env;

/// This function returns a tera context with the tranlsations, css and more
pub fn get_email_context(translations: &HashMap<String, String>) -> Context {
    let admin_email: String = env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set");
    let logo_url: String = env::var("LOGO_URL").expect("LOGO_URL must be set");
    let app_name: String = env::var("APP_NAME").expect("APP_NAME must be set");
    let app_main_front_end: String = env::var("APP_MAIN_FRONT_END").expect("APP_MAIN_FRONT_END must be set");

    let mut context = Context::new();

    // Insert the translations in the context
    for (k, v) in translations {
        context.insert(k, v);
    }

    // Insert the css in the context, we cannot put a link in the templates themself because it is ignored by most email clients
    let inline_css = std::fs::read_to_string("templates/style.css").unwrap();
    context.insert("inline_css", &inline_css);

    // Insert the admin_email to the context
    context.insert("admin_email", admin_email.as_str());

    // Insert the logo_url in the context
    context.insert("logo_url", logo_url.as_str());

    // Insert the app_name in the context
    context.insert("app_name", app_name.as_str());

    // Insert the app_main_front_end in the context
    context.insert("app_main_front_end", app_main_front_end.as_str());

    context
}