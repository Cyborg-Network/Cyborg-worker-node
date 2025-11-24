use futures_util::stream::StreamExt;
use zbus::{message::Type::Signal, Connection, MatchRule, MessageStream};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn watch_for_zk_stage_update(zk_stage: Arc<Mutex<u8>>) -> zbus::Result<()> {
    println!("Connecting to D-Bus function called...");
    let connection = Connection::system().await?;
    let rule = MatchRule::builder()
        .msg_type(Signal)
        .sender("com.cyborg.CyborgAgent")?
        .interface("com.cyborg.AgentZkInterface")?
        .member("ZkUpdate")?
        .build();

    let mut msg_stream = MessageStream::for_match_rule(rule, &connection, None).await?;

    //let systemd_proxy = ZkUpdateManagerProxy::new(&connection).await?;
    //let mut zk_update_stream = systemd_proxy.receive_zk_update().await?;

    println!("Waiting for zk updates...");

    while let Some(msg) = msg_stream.next().await {
        // `ZkUpdateArgs` should contain the arguments expected in the signal
        match msg{
            Ok(msg) => {
                let body = msg.body();
                let zk_update_args: zbus::zvariant::Structure = body.deserialize()?;
                let current_zk_stage = &zk_update_args.into_fields()[0];
                match current_zk_stage {
                    zbus::zvariant::Value::U8(stage) => {
                        println!("Current zk stage: {}", stage);
                        *zk_stage.lock().await = *stage;
                    },
                    _ => println!("Field 1 not a u8"),
                }
            },
            Err(e) => println!("Error receiving message: {}", e),
        }

    }

    panic!("Stream ended unexpectedly");
}