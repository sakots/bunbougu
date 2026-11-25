use super::*;

#[test]
fn reopening_reads_saved_body_and_preserves_other_pages() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    save(path, "  日記  ", "[[予定]]\nこんにちは").unwrap();
    save(path, "予定", "明日").unwrap();
    let diary = fs::read_dir(path.join("pages"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|file| fs::read_to_string(file).unwrap().contains("こんにちは"))
        .unwrap();
    let original = fs::read(&diary).unwrap();
    save(path, "予定", "明後日").unwrap();

    assert_eq!(fs::read_dir(path.join("pages")).unwrap().count(), 2);
    assert_eq!(fs::read(diary).unwrap(), original);
    let notes = load(path).unwrap();
    assert_eq!(notes.len(), 2);
    assert_eq!(notes["日記"], "[[予定]]\nこんにちは");
    assert_eq!(notes["予定"], "明後日");
}

#[test]
fn corrupt_page_is_not_overwritten() {
    let dir = tempfile::tempdir().unwrap();
    save(dir.path(), "日記", "本文").unwrap();
    let file = fs::read_dir(dir.path().join("pages"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::write(&file, b"invalid JSON").unwrap();

    assert!(save(dir.path(), "日記", "新しい本文").is_err());
    assert_eq!(fs::read(file).unwrap(), b"invalid JSON");
}

#[test]
fn switching_folders_keeps_notes_separate_and_remembers_the_selection() {
    let default = tempfile::tempdir().unwrap();
    let selected = tempfile::tempdir().unwrap();
    save(default.path(), "元のノート", "元の本文").unwrap();
    save(selected.path(), "別のノート", "別の本文").unwrap();

    assert_eq!(
        select_folder(default.path(), selected.path()).unwrap()["別のノート"],
        "別の本文"
    );
    assert_eq!(active_folder(default.path()).unwrap(), selected.path());
    assert_eq!(load(default.path()).unwrap()["元のノート"], "元の本文");
}

#[test]
fn invalid_notes_do_not_replace_the_current_folder_setting() {
    let default = tempfile::tempdir().unwrap();
    let selected = tempfile::tempdir().unwrap();
    let broken = tempfile::tempdir().unwrap();
    select_folder(default.path(), selected.path()).unwrap();
    fs::write(broken.path().join("notes.json"), b"invalid JSON").unwrap();

    assert!(select_folder(default.path(), broken.path()).is_err());
    assert_eq!(active_folder(default.path()).unwrap(), selected.path());
}

#[test]
fn legacy_notes_are_converted_once_without_changing_the_backup() {
    let dir = tempfile::tempdir().unwrap();
    let folder = dir.path();
    let legacy = br#"{"CON":"reserved name","con":"case variant","../other":"literal title"}"#;
    fs::write(folder.join("notes.json"), legacy).unwrap();

    let notes = load(folder).unwrap();
    assert_eq!(notes["CON"], "reserved name");
    assert_eq!(notes["con"], "case variant");
    assert_eq!(notes["../other"], "literal title");
    assert_eq!(fs::read_dir(folder.join("pages")).unwrap().count(), 3);
    save(folder, "CON", "new body").unwrap();
    assert_eq!(load(folder).unwrap()["CON"], "new body");
    assert_eq!(fs::read(folder.join("notes.json")).unwrap(), legacy);
}

#[test]
fn deletion_removes_only_the_named_page_and_does_not_restore_the_backup() {
    let dir = tempfile::tempdir().unwrap();
    let folder = dir.path();
    fs::write(folder.join("notes.json"), br#"{"Plan":"old body"}"#).unwrap();
    load(folder).unwrap();
    save(folder, "plan", "keep").unwrap();

    delete(folder, "Plan").unwrap();
    assert_eq!(load(folder).unwrap(), Notes::from([("plan".into(), "keep".into())]));
    delete(folder, "plan").unwrap();
    assert!(load(folder).unwrap().is_empty());
    assert_eq!(fs::read_dir(folder.join("pages")).unwrap().count(), 0);
}
