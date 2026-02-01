use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use std::env;
use axum::http::StatusCode;
use log::error;

/// This function takes a lettre message, and sends it using the mailer specified in .env
pub async fn send_mail(email: Message) -> Result<(), StatusCode> {

    #[cfg(debug_assertions)]
    {
        let use_mailhog = env::var("MAILHOG")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase() == "true";

        // In dev only, if the MAILHOG env var is true
        if use_mailhog {
            let mailer = SmtpTransport::relay("localhost")
                .unwrap()
                .port(1025) // Mailhog port
                .tls(lettre::transport::smtp::client::Tls::None) // No TLS for Mailhog
                .credentials(Credentials::new("root".to_owned(), "root".to_owned()))
                .build();

            return match mailer.send(&email) {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!("(DEV - Mailhog) Error sending the email: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            };
        }

        // In mail only, if TEST_MAIL var is true, not send any email at all
        let not_send_email = env::var("TEST_MAIL")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase() == "true";

        if not_send_email {
            return Ok(());
        }
    }

    // Prod or if the env var MAILHOG is false
    let mail_server_url = env::var("MAIL_SERVER_URL").expect("MAIL_SERVER_URL must be set");
    let mail_server_username = env::var("MAIL_SERVER_USERNAME").expect("MAIL_SERVER_USERNAME must be set");
    let mail_server_password = env::var("MAIL_SERVER_PASSWORD").expect("MAIL_SERVER_PASSWORD must be set");

    let creds = Credentials::new(mail_server_username, mail_server_password);

    let mailer = SmtpTransport::relay(&mail_server_url)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error sending the email : {}", e);

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
