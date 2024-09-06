use std::env;

use image::{codecs::png, ImageEncoder, Luma};
use lettre::{
    message::{Attachment, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::errors::{SendReceiptEmailError, ServerError};

use super::orders::Order;

type SmtpUsername = String;
type SmtpPassword = String;
fn get_smtp_credentials() -> (SmtpUsername, SmtpPassword) {
    let smtp_username = env::var("SMTP_USERNAME").expect("env var SMTP_USERNAME not found");
    let smtp_password = env::var("SMTP_PASSWORD").expect("env var SMTP_PASSWORD not found");
    (smtp_username, smtp_password)
}
pub async fn send_qr(order: &Order) -> Result<(), SendReceiptEmailError> {
    let to: Mailbox = order
        .user_email
        .clone()
        .ok_or_else(|| SendReceiptEmailError::NoEmailAddress)?
        .parse()?;
    let receipt = order
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
    let attachment = Attachment::new("recu.png".to_owned()).body(res, "image/png".parse().unwrap());
    let body = SinglePart::plain(
            format!("Merci pour votre commande.\nVous trouverez en pièce jointe le qr-code à montrer au bar.\n\nRésumé de votre commande :\n{}\n\n\nReçu: {}",
                order.get_details().await?.iter().map(|d|format!("{}x{} = {}€", d.product_name, d.quantity, d.subtotal_ttc/100)).collect::<Vec<String>>().join("\n"),
                *receipt
            ),
        );
    let creds = get_smtp_credentials();
    let from: Mailbox = creds.0.parse()?;
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
            .credentials(Credentials::new(creds.0, creds.1))
            .build();

    // Send the email
    mailer.send(email).await.map_err(ServerError::EmailSend)?;
    Ok(())
}
