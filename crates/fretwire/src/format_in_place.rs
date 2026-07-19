use crate::{
    Error::{self, FormatFailed, IOFailed, LockFailed},
    IntoIOFailed,
};
use core::cmp::max;
use fretwire_format::{MovePolicy, format};
use fretwire_locale::Locale;
use positioned_io::{Size, SizeCursor};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions, TryLockError},
    io::{self, BufReader, BufWriter, Read, Seek, SeekFrom::Start, Write, copy},
    path::PathBuf,
};

const STAGE_ONE_MARKER: &str = "\n\nFRETWIRE STAGE ONE\n\n";
const STAGE_TWO_MARKER: &str = "\n\nFRETWIRE STAGE TWO\n\n";

pub struct FormatInPlace {
    file: File,
    original_size: u64,
    stage_one_size: u64,
}

impl FormatInPlace {
    pub fn try_new(
        path: &PathBuf,
        locale: &Locale,
        move_policy: MovePolicy,
        prepend_lines: impl IntoIterator<Item = String>,
        allow_creation: bool,
    ) -> Result<(Self, HashMap<PathBuf, Vec<String>>), Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(allow_creation)
            .open(path)
            .path(path)?;

        file.try_lock().map_err(|error| match error {
            TryLockError::Error(error) => IOFailed {
                error: Some(error),
                path: path.into(),
            },
            _ => LockFailed(path.into()),
        })?;

        let original_size = file.size().path(path)?.path(path)?;
        let sink_capacity = max(1 << 13, STAGE_ONE_MARKER.len() * 2);

        let mut source = BufReader::new((&file).take(original_size));
        let mut sink = BufWriter::with_capacity(
            sink_capacity,
            SizeCursor::new_pos(file.try_clone().path(path)?, original_size),
        );

        sink.write_all(STAGE_ONE_MARKER.as_bytes()).expect(
            "BufWriter should not do any IO while there is free space in the buffer",
        );

        match format(&mut source, &mut sink, locale, move_policy, prepend_lines) {
            Err(error) => {
                let _ = sink.into_parts();
                let _ = file.set_len(original_size);

                Err(FormatFailed {
                    error,
                    path: Some(path.into()),
                })
            }
            Ok(lines_to_move) => Ok((
                Self {
                    file,
                    original_size,
                    stage_one_size: sink.get_ref().position(),
                },
                lines_to_move,
            )),
        }
    }

    pub fn commit(mut self, skip_disk_sync: bool) -> Result<Self, io::Error> {
        match self.clone_for_stage_two(skip_disk_sync) {
            Err(error) => {
                let _ = self.rollback();

                Err(error)
            }
            Ok(file) => {
                let mut source = BufReader::new(
                    (&self.file).take(self.stage_one_size - self.original_size),
                );
                let mut sink = BufWriter::new(SizeCursor::new(file));
                let size = copy(&mut source, &mut sink)?;

                self.file.set_len(size)?;

                Ok(self)
            }
        }
    }

    fn clone_for_stage_two(&mut self, skip_disk_sync: bool) -> Result<File, io::Error> {
        self.file.write_all(STAGE_TWO_MARKER.as_bytes())?;

        if !skip_disk_sync {
            self.file.sync_all()?;
        }

        self.file.seek(Start(self.original_size))?;
        self.file.try_clone()
    }

    pub fn rollback(self) -> Result<(), io::Error> {
        self.file.set_len(self.original_size)
    }
}
