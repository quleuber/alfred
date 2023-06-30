use bastion::prelude::*;
use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug, Clone)]
pub struct PhoneNotif {
    pub device: String,
    pub title: String,
    pub text: String,
    pub app_name: String,
    pub app_id: String,
    pub channel_id: Option<String>,
    pub when: Option<NaiveDateTime>,
    pub local_recieved_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct NotifJson {
    device: String,
    title: String,
    text: String,
    app_name: String,
    app_id: String,
    channel_id: Option<String>,
    when: Option<u32>,
}

impl From<NotifJson> for PhoneNotif {
    fn from(val: NotifJson) -> Self {
        PhoneNotif {
            device: val.device,
            title: val.title,
            text: val.text,
            app_name: val.app_name,
            app_id: val.app_id,
            channel_id: val.channel_id,
            local_recieved_at: Utc::now(),
            when: val
                .when
                .and_then(|t| NaiveDateTime::from_timestamp_opt(t as i64, 0)),
        }
    }
}

pub async fn collector(ctx: BastionContext) -> Result<(), ()> {
    loop {
        msg! { ctx.recv().await?,
          notf: PhoneNotif =!> {
            println!("Received a message: {:?}", notf);
          };
          _: _ => ();
        };
    }
}
