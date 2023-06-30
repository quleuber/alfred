mod actors;

use std::future::Future;
use std::pin::Pin;

use bastion::prelude::*;
use tide::Request;

use crate::actors::notifications::{self, PhoneNotif};

type BastionFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

fn main() -> Result<(), ()> {
    Bastion::init();
    Bastion::start();

    eprintln!("Starting notifications collector actor...");
    Bastion::children(|c| {
        c.with_distributor(Distributor::named("notifications"))
            .with_exec(notifications::collector)
    })
    .expect("couldn't start notifications_collector");

    // let pass_notification = |notif: notifications::NotifJson| {
    //     notification_ref.elems()[0]
    //         .tell_anonymously(notif)
    //         .expect("couldn't send notification")
    // };

    Bastion::children(|children| children.with_exec(tide_task())).expect("couldn't start tide");

    Bastion::block_until_stopped();
    Ok(())
}

fn tide_task() -> impl Fn(BastionContext) -> BastionFuture<Result<(), ()>> {
    |_: BastionContext| {
        Box::pin(async {
            femme::start();
            let mut app = tide::new();
            app.at("/").get(|_| async move { Ok("Hello, world!") });
            app.at("/collect/notification")
                .post(handle_notification);
            let res = app.listen("0.0.0.0:8080").await;
            match res {
                Err(e) => panic!("Error: {}", e),
                Ok(_) => Ok(()),
            }
        })
    }
}

async fn handle_notification(mut req: Request<()>) -> tide::Result {
    let notif_dist = Distributor::named("notifications");
    let notif: notifications::NotifJson = req.body_json().await?;
    // eprintln!("Received notification: {:?}", notif);
    // let notif = notifications::PhoneNotif {
    //     device: "ferris".to_string(),
    //     title: "Test Title".to_string(),
    //     text: "Test Body".to_string(),
    //     app_name: "Test App".to_string(),
    //     app_id: "com.test.app".to_string(),
    //     channel_id: None,
    //     when: None,
    //     local_recieved_at: chrono::Utc::now(),
    // };
    // eprintln!("Posting notification message: {:?}", notif);
    notif_dist.ask_one(PhoneNotif::from(notif.clone()))?;
    Ok(format!("You've posted the notification {:?}", notif).into())
}
