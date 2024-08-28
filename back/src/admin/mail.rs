use std::env;

use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::errors::ServerError;

type SmtpUsername = String;
type SmtpPassword = String;
fn get_smtp_credentials() -> (SmtpUsername, SmtpPassword) {
    let smtp_username = env::var("SMTP_USERNAME").expect("env var SMTP_USERNAME not found");
    let smtp_password = env::var("SMTP_PASSWORD").expect("env var SMTP_PASSWORD not found");
    (smtp_username, smtp_password)
}

pub async fn send_code(to: &Mailbox, code: String) -> Result<(), ServerError> {
    let creds = get_smtp_credentials();
    let from: Mailbox = creds.0.parse()?;
    let email = Message::builder()
            .from(from)
            .to(to.clone())
            .subject("connexion à lhavrais-pay")
            .header(ContentType::TEXT_PLAIN)
            .body(format!("Une tentative de connexion pour le compte \n{to}\n a été détéctée.\n\n\nAfin de vous connecter, veuillez saisir le code    \n\n{code}\n\n dans l'invite.\n\n\n--------\n\nIgnorez ce message si vous n'êtes pas à l'origine de la connexion."))
            ?;

    // Open a remote connection to gmail using STARTTLS
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.gmail.com")
            .unwrap()
            .credentials(Credentials::new(creds.0, creds.1))
            .build();

    // Send the email
    mailer.send(email).await?;
    Ok(())
}
