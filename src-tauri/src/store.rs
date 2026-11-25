use std::{
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
};

pub(super) type Notes = BTreeMap<String, String>;

const FOLDER_SETTING: &str = "storage-folder.json";

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
    let notes = load(&folder.join("notes.json"))?;
    tempfile::NamedTempFile::new_in(folder)?.close()?;
    write_json(&default.join(FOLDER_SETTING), folder)?;
    Ok(notes)
}

pub(super) fn load(path: &Path) -> io::Result<Notes> {
    match fs::File::open(path) {
        Ok(file) => Ok(serde_json::from_reader(io::BufReader::new(file))?),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Notes::new()),
        Err(error) => Err(error),
    }
}

pub(super) fn save(path: &Path, title: &str, body: &str) -> io::Result<()> {
    let title = title.trim();
    if title.is_empty() || title.contains(['[', ']', '\r', '\n', '\u{2028}', '\u{2029}']) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "無効なページ名"));
    }

    let mut notes = load(path)?;
    notes.insert(title.to_owned(), body.to_owned());
    write_json(path, &notes)
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
