use std::env;

use axum::async_trait;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

use crate::errors::ServerError;

#[async_trait]
pub trait MailManager: Send + Sync {
    async fn send_mail(&self, message: Message) -> Result<(), ServerError>;
}

pub struct GmailManager {}

#[async_trait]
impl MailManager for GmailManager {
    async fn send_mail(&self, message: Message) -> Result<(), ServerError> {
        let smtp_username = env::var("SMTP_USERNAME").unwrap_or("".into());
        let smtp_password = env::var("SMTP_PASSWORD").unwrap_or("".into());
        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.gmail.com")?
                .credentials(Credentials::new(smtp_username, smtp_password))
                .build();

        // Send the email
        mailer.send(message).await?;
        Ok(())
    }
}
