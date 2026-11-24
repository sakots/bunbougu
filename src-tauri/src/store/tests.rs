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
