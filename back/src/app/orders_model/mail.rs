use std::{env, sync::Arc};

use image::{codecs::png, ImageEncoder, Luma};
use lettre::{
    message::{Attachment, Mailbox, MultiPart, SinglePart},
    Message,
};
use sqlx::MySqlPool;

use crate::{
    errors::{SendReceiptEmailError, ServerError},
    mail_manager::MailManager,
};

use super::orders::Order;

type SmtpUsername = String;
type SmtpPassword = String;
fn get_smtp_credentials() -> (SmtpUsername, SmtpPassword) {
    let smtp_username = env::var("SMTP_USERNAME").expect("env var SMTP_USERNAME not found");
    let smtp_password = env::var("SMTP_PASSWORD").expect("env var SMTP_PASSWORD not found");
    (smtp_username, smtp_password)
}
pub async fn send_qr(
    pool: &MySqlPool,
    mail_manager: Arc<Box<dyn MailManager>>,
    order: &Order,
) -> Result<(), SendReceiptEmailError> {
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
    let body = SinglePart::plain(format!(
        "Merci pour votre commande.
Vous trouverez en pièce jointe le qr-code à montrer au bar.

Résumé de votre commande :
{}

Total: {}€


Reçu: {}",
        order
            .get_details(pool)
            .await?
            .iter()
            .map(|d| format!(
                "{} x {} = {}€",
                d.quantity,
                d.item_name,
                d.subtotal_ttc / 100
            ))
            .collect::<Vec<String>>()
            .join("\n"),
        order.get_full_price_ttc(pool).await? / 100,
        *receipt
    ));
    let creds = get_smtp_credentials();
    let from: Mailbox = creds.0.parse()?;
    let email = Message::builder()
        .from(from)
        .to(to.clone())
        .subject("Merci pour votre commande")
        .multipart(MultiPart::mixed().singlepart(body).singlepart(attachment))
        .map_err(ServerError::EmailBuild)?;
    mail_manager.send_mail(email).await?;
    Ok(())
}
