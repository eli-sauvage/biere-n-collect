use std::collections::HashMap;

use serde::Serialize;
use sqlx::MySqlPool;
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

pub async fn process_orders_to_report(
    pool: &MySqlPool,
    orders: Vec<Order>,
) -> Result<Report, ServerError> {
    let mut unique_items: HashMap<String, ReportItem> = HashMap::new();
    let mut handles: JoinSet<Result<Vec<OrderDetailElement>, ServerError>> = JoinSet::new();
    orders.into_iter().for_each(|order| {
        let thread_pool = pool.to_owned();
        handles.spawn(async move { order.get_details(&thread_pool).await });
    });
    handles
        .join_all()
        .await
        .into_iter()
        .collect::<Result<Vec<Vec<OrderDetailElement>>, ServerError>>()?
        .into_iter()
        .flatten()
        .for_each(|order_detail| {
            if let Some(item) = unique_items.get_mut(&order_detail.item_name) {
                item.quantity += order_detail.quantity;
                item.subtotal_ttc += order_detail.subtotal_ttc;
                item.subtotal_ht += order_detail.subtotal_ht;
            } else {
                let item = ReportItem {
                    item_name: order_detail.item_name.clone(),
                    quantity: order_detail.quantity,
                    tva: order_detail.tva,
                    subtotal_ht: order_detail.subtotal_ht,
                    subtotal_ttc: order_detail.subtotal_ttc,
                };
                unique_items.insert(order_detail.item_name, item);
            }
        });
    Ok(unique_items.into_values().collect())
}

#[sqlx::test]
async fn test_process_orders_to_report(pool: MySqlPool) {
    use crate::app::{
        orders::{Cart, CartElement},
        product_variations::Variation,
    };
    use serde_json::json;
    println!("{:?}", json!([{"a":3}]));
    let cart = |variation_id: u32| Cart {
        elements: vec![CartElement {
            variation_id,
            quantity: 1,
        }],
    };
    let variation_1 = Variation::get(&pool, 1).await.unwrap().unwrap();
    let order1 = Order::generate_from_cart(&pool, cart(variation_1.id))
        .await
        .unwrap();
    let order1 = Order::get(&pool, order1).await.unwrap().unwrap();
    let order2 = Order::generate_from_cart(&pool, cart(variation_1.id))
        .await
        .unwrap();
    let order2 = Order::get(&pool, order2).await.unwrap().unwrap();

    let variation_2 = Variation::get(&pool, 2).await.unwrap().unwrap();
    let order3 = Order::generate_from_cart(&pool, cart(variation_2.id))
        .await
        .unwrap();
    let order3 = Order::get(&pool, order3).await.unwrap().unwrap();
    let item_name_1 = order1.get_details(&pool).await.unwrap()[0]
        .item_name
        .clone();
    let item_name_2 = order3.get_details(&pool).await.unwrap()[0]
        .item_name
        .clone();
    let report = process_orders_to_report(&pool, vec![order1, order2, order3])
        .await
        .unwrap();

    assert_eq!(
        report.iter().filter(|e| e.item_name == item_name_1).count(),
        1
    );
    assert_eq!(
        report.iter().filter(|e| e.item_name == item_name_2).count(),
        1
    );

    assert_eq!(
        report
            .iter()
            .find(|e| e.item_name == item_name_1)
            .unwrap()
            .quantity,
        2
    );
    assert_eq!(
        report
            .iter()
            .find(|e| e.item_name == item_name_2)
            .unwrap()
            .quantity,
        1
    );

    assert_eq!(
        report
            .iter()
            .find(|e| e.item_name == item_name_1)
            .unwrap()
            .subtotal_ht,
        2 * variation_1.price_ht
    );
    assert_eq!(
        report
            .iter()
            .find(|e| e.item_name == item_name_2)
            .unwrap()
            .subtotal_ht,
        variation_2.price_ht
    );
}
