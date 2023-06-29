use bastion::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct PhoneNotification {
  device: String,
  title: String,
  text: String,
  app_name: String,
  app_id: String,
  channel_id: String,
  when: DateTime<Utc>,
}

pub async fn notifications_collector(ctx: BastionContext) -> Result<(), ()> {

  msg! { ctx.recv().await?,
    notf: PhoneNotification =!> {
      println!("Received a message: {:?}", notf);
    };
    _: _ => ();
  };

  Ok(())
}
