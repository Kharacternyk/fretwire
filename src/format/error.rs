use std::io;

pub enum Error {
    ReadFailed(io::Error),
    WriteFailed(io::Error),
}
