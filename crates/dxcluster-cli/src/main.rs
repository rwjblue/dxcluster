use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use dxcluster_client::{ClientEvent, TelnetClient, TelnetOptions};
use dxcluster_types::{Callsign, FrequencyHz};
use dxcluster_wire::{ServerLine, UserCommand, user::ShowCommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "Interact with dxcluster nodes over telnet", long_about = None)]
struct Cli {
    /// DX cluster server address (host:port)
    #[arg(long, global = true, default_value = "127.0.0.1:7300")]
    addr: String,

    /// Optional callsign to log in with
    #[arg(long, global = true)]
    callsign: Option<String>,

    /// Optional password to send during login
    #[arg(long, global = true)]
    password: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Connect to a cluster node and print the first response
    Login,
    /// Send a DX spot to the cluster
    Spot(SpotArgs),
    /// Request a list of recent spots
    List,
    /// Stream cluster output until disconnected
    Watch,
}

#[derive(Debug, Args, Clone)]
struct SpotArgs {
    /// Callsign being spotted
    dx: String,
    /// Frequency in kHz (e.g. 14074)
    frequency: String,
    /// Additional comment for the spot
    comment: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Login => login(&cli).await?,
        Commands::Spot(args) => spot(&cli, args.clone()).await?,
        Commands::List => list(&cli).await?,
        Commands::Watch => watch(&cli).await?,
    }

    Ok(())
}

async fn login(cli: &Cli) -> Result<()> {
    let mut client = connect(cli).await?;
    println!("connected to {}", cli.addr);
    if let Some(event) = client.next_event().await.transpose()? {
        print_event(&event);
    }
    Ok(())
}

async fn spot(cli: &Cli, args: SpotArgs) -> Result<()> {
    let client = connect(cli).await?;
    let command = args.into_command()?;
    client
        .send_command(command)
        .await
        .context("failed to submit spot")?;
    println!("spot submitted to {}", cli.addr);
    Ok(())
}

async fn list(cli: &Cli) -> Result<()> {
    let mut client = connect(cli).await?;
    client
        .send_command(UserCommand::Show(ShowCommand::Dx))
        .await
        .context("failed to request spots")?;

    while let Some(event) = client.next_event().await.transpose()? {
        print_event(&event);
        if matches!(event, ClientEvent::UserLine(ServerLine::Prompt)) {
            break;
        }
    }

    Ok(())
}

async fn watch(cli: &Cli) -> Result<()> {
    let mut client = connect(cli).await?;
    println!("listening for cluster output from {}", cli.addr);
    while let Some(event) = client.next_event().await.transpose()? {
        print_event(&event);
    }
    Ok(())
}

impl SpotArgs {
    fn into_command(self) -> Result<UserCommand> {
        let dx = Callsign::parse_loose(&self.dx).context("invalid callsign")?;
        let frequency = FrequencyHz::from_khz_str(&self.frequency).context("invalid frequency")?;
        let comment = self.comment.join(" ");

        Ok(UserCommand::Dx {
            dx,
            frequency,
            comment,
        })
    }
}

fn build_options(cli: &Cli) -> Result<TelnetOptions> {
    let callsign = cli
        .callsign
        .as_deref()
        .map(Callsign::parse_loose)
        .transpose()
        .context("invalid callsign")?;

    Ok(TelnetOptions {
        callsign,
        password: cli.password.clone(),
    })
}

async fn connect(cli: &Cli) -> Result<TelnetClient> {
    let options = build_options(cli)?;
    TelnetClient::connect(&cli.addr, options)
        .await
        .with_context(|| format!("failed to connect to {}", cli.addr))
}

fn print_event(event: &ClientEvent) {
    match event {
        ClientEvent::UserLine(ServerLine::Banner(msg)) => println!("banner: {msg}"),
        ClientEvent::UserLine(ServerLine::Prompt) => println!(">"),
        ClientEvent::UserLine(ServerLine::Spot(spot)) => println!("spot: {spot:?}"),
        ClientEvent::UserLine(ServerLine::Message(msg)) => println!("message: {msg}"),
        ClientEvent::PeerFrame(frame) => println!("peer: {frame:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_spot_with_comment() {
        let cli = Cli::parse_from([
            "dxcluster-cli",
            "--addr",
            "cluster.example:7300",
            "spot",
            "n0call",
            "14074",
            "ft8",
            "cq",
        ]);

        match cli.command {
            Commands::Spot(args) => {
                assert_eq!(args.dx, "n0call");
                assert_eq!(args.frequency, "14074");
                assert_eq!(args.comment, vec!["ft8", "cq"]);
            }
            other => panic!("expected spot args, got {other:?}"),
        }
    }

    #[test]
    fn spot_args_convert_to_command() {
        let args = SpotArgs {
            dx: String::from("n0call"),
            frequency: String::from("14074"),
            comment: vec![String::from("ft8"), String::from("cq")],
        };

        let command = args.into_command().expect("conversion succeeded");

        assert!(matches!(
            command,
            UserCommand::Dx {
                frequency,
                comment,
                ..
            } if frequency.to_khz_string() == "14074" && comment == "ft8 cq"
        ));
    }

    #[test]
    fn build_options_parses_callsign() {
        let cli = Cli::parse_from([
            "dxcluster-cli",
            "--addr",
            "127.0.0.1:7300",
            "--callsign",
            "n0call",
            "login",
        ]);

        let options = build_options(&cli).expect("options parsed");
        assert_eq!(options.callsign.unwrap().to_string(), "N0CALL");
    }
}
