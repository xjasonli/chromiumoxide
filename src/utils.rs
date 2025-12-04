use std::path::{Path, PathBuf};
use crate::js::IntoJsArgs;

/// Write to file with configured runtime
pub(crate) async fn write<P: AsRef<Path> + Unpin, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> std::io::Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "async-std-runtime")] {
            async_std::fs::write(path.as_ref(), contents.as_ref()).await
        } else if #[cfg(feature = "tokio-runtime")] {
            tokio::fs::write(path.as_ref(), contents.as_ref()).await
        }
    }
}

/// Canonicalize path
///
/// Chromium sandboxing does not support Window UNC paths which are used by Rust
/// when the path is relative. See <https://bugs.chromium.org/p/chromium/issues/detail?id=1415018>.
pub(crate) async fn canonicalize<P: AsRef<Path> + Unpin>(path: P) -> std::io::Result<PathBuf> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "async-std-runtime")] {
            let path: PathBuf = async_std::fs::canonicalize(path.as_ref()).await?.into();
        } else if #[cfg(feature = "tokio-runtime")] {
            let path = tokio::fs::canonicalize(path.as_ref()).await?;
        }
    }
    Ok(dunce::simplified(&path).to_path_buf())
}

/// Absolute path
///
pub(crate) fn absolute(path: PathBuf) -> std::io::Result<PathBuf> {
    let path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };
    Ok(dunce::simplified(&path).to_path_buf())
}

/// Canonicalize path except if target binary is snap, in this case only make the path absolute
///
pub(crate) async fn canonicalize_except_snap(path: PathBuf) -> std::io::Result<PathBuf> {
    // Canonalize paths to reduce issues with sandboxing
    let executable_cleaned: PathBuf = canonicalize(&path).await?;

    // Handle case where executable is provided by snap, ignore canonicalize result and only make path absolute
    Ok(if executable_cleaned.to_str().unwrap().ends_with("/snap") {
        absolute(path).unwrap()
    } else {
        executable_cleaned
    })
}

pub(crate) mod base64 {
    use base64::engine::general_purpose::STANDARD;
    use base64::{DecodeError, Engine};

    /// Decode base64 using the standard alphabet and padding
    pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
        STANDARD.decode(input)
    }
}

/// Creates a javascript function string as `(<function>)("<param 1>", "<param
/// 2>")`
pub fn evaluation_string<'a, Args: IntoJsArgs<'a>>(
    function: impl AsRef<str>,
    params: Args
) -> Result<String, serde_json::Error> {
    let values = params.into_vec();
    //let values = params.into_json_values()?;
    let params = values
        .iter()
        .map(|s| serde_json::to_string(s))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(format!("({})({})", function.as_ref(), params.join(", ")))
}
