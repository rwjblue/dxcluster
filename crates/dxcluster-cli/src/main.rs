use dxcluster_client::{TelnetClient, TelnetOptions};
use dxcluster_wire::UserCommand;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = TelnetClient::connect("127.0.0.1:7300", TelnetOptions::default()).await?;
    client
        .send_command(UserCommand::Raw(String::from("sh/dx")))
        .await?;
    if let Some(event) = client.next_event().await {
        println!("event: {:?}", event?);
    }
    Ok(())
}
