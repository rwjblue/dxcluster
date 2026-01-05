use dxcluster_wire::PeerFrame;

pub struct PeerSession;

impl PeerSession {
    pub async fn handle(&self, _frame: PeerFrame) {}
}
