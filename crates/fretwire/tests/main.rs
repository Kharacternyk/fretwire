use fretwire::{Error::FormatInPlaceFailed, run};
use fretwire_format::{
    Error::{DeletionForbidden, ExternalWriteForbidden},
    MovePolicy,
};
use fretwire_format_in_place::Error::{FormatFailed, LockFailed};
use std::{
    borrow::Cow::Borrowed,
    env::set_current_dir,
    fs::{File, read, write},
    path::PathBuf,
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
    let path = PathBuf::from("test.few");

    write(&path, &content).unwrap();

    let locale = "".parse().unwrap();
    let mut move_policy = MovePolicy {
        marker: ":>",
        allow_external_writes: false,
        allow_deletions: false,
    };
    let one_paragraph = false;
    let timestamp_marker = "";
    let timestamp_pattern = "";
    let skip_disk_sync = true;

    match run(
        Some(&path),
        &locale,
        move_policy,
        one_paragraph,
        timestamp_marker,
        timestamp_pattern,
        skip_disk_sync,
    ) {
        Err(FormatInPlaceFailed(FormatFailed(DeletionForbidden { .. }), new_path))
            if new_path == Borrowed(&path) => {}
        _ => panic!(),
    }

    assert_eq!(read(&path).unwrap(), content.as_bytes());

    move_policy.allow_deletions = true;

    match run(
        Some(&path),
        &locale,
        move_policy,
        one_paragraph,
        timestamp_marker,
        timestamp_pattern,
        skip_disk_sync,
    ) {
        Err(FormatInPlaceFailed(FormatFailed(ExternalWriteForbidden { .. }), new_path))
            if new_path == Borrowed(&path) => {}
        _ => panic!(),
    }

    assert_eq!(read(&path).unwrap(), content.as_bytes());

    move_policy.allow_external_writes = true;

    let moved_path = "moved.few";
    let moved_file = File::create(moved_path).unwrap();

    moved_file.try_lock().unwrap();

    match run(
        Some(&path),
        &locale,
        move_policy,
        one_paragraph,
        timestamp_marker,
        timestamp_pattern,
        skip_disk_sync,
    ) {
        Err(FormatInPlaceFailed(LockFailed, new_path))
            if new_path.to_str() == Some(moved_path) => {}
        _ => panic!(),
    }

    assert_eq!(read(&path).unwrap(), content.as_bytes());

    drop(moved_file);

    assert!(
        run(
            Some(&path),
            &locale,
            move_policy,
            one_paragraph,
            timestamp_marker,
            timestamp_pattern,
            skip_disk_sync,
        )
        .is_ok()
    );

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

    assert_eq!(read(path).unwrap(), content.as_bytes());

    let content = ["Move", "Spans", ""].join("\n");

    assert_eq!(read(moved_path).unwrap(), content.as_bytes());
}
