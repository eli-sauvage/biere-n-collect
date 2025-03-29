use std::env;

use axum::async_trait;
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::errors::ServerError;

#[async_trait]
pub trait MailManager: Send + Sync {
    async fn send_mail(&self, message: Message) -> Result<(), ServerError>;
    fn get_sender(&self) -> Result<Mailbox, ServerError>;
}

pub struct GmailManager {}

#[async_trait]
impl MailManager for GmailManager {
    async fn send_mail(&self, message: Message) -> Result<(), ServerError> {
        let mailer: AsyncSmtpTransport<Tokio1Executor>;
        #[cfg(not(feature = "local-smtp-testing"))]
        {
            let smtp_username = env::var("SMTP_USERNAME").unwrap_or("".into());
            let smtp_password = env::var("SMTP_PASSWORD").unwrap_or("".into());
            let smtp_server = env::var("SMTP_SERVER").unwrap_or("".into());
            mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server)?
                .credentials(Credentials::new(smtp_username, smtp_password))
                .build();
        }
        #[cfg(feature = "local-smtp-testing")]
        {
            use lettre::transport::smtp::client::{Tls, TlsParameters};
            let tls = TlsParameters::builder("localhost".to_string());
            // WARNING: making the TLS client accept any certificate is very unsafe and shouldn't be used in production.
            // #473 will allow configuring a custom root certificate.
            let tls = tls.dangerous_accept_invalid_certs(true);
            let tls = tls.build().unwrap();

            mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("localhost")
                .port(25)
                .tls(Tls::Required(tls))
                .credentials(Credentials::new(
                    "username".to_owned(),
                    "password".to_owned(),
                ))
                .build();
        }

        // Send the email
        mailer.send(message).await?;
        println!("sent");
        Ok(())
    }
    fn get_sender(&self) -> Result<Mailbox, ServerError> {
        env::var("SMTP_USERNAME")
            .unwrap_or("".into())
            .parse()
            .map_err(ServerError::EmailAddress)
    }
}

#[cfg(test)]
use tokio::sync::RwLock;

#[cfg(test)]
#[derive(Default)]
pub struct TestMailManager {
    pub received_mail: RwLock<Vec<Message>>,
}

#[cfg(test)]
#[async_trait]
impl MailManager for TestMailManager {
    async fn send_mail(&self, message: Message) -> Result<(), ServerError> {
        self.received_mail.write().await.push(message);
        Ok(())
    }
    fn get_sender(&self) -> Result<Mailbox, ServerError> {
        String::from("test@example.com")
            .parse()
            .map_err(ServerError::EmailAddress)
    }
}
