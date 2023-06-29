use bastion::prelude::*;
use serde::Deserialize;
use tide::Request;
// use tide::prelude::*;

#[fort::root]

async fn main(_: BastionContext) -> Result<(), ()> {
    let mut app = tide::new();
    app.at("/").get(|_| async move { Ok("Hello, world!") });
    app.at("/orders/shoes").post(order_shoes);
    let res = app.listen("127.0.0.1:8080").await;
    match res {
        Err(e) => panic!("Error: {}", e),
        Ok(_) => Ok(())
    }
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}
