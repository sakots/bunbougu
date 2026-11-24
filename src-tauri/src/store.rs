use std::{collections::BTreeMap, fs, io, path::Path};

pub(super) type Notes = BTreeMap<String, String>;

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
    let dir = path.parent().expect("storage path has a parent directory");
    fs::create_dir_all(dir)?;
    let file = tempfile::NamedTempFile::new_in(dir)?;
    let mut writer = io::BufWriter::new(file.as_file());
    serde_json::to_writer(&mut writer, &notes)?;
    io::Write::flush(&mut writer)?;
    drop(writer);
    file.as_file().sync_all()?;
    file.persist(path)?;
    Ok(())
}

#[cfg(test)]
mod tests;
