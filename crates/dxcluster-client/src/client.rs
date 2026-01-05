use dxcluster_wire::{PeerFrame, ServerLine, UserCommand};

#[derive(Debug)]
pub enum ClientEvent {
    UserLine(ServerLine),
    PeerFrame(PeerFrame),
}

#[derive(Debug, Clone)]
pub struct ClientHandle;

impl ClientHandle {
    pub async fn send_command(&self, _cmd: UserCommand) -> Result<(), crate::error::ClientError> {
        Ok(())
    }

    pub async fn send_frame(&self, _frame: PeerFrame) -> Result<(), crate::error::ClientError> {
        Ok(())
    }
}
