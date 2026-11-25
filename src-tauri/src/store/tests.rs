use super::*;

#[test]
fn reopening_reads_saved_body_and_preserves_other_pages() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("notes.json");
    save(&path, "  日記  ", "[[予定]]\nこんにちは").unwrap();
    save(&path, "予定", "明日").unwrap();
    save(&path, "予定", "明後日").unwrap();

    let notes = load(&path).unwrap();
    assert_eq!(notes.len(), 2);
    assert_eq!(notes["日記"], "[[予定]]\nこんにちは");
    assert_eq!(notes["予定"], "明後日");
}

#[test]
fn corrupt_storage_is_not_overwritten() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("notes.json");
    fs::write(&path, b"invalid JSON").unwrap();

    assert!(save(&path, "日記", "新しい本文").is_err());
    assert_eq!(fs::read(&path).unwrap(), b"invalid JSON");
}

#[test]
fn switching_folders_keeps_notes_separate_and_remembers_the_selection() {
    let default = tempfile::tempdir().unwrap();
    let selected = tempfile::tempdir().unwrap();
    save(&default.path().join("notes.json"), "元のノート", "元の本文").unwrap();
    save(&selected.path().join("notes.json"), "別のノート", "別の本文").unwrap();

    assert_eq!(select_folder(default.path(), selected.path()).unwrap()["別のノート"], "別の本文");
    assert_eq!(active_folder(default.path()).unwrap(), selected.path());
    assert_eq!(load(&default.path().join("notes.json")).unwrap()["元のノート"], "元の本文");
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
