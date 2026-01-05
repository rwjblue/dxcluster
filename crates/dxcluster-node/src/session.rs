use dxcluster_wire::{ServerLine, UserCommand};

pub struct Session;

impl Session {
    pub async fn handle(&self, _cmd: UserCommand) -> Option<ServerLine> {
        None
    }
}
