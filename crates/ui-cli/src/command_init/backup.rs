use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// RAII backup guard. Copies `path` to `path.bak` on construction.
///
/// If the guard is dropped while still armed (i.e. the operation failed),
/// the original file is restored from the backup. Call `disarm()` on success
/// to delete the backup and prevent any restore.
pub struct FileBackup {
    original: PathBuf,
    backup: PathBuf,
    armed: bool,
}

impl FileBackup {
    /// Back up `path` to `<path>.bak`.
    /// Returns `None` if the file does not exist (nothing to back up).
    pub fn new(path: &Path) -> io::Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        let backup = PathBuf::from(format!("{}.bak", path.display()));
        fs::copy(path, &backup)?;
        Ok(Some(Self { original: path.to_path_buf(), backup, armed: true }))
    }

    /// Disarm the guard: delete the backup file and prevent restore on drop.
    /// Call this after a successful write.
    pub fn disarm(&mut self) {
        self.armed = false;
        let _ = fs::remove_file(&self.backup);
    }

    /// Path of the backup file (`<original>.bak`).
    #[cfg(test)]
    pub fn backup_path(&self) -> &Path {
        &self.backup
    }
}

impl Drop for FileBackup {
    fn drop(&mut self) {
        if self.armed {
            let _ = fs::copy(&self.backup, &self.original);
            let _ = fs::remove_file(&self.backup);
        }
    }
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write(dir: &TempDir, name: &str, content: &str) -> PathBuf {
        let p = dir.path().join(name);
        fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn returns_none_when_file_does_not_exist() {
        let dir = TempDir::new().unwrap();
        let result = FileBackup::new(&dir.path().join("missing.toml")).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn creates_bak_file_when_original_exists() {
        let dir = TempDir::new().unwrap();
        let original = write(&dir, "config.toml", "original");
        let guard = FileBackup::new(&original).unwrap().unwrap();
        assert!(guard.backup_path().exists());
    }

    #[test]
    fn disarm_deletes_backup_file() {
        let dir = TempDir::new().unwrap();
        let original = write(&dir, "config.toml", "original");
        let mut guard = FileBackup::new(&original).unwrap().unwrap();
        let backup_path = guard.backup_path().to_path_buf();
        guard.disarm();
        assert!(!backup_path.exists());
    }

    #[test]
    fn disarmed_guard_does_not_restore_on_drop() {
        let dir = TempDir::new().unwrap();
        let original = write(&dir, "config.toml", "original");
        let mut guard = FileBackup::new(&original).unwrap().unwrap();
        guard.disarm();
        fs::write(&original, "modified").unwrap();
        drop(guard);
        let content = fs::read_to_string(&original).unwrap();
        assert_eq!(content, "modified");
    }

    #[test]
    fn armed_drop_restores_original_content() {
        let dir = TempDir::new().unwrap();
        let original = write(&dir, "config.toml", "original");
        let guard = FileBackup::new(&original).unwrap().unwrap();
        // Simulate a failed write — overwrite the file then drop (still armed)
        fs::write(&original, "corrupted").unwrap();
        drop(guard);
        let content = fs::read_to_string(&original).unwrap();
        assert_eq!(content, "original");
    }

    #[test]
    fn armed_drop_removes_backup_file_after_restore() {
        let dir = TempDir::new().unwrap();
        let original = write(&dir, "config.toml", "original");
        let guard = FileBackup::new(&original).unwrap().unwrap();
        let backup_path = guard.backup_path().to_path_buf();
        drop(guard);
        assert!(!backup_path.exists());
    }

    #[test]
    fn backup_content_matches_original() {
        let dir = TempDir::new().unwrap();
        let original = write(&dir, "config.toml", "important data");
        let guard = FileBackup::new(&original).unwrap().unwrap();
        let backup_content = fs::read_to_string(guard.backup_path()).unwrap();
        assert_eq!(backup_content, "important data");
    }
}
