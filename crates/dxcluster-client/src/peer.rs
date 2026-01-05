use dxcluster_types::NodeId;
use dxcluster_wire::PeerFrame;

use crate::client::{ClientEvent, ClientHandle};
use crate::error::ClientError;

#[derive(Debug, Clone)]
pub struct PeerOptions {
    pub node_id: NodeId,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct PeerClient {
    handle: ClientHandle,
    options: PeerOptions,
}

impl PeerClient {
    pub async fn connect(
        _addr: impl AsRef<str>,
        options: PeerOptions,
    ) -> Result<Self, ClientError> {
        Ok(PeerClient {
            handle: ClientHandle,
            options,
        })
    }

    pub async fn next_event(&mut self) -> Option<Result<ClientEvent, ClientError>> {
        let frame = PeerFrame::Pong {
            nonce: String::from("0"),
        };
        Some(Ok(ClientEvent::PeerFrame(frame)))
    }

    pub async fn send_frame(&self, frame: PeerFrame) -> Result<(), ClientError> {
        self.handle.send_frame(frame).await
    }

    pub fn options(&self) -> &PeerOptions {
        &self.options
    }
}
