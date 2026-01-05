use dxcluster_model::Spot;

use crate::user;

pub fn spot_user_line(spot: &Spot) -> String {
    format!(
        "DX de {0}: {1} {2} {3}",
        spot.spotter.as_str(),
        spot.freq.0,
        spot.dx.as_str(),
        spot.comment
    )
}

pub fn banner(node_name: &str) -> String {
    user::format_banner(node_name)
}
