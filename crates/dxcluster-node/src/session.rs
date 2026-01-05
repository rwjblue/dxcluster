use std::io;

use dxcluster_model::{Filter, Spot};
use dxcluster_types::{Callsign, SpotId};
use dxcluster_wire::format::{banner as format_banner, spot_user_line};
use dxcluster_wire::{ServerLine, UserCommand};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

use crate::state::NodeState;

/// Default callsign applied to anonymous users until proper login flows
/// are implemented.
///
/// The value must be valid even when the strict callsign feature is enabled
/// (requires at least one digit), so we include a zero.
const DEFAULT_CALLSIGN: &str = "N0CALL";

/// Telnet-style session for a single user connection.
///
/// The session owns the TCP stream, reads user commands framed according to
/// [`dxcluster_wire`] parsing rules, mutates shared [`NodeState`], and
/// responds with formatted server lines. It keeps track of per-user filters
/// and callsigns for spot attribution.
pub struct UserSession<T> {
    stream: T,
    state: NodeState,
    filter: Filter,
    callsign: Callsign,
}

impl<T> UserSession<T>
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    /// Construct a new session wrapping a TCP stream and shared node state.
    pub fn new(stream: T, state: NodeState) -> Self {
        let callsign =
            Callsign::parse_loose(DEFAULT_CALLSIGN).expect("default callsign should be valid");
        Self {
            stream,
            state,
            filter: Filter,
            callsign,
        }
    }

    /// Run the session loop until the client disconnects or an IO error is
    /// encountered.
    pub async fn run(self) -> io::Result<()> {
        let UserSession {
            stream,
            state,
            mut filter,
            callsign,
        } = self;

        let (reader, mut writer) = tokio::io::split(stream);
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        write_line(
            &mut writer,
            ServerLine::Banner(format_banner(state.node_id().0.as_str())),
        )
        .await?;
        write_line(&mut writer, ServerLine::Prompt).await?;

        loop {
            line.clear();
            let read = reader.read_line(&mut line).await?;
            if read == 0 {
                break;
            }

            match dxcluster_wire::parse::parse_line(&line) {
                Ok(cmd) => {
                    let responses = handle_command(&state, &mut filter, &callsign, cmd).await;
                    for response in responses {
                        write_line(&mut writer, response).await?;
                    }
                }
                Err(err) => {
                    write_line(&mut writer, ServerLine::Message(format!("ERR: {err}"))).await?;
                }
            }

            write_line(&mut writer, ServerLine::Prompt).await?;
        }

        Ok(())
    }
}

async fn handle_command(
    state: &NodeState,
    filter: &mut Filter,
    callsign: &Callsign,
    cmd: UserCommand,
) -> Vec<ServerLine> {
    match cmd {
        UserCommand::Dx {
            dx,
            frequency,
            comment,
        } => {
            let ts = time::OffsetDateTime::now_utc();
            let spot_id = SpotId::hash_components(&[
                dx.as_str().as_bytes(),
                &frequency.0.to_be_bytes(),
                &ts.unix_timestamp().to_be_bytes(),
            ]);
            let origin = Some(state.node_id().clone());
            let spot = Spot::new_local(
                spot_id,
                ts,
                frequency,
                dx,
                callsign.clone(),
                comment,
                origin,
            );
            state.insert(spot.clone()).await;
            vec![ServerLine::Spot(spot)]
        }
        UserCommand::Show(show) => match show {
            dxcluster_wire::user::ShowCommand::Dx => state
                .recent(10)
                .await
                .into_iter()
                .filter(|spot| filter.matches(spot))
                .map(ServerLine::Spot)
                .collect(),
            dxcluster_wire::user::ShowCommand::Filters => {
                vec![ServerLine::Message(
                    "Filters: accepting all spots".to_string(),
                )]
            }
        },
        UserCommand::Heartbeat => vec![ServerLine::Message("PONG".into())],
        UserCommand::Raw(raw) => vec![ServerLine::Message(format!("Unknown command: {raw}"))],
    }
}

async fn write_line<W: AsyncWrite + Unpin>(writer: &mut W, line: ServerLine) -> io::Result<()> {
    let rendered = match line {
        ServerLine::Banner(text) => text,
        ServerLine::Prompt => dxcluster_wire::user::format_prompt(),
        ServerLine::Spot(spot) => spot_user_line(&spot),
        ServerLine::Message(text) => text,
    };

    writer.write_all(rendered.as_bytes()).await?;
    writer.write_all(b"\n").await
}
