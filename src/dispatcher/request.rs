use std::path::PathBuf;

pub enum Request {
    List,
    Add { id: String, path: PathBuf },
    Find { pinpoint: String, key: String, auto: bool },
    Delete { id: String },
    Clear { force: bool },
    To { path: String },
}
