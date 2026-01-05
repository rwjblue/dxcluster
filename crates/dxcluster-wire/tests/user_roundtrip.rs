use dxcluster_types::{Callsign, FrequencyHz};
use dxcluster_wire::user::{ShowCommand, UserCommand, format_command, parse_line};

#[test]
fn dx_command_roundtrips() {
    let parsed = parse_line("DX K1ABC 14.070 PSK31 fun").expect("should parse DX line");
    let expected = UserCommand::Dx {
        dx: Callsign::parse_loose("K1ABC").unwrap(),
        frequency: FrequencyHz::from_khz_str("14.070").unwrap(),
        comment: String::from("PSK31 fun"),
    };

    assert_eq!(parsed, expected);

    let formatted = format_command(&parsed);
    let reparsed = parse_line(&formatted).expect("format should produce parseable command");
    assert_eq!(reparsed, expected);
}

#[test]
fn show_filters_roundtrips() {
    let parsed = parse_line("sh/filters").expect("case-insensitive");
    assert_eq!(parsed, UserCommand::Show(ShowCommand::Filters));

    let formatted = format_command(&parsed);
    let reparsed = parse_line(&formatted).expect("format should produce parseable command");
    assert_eq!(reparsed, parsed);
}

#[test]
fn heartbeat_roundtrips() {
    let parsed = parse_line("ping").expect("heartbeat parses");
    assert_eq!(parsed, UserCommand::Heartbeat);

    let formatted = format_command(&parsed);
    let reparsed = parse_line(&formatted).expect("format should produce parseable command");
    assert_eq!(reparsed, parsed);
}
