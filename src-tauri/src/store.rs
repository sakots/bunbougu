use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
};

pub(super) type Notes = BTreeMap<String, String>;

const FOLDER_SETTING: &str = "storage-folder.json";
const PAGES: &str = "pages";

#[derive(Deserialize, Serialize)]
struct Page<T> {
    title: T,
    body: T,
}

fn page_filename(title: &str) -> String {
    format!("{:x}.json", Sha256::digest(title.as_bytes()))
}

pub(super) fn active_folder(default: &Path) -> io::Result<PathBuf> {
    let file = match fs::File::open(default.join(FOLDER_SETTING)) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(default.to_owned()),
        Err(error) => return Err(error),
    };
    let folder: PathBuf = serde_json::from_reader(io::BufReader::new(file))?;
    check_folder(&folder)?;
    Ok(folder)
}

fn check_folder(folder: &Path) -> io::Result<()> {
    if !folder.is_absolute() || !fs::metadata(folder)?.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "無効な保存フォルダー"));
    }
    Ok(())
}

#[cfg(any(desktop, test))]
pub(super) fn select_folder(default: &Path, folder: &Path) -> io::Result<Notes> {
    check_folder(folder)?;
    let notes = load(folder)?;
    let pages = folder.join(PAGES);
    tempfile::NamedTempFile::new_in(if pages.is_dir() { &pages } else { folder })?.close()?;
    write_json(&default.join(FOLDER_SETTING), folder)?;
    Ok(notes)
}

pub(super) fn load(folder: &Path) -> io::Result<Notes> {
    let entries = match fs::read_dir(pages_dir(folder)?) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Notes::new()),
        Err(error) => return Err(error),
    };
    let mut notes = Notes::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let page: Page<String> =
            serde_json::from_reader(io::BufReader::new(fs::File::open(&path)?))?;
        if entry.file_name() != page_filename(&page.title).as_str() {
            return Err(page_filename_mismatch());
        }
        notes.insert(page.title, page.body);
    }
    Ok(notes)
}

fn page_filename_mismatch() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "ページ名とファイル名が一致しません",
    )
}

fn pages_dir(folder: &Path) -> io::Result<PathBuf> {
    let pages = folder.join(PAGES);
    if !pages.try_exists()? {
        if let Some(legacy) = read_json::<Notes>(&folder.join("notes.json"))? {
            // Publish the converted pages together; keep the original as a backup.
            let staged = tempfile::tempdir_in(folder)?;
            for (title, body) in legacy {
                write_json(
                    &staged.path().join(page_filename(&title)),
                    &Page { title, body },
                )?;
            }
            fs::rename(staged.path(), &pages)?;
        }
    }
    Ok(pages)
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> io::Result<Option<T>> {
    match fs::File::open(path) {
        Ok(file) => Ok(Some(serde_json::from_reader(io::BufReader::new(file))?)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

pub(super) fn save(folder: &Path, title: &str, body: &str) -> io::Result<()> {
    let title = title.trim();
    if title.is_empty() || title.contains(['[', ']', '\r', '\n', '\u{2028}', '\u{2029}']) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "無効なページ名"));
    }

    let path = pages_dir(folder)?.join(page_filename(title));
    if let Some(existing) = read_json::<Page<String>>(&path)? {
        if existing.title != title {
            return Err(page_filename_mismatch());
        }
    }
    write_json(&path, &Page { title, body })
}

pub(super) fn delete(folder: &Path, title: &str) -> io::Result<()> {
    fs::remove_file(folder.join(PAGES).join(page_filename(title)))
}

fn write_json(path: &Path, value: &(impl serde::Serialize + ?Sized)) -> io::Result<()> {
    let dir = path.parent().expect("storage path has a parent directory");
    fs::create_dir_all(dir)?;
    let file = tempfile::NamedTempFile::new_in(dir)?;
    let mut writer = io::BufWriter::new(file.as_file());
    serde_json::to_writer(&mut writer, value)?;
    io::Write::flush(&mut writer)?;
    drop(writer);
    file.as_file().sync_all()?;
    file.persist(path)?;
    Ok(())
}

#[cfg(test)]
mod tests;
