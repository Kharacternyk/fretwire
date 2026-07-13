use crate::{
    Error::{self, FormatFailed},
    IntoIOFailed,
};
use fretwire_format::{MovePolicy, format};
use fretwire_locale::Locale;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{
        self, BufReader, BufWriter, Read, Seek,
        SeekFrom::{End, Start},
        Write, copy,
    },
    path::PathBuf,
};

const PROGRESS_MARKER: &str = "\n\nFRETWIRE IN PROGRESS\n\n";

pub struct FormattedFile {
    original_length: u64,
    source: File,
    sink: File,
    lines_to_move: HashMap<String, Vec<String>>,
    skip_disk_sync: bool,
}

impl FormattedFile {
    pub fn try_new(
        path: &PathBuf,
        locale: &Locale,
        move_policy: MovePolicy,
        skip_disk_sync: bool,
        prepend_lines: impl IntoIterator<Item = String>,
        allow_creation: bool,
    ) -> Result<Self, Error> {
        let source = OpenOptions::new()
            .read(true)
            .write(true)
            .create(allow_creation)
            .open(path)
            .path(path)?;
        let mut sink = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .path(path)?;
        let original_length = sink.seek(End(0)).path(path)?;

        let mut buf_source = BufReader::new(&source).take(original_length);
        let mut buf_sink = BufWriter::new(&sink);

        buf_sink.write_all(PROGRESS_MARKER.as_bytes()).path(path)?;

        let lines_to_move = format(
            &mut buf_source,
            &mut buf_sink,
            locale,
            move_policy,
            prepend_lines,
        )
        .map_err(|error| {
            let _ = buf_sink.into_parts();
            let _ = source.set_len(original_length);

            FormatFailed {
                error,
                path: Some(path.into()),
            }
        })?;

        Ok(Self {
            original_length,
            source,
            sink,
            lines_to_move,
            skip_disk_sync,
        })
    }

    pub fn lines_to_move(&self) -> &HashMap<String, Vec<String>> {
        &self.lines_to_move
    }

    pub fn commit(&mut self) -> Result<(), io::Error> {
        self.sink
            .seek(Start(self.original_length + (PROGRESS_MARKER.len() as u64)))?;
        self.source.seek(Start(0))?;

        if !self.skip_disk_sync {
            self.sink.sync_all().inspect_err(|_| {
                self.rollback();
            })?;
        }

        let size = copy(&mut self.sink, &mut self.source)?;

        self.source.set_len(size)
    }

    pub fn rollback(&mut self) -> Result<(), io::Error> {
        self.source.set_len(self.original_length)
    }
}
