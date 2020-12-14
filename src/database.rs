#[derive(Debug, Clone, Error, Serialize, Deserialize, PartialEq, Eq)]
#[error(display = "failed to load database")]
pub enum LoadError {
    #[error(source)]
    IOError(std::io::Error),

    IncorrectKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub organization: String,
    pub root: PathBuf,
    pub boards: Vec<PathBuf>,
}

impl Database {
    pub fn try_load(filename: &str) -> Result<Self, LoadError> {

    }
}