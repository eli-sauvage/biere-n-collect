use std::collections::HashMap;

use serde::Serialize;
use tokio::task::JoinSet;

use crate::{
    app::orders::{Order, OrderDetailElement},
    errors::ServerError,
};

pub type Report = Vec<ReportItem>;
#[derive(Serialize)]
pub struct ReportItem {
    item_name: String,
    quantity: u32,
    tva: f32,
    subtotal_ht: i32,
    subtotal_ttc: i32,
}

pub async fn process_orders_to_report(orders: Vec<Order>) -> Result<Report, ServerError> {
    let mut unique_items: HashMap<String, ReportItem> = HashMap::new();
    let mut handles: JoinSet<Result<Vec<OrderDetailElement>, ServerError>> = JoinSet::new();
    orders.into_iter().for_each(|order| {
        handles.spawn(async move { order.get_details().await });
    });
    handles
        .join_all()
        .await
        .into_iter()
        .collect::<Result<Vec<Vec<OrderDetailElement>>, ServerError>>()?
        .into_iter()
        .flatten()
        .for_each(|order_detail| {
            let item_name = if order_detail.variation_name.is_empty() {
                order_detail.product_name
            } else {
                format!(
                    "{} ({})",
                    order_detail.product_name, order_detail.variation_name
                )
            };
            if let Some(item) = unique_items.get_mut(&item_name) {
                item.quantity += order_detail.quantity as u32;
                item.subtotal_ttc += order_detail.subtotal_ttc;
                item.subtotal_ht += order_detail.subtotal_ht;
            } else {
                let item = ReportItem {
                    item_name: item_name.clone(),
                    quantity: order_detail.quantity as u32,
                    tva: order_detail.tva,
                    subtotal_ht: order_detail.subtotal_ht,
                    subtotal_ttc: order_detail.subtotal_ttc,
                };
                unique_items.insert(item_name, item);
            }
        });
    Ok(unique_items.into_values().collect())
}
