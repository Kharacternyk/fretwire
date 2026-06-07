use std::io;

#[derive(Debug)]
pub enum Error {
    ReadFailed(io::Error),
    WriteFailed(io::Error),
}
