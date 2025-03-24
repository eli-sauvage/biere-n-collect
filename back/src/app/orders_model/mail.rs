use std::sync::Arc;

use image::{codecs::png, ImageEncoder, Luma};
use lettre::{
    message::{Attachment, Mailbox, MultiPart, SinglePart},
    Message,
};
use sqlx::SqlitePool;

use crate::{
    errors::{SendReceiptEmailError, ServerError},
    mail_manager::MailManager,
};

use super::orders::Order;

pub async fn send_qr(
    pool: &SqlitePool,
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
    let email = Message::builder()
        .to(to.clone())
        .from(mail_manager.get_sender()?)
        .subject("Merci pour votre commande")
        .multipart(MultiPart::mixed().singlepart(body).singlepart(attachment))
        .map_err(ServerError::EmailBuild)?;
    mail_manager.send_mail(email).await?;
    Ok(())
}

pub async fn send_notification(
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

    let body = SinglePart::plain(format!(
        "Votre commande est prête !

Vous pouvez venir la récupérer au bar dès maintenant.

Reçu: {}",
        *receipt
    ));
    let email = Message::builder()
        .to(to.clone())
        .from(mail_manager.get_sender()?)
        .subject("Merci pour votre commande")
        .singlepart(body)
        .map_err(ServerError::EmailBuild)?;
    mail_manager.send_mail(email).await?;
    Ok(())
}
