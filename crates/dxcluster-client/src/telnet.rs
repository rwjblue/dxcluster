use dxcluster_types::Callsign;
use dxcluster_wire::{ServerLine, UserCommand};

use crate::client::{ClientEvent, ClientHandle};
use crate::error::ClientError;

#[derive(Debug, Default, Clone)]
pub struct TelnetOptions {
    pub callsign: Option<Callsign>,
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TelnetClient {
    handle: ClientHandle,
}

impl TelnetClient {
    pub async fn connect(
        _addr: impl AsRef<str>,
        _options: TelnetOptions,
    ) -> Result<Self, ClientError> {
        Ok(TelnetClient {
            handle: ClientHandle,
        })
    }

    pub async fn next_event(&mut self) -> Option<Result<ClientEvent, ClientError>> {
        let line = ServerLine::Message(String::from("ok"));
        Some(Ok(ClientEvent::UserLine(line)))
    }

    pub async fn send_command(&self, cmd: UserCommand) -> Result<(), ClientError> {
        self.handle.send_command(cmd).await
    }
}
