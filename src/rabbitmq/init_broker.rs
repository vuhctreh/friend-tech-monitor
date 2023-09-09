use eyre::Result;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::publisher_confirm::Confirmation;
use lapin::types::FieldTable;
use log;
use simple_logger::SimpleLogger;

const ADDRESS: &str = "amqp://127.0.0.1:5672/%2f";

pub async fn rabbit_init() -> Result<Channel> {
    let conn: Connection = Connection::connect(
        ADDRESS.into(),
        ConnectionProperties::default(),
    )
        .await?;

    log::info!("CONNECTED");

    let channel_a: Channel = conn.create_channel().await?;

    let queue = channel_a
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    log::info!("Declared queue");

    // let payload: &[u8] = b"The slow grey wolf";
    //
    // let confirm = channel_a
    //     .basic_publish(
    //         "",
    //         "hello",
    //         BasicPublishOptions::default(),
    //         payload,
    //         BasicProperties::default(),
    //     )
    //     .await?
    //     .await?;

    Ok(channel_a)
}

#[tokio::test]
async fn rabbit_test() -> Result<()> {
    dotenvy::dotenv().expect("ERROR: Could not load .env file.");
    SimpleLogger::new().init().unwrap();

    rabbit_init().await?;
    Ok(())
}