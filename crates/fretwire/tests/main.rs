use fretwire::{Error::ExternalWriteForbidden, Settings, run_with_settings};
use std::{
    borrow::Cow::Borrowed,
    env::set_current_dir,
    fs::{read, write},
};

#[test]
fn main() {
    set_current_dir(env!("CARGO_TARGET_TMPDIR")).unwrap();

    let content = [
        "a",
        "Long",
        "Sentence:>",
        "that ",
        "Spans:>./moved.few",
        "",
        "Multiple",
        "paragraphs\t",
        "",
        "and",
        "has ",
        "move :> moved.few ",
        "Markers",
        "in",
        "it:>",
        "",
    ];
    let content = content.join("\n");

    write("test.few", &content).unwrap();

    let mut settings = Settings {
        file: Some("test.few".into()),
        move_marker: Borrowed(":>"),
        allow_external_writes: false,
        skip_disk_sync: true,
        locale: "".parse().unwrap(),
    };

    assert!(matches!(
        run_with_settings(&settings),
        Err(ExternalWriteForbidden { .. }),
    ));
    assert_eq!(read("test.few").unwrap(), content.as_bytes());

    settings.allow_external_writes = true;

    assert!(run_with_settings(&settings).is_ok());

    let content = [
        "a",
        "long",
        "that",
        "",
        "Multiple",
        "Paragraphs",
        "",
        "and",
        "has",
        "in",
        "markers",
        "",
    ];
    let content = content.join("\n");

    assert_eq!(read("test.few").unwrap(), content.as_bytes());

    let content = ["Move", "Spans", ""].join("\n");

    assert_eq!(read("moved.few").unwrap(), content.as_bytes());
}
