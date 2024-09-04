use image::{codecs::png, ImageEncoder, Luma};
use lettre::{
    message::{Attachment, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::errors::{SendReceiptEmailError, ServerError};

use super::{config::config, orders::Order};

impl Order {
    pub async fn send_qr(&self) -> Result<(), SendReceiptEmailError> {
        let to: Mailbox = self
            .user_email
            .clone()
            .ok_or_else(|| SendReceiptEmailError::NoEmailAddress)?
            .parse()?;
        let receipt = self
            .receipt
            .clone()
            .ok_or_else(|| SendReceiptEmailError::NoReceipt)?;

        let img = receipt.get_qr_code()?.render::<Luma<u8>>().build();
        let mut res: Vec<u8> = vec![];
        png::PngEncoder::new(&mut res).write_image(
            &img,
            img.width(),
            img.height(),
            image::ExtendedColorType::L8,
        )?;
        let attachment =
            Attachment::new("recu.png".to_owned()).body(res, "image/png".parse().unwrap());
        let body = SinglePart::plain(
            format!("Merci pour votre commande.\nVous trouverez en pièce jointe le qr-code à montrer au bar.\n\nRésumé de votre commande :\n{}\n\n\nReçu: {}",
                self.get_details().await?.iter().map(|d|format!("{}x{} = {}€", d.name, d.quantity, d.subtotal/100)).collect::<Vec<String>>().join("\n"),
                *receipt
            ),
        );
        let conf = config().read().await;
        let username = conf.smtp_username();
        let password = conf.smtp_password();
        let from: Mailbox = username.to_owned().parse()?;
        let email = Message::builder()
            .from(from)
            .to(to.clone())
            .subject("connexion à lhavrais-pay")
            .multipart(MultiPart::mixed().singlepart(body).singlepart(attachment))
            .map_err(ServerError::EmailBuild)?;
        // Open a remote connection to gmail using STARTTLS
        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.gmail.com")
                .unwrap()
                .credentials(Credentials::new(username.to_owned(), password.to_owned()))
                .build();

        // Send the email
        mailer.send(email).await.map_err(ServerError::EmailSend)?;
        Ok(())
    }
}
