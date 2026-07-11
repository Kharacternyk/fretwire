use std::io;

#[derive(Debug)]
pub enum Error {
    ReadFailed(io::Error),
    WriteFailed(io::Error),
    DeletionForbidden { line: String },
    ExternalWriteForbidden { line: String, path: String },
}
