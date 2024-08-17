use std::env;

use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

pub struct MailManager {
    smtp_username: String,
    smtp_password: String,
}
impl MailManager {
    pub fn new() -> MailManager {
        let smtp_username = env::var("SMTP_USERNAME").expect("env var SMTP_USERNAME not found");
        let smtp_password = env::var("SMTP_PASSWORD").expect("env var SMTP_PASSWORD not found");
        MailManager {
            smtp_username,
            smtp_password,
        }
    }
    pub async fn send_code(&self, to: String, code: String) {
        let email = Message::builder()
            .from(self.smtp_username.parse().unwrap())
            .to(to.parse().unwrap())
            .subject("connextion à lhavrais-pay")
            .header(ContentType::TEXT_PLAIN)
            .body(format!("Une tentative de connexion pour le compte \n{to}\n a été détéctée.\n\n\nAfin de vous connecter, veuillez saisir le code    \n\n{code}\n\n dans l'invite.\n\n\n--------\n\nIgnorez ce message si vous n'êtes pas à l'origine de la connexion."))
            .unwrap();

        let creds = Credentials::new(self.smtp_username.to_owned(), self.smtp_password.to_owned());

        // Open a remote connection to gmail using STARTTLS
        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

        // Send the email
        match mailer.send(email).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }
}
