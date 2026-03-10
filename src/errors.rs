use std::path::PathBuf;

/// All recoverable errors that md-prune can produce.
///
/// Uses `thiserror` derive macro to auto-generate `Display` and `Error`
/// implementations. Each variant's `#[error("...")]` attribute defines
/// the human-readable message.
#[derive(Debug, thiserror::Error)]
pub enum MdPruneError {
    /// Input file does not exist.
    #[error("file not found: {path}")]
    FileNotFound { path: PathBuf },

    /// I/O error while reading the input file.
    #[error("failed to read {path}")]
    ReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// I/O error while writing the output file.
    #[error("failed to write {path}")]
    WriteError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_not_found_displays_path() {
        let err = MdPruneError::FileNotFound {
            path: "/tmp/missing.md".into(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("/tmp/missing.md"), "Error should contain path");
    }

    #[test]
    fn write_error_displays_path() {
        let err = MdPruneError::WriteError {
            path: "/tmp/out.md".into(),
            source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("/tmp/out.md"));
    }

    #[test]
    fn error_is_send_sync() {
        // Rust note: Send + Sync means the error can be shared across threads.
        // anyhow::Error requires this. If this compiles, the trait bounds hold.
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MdPruneError>();
    }
}
