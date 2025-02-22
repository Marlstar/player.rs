use std::path::PathBuf;
use std::sync::OnceLock;

pub static DIRS: OnceLock<Dirs> = OnceLock::new();
pub fn dirs() -> &'static Dirs {
    return DIRS.get_or_init(|| Dirs::new());
}

pub struct Dirs {
    dirs: directories::BaseDirs
}
impl Default for Dirs {
    fn default() -> Self {
        Self {
            dirs: directories::BaseDirs::new().unwrap()
        }
    }
}
impl Dirs {
    pub fn new() -> Self {
        Default::default()
    }
}
impl Dirs {
    pub fn base(&self) -> PathBuf {
        return self.dirs.data_dir().to_owned().join("player");
    }

    pub fn audio_files(&self) -> PathBuf {
        return self.base().join("audio");
    }
    pub fn audio_file(&self, name: &str) -> PathBuf {
        return self.audio_files().join(format!("{name}.mp3"));
    }
}
