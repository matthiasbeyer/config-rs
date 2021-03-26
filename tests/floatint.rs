extern crate config;
use config::*;

fn make() -> Config {
    Config::default()
        .merge(File::from_str(
            r#"{"floatval": "9223372036854775807"}"#,
            FileFormat::Json,
        ))
        .unwrap()
        .clone()
}

#[test]
fn test_invalid_float() {
    let c = make();
    let f = c.get_float("floatval").unwrap();
    assert_eq!(format!("{}", f), format!("{}", 9223372036854775807 as i64));
}

