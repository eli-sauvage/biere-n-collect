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
    let smtp_username = env::var("SMTP_USERNAME").unwrap_or("".into());
    let smtp_password = env::var("SMTP_PASSWORD").unwrap_or("".into());
    (smtp_username, smtp_password)
}

pub async fn send_code(to: &Mailbox, code: [u8; 6]) -> Result<(), ServerError> {
    let code = code
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .chunks(2)
        .map(|chunk| chunk.concat())
        .collect::<Vec<String>>();
    let creds = get_smtp_credentials();
    let from: Mailbox = creds.0.parse()?;
    let login_link = env::var("VITE_SITE_URL")
        .map(|e| format!("{e}/login?email={to}&code={}", code.join("")))
        .unwrap_or("".to_string());
    let email = Message::builder()
        .from(from)
        .to(to.clone())
        .subject(format!("connexion à biere n collect pour le bar {}", env::var("VITE_BAR_NAME").unwrap_or("".into())))
        .header(ContentType::TEXT_PLAIN)
        .body(format!(
            "Une tentative de connexion pour le compte
            {to}
            a été détéctée.
            \n\nAfin de vous connecter, veuillez saisir le code \n{}\n dans l'invite.
            Vous pouvez également cliquer sur le lien ci dessous :
            {login_link}
            \n\n\n--------\n\n<i>Ignorez ce message si vous n'êtes pas à l'origine de la connexion.</i>",
            code.join(" - ")
        ))?;
    // Open a remote connection to gmail using STARTTLS
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.gmail.com")?
            .credentials(Credentials::new(creds.0, creds.1))
            .build();

    // Send the email
    mailer.send(email).await?;
    Ok(())
}
