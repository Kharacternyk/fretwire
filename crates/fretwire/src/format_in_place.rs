use crate::{
    Error::{self, FormatFailed, IOFailed, LockFailed},
    IntoIOFailed,
};
use core::cmp::max;
use fretwire_format::{MovePolicy, format};
use fretwire_locale::Locale;
use positioned_io::{RandomAccessFile, Size, SizeCursor, Slice, WriteAt};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions, TryLockError},
    io::{self, BufReader, BufWriter, Write, copy},
    path::PathBuf,
};

const STAGE_ONE_MARKER: &str = "\n\nFRETWIRE STAGE ONE\n\n";
const STAGE_TWO_MARKER: &str = "\n\nFRETWIRE STAGE TWO\n\n";

pub struct FormatInPlace {
    file: File,
    original_size: u64,
    formatted_size: u64,
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
            TryLockError::WouldBlock => LockFailed(path.into()),
        })?;

        let original_size = file.size().path(path)?.path(path)?;
        let file = RandomAccessFile::try_new(file).path(path)?;

        let sink_capacity = max(1 << 13, STAGE_ONE_MARKER.len() * 2);

        let mut source =
            BufReader::new(SizeCursor::new(Slice::new(&file, 0, Some(original_size))));
        let mut sink = BufWriter::with_capacity(
            sink_capacity,
            SizeCursor::new(Slice::new(&file, original_size, None)),
        );

        sink.write_all(STAGE_ONE_MARKER.as_bytes()).expect(
            "BufWriter should not do any IO while there is free space in the buffer",
        );

        let format_result =
            format(&mut source, &mut sink, locale, move_policy, prepend_lines);

        drop(source);

        if format_result.is_err() {
            let _ = sink.into_parts();
        } else {
            drop(sink);
        }

        match (format_result, file.try_into_inner()) {
            (Err(error), file) => {
                if let Ok(file) = file {
                    let _ = file.set_len(original_size);
                }

                Err(FormatFailed {
                    error,
                    path: Some(path.into()),
                })
            }
            (Ok(_), Err((_, error))) => Err(IOFailed {
                error: Some(error),
                path: path.into(),
            }),
            (Ok((formatted_size, lines_to_move)), Ok(file)) => Ok((
                Self {
                    file,
                    original_size,
                    formatted_size,
                },
                lines_to_move,
            )),
        }
    }

    pub fn commit(mut self, skip_disk_sync: bool) -> Result<(), io::Error> {
        if let error @ Err(_) = self.init_stage_two(skip_disk_sync) {
            let _ = self.rollback();

            error
        } else {
            let file = RandomAccessFile::try_new(self.file)?;

            let mut source = BufReader::new(SizeCursor::new(Slice::new(
                &file,
                self.original_size + STAGE_ONE_MARKER.len() as u64,
                Some(self.formatted_size),
            )));
            let mut sink = BufWriter::new(SizeCursor::new(&file));

            let size = copy(&mut source, &mut sink)?;

            drop(source);
            sink.into_inner()?;

            match file.try_into_inner() {
                Ok(file) => {
                    file.set_len(size)?;
                    Ok(())
                }
                Err((_, error)) => Err(error),
            }
        }
    }

    fn init_stage_two(&mut self, skip_disk_sync: bool) -> Result<(), io::Error> {
        self.file.write_all_at(
            self.original_size + STAGE_ONE_MARKER.len() as u64 + self.formatted_size,
            STAGE_TWO_MARKER.as_bytes(),
        )?;

        if !skip_disk_sync {
            self.file.sync_data()?;
        }

        Ok(())
    }

    pub fn rollback(self) -> Result<(), io::Error> {
        self.file.set_len(self.original_size)
    }
}
