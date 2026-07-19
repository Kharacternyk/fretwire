use crate::{
    Error::{self, DeletionForbidden, ExternalWriteForbidden, ReadFailed, WriteFailed},
    MovePolicy, StateMachine,
};
use fretwire_locale::Locale;
use path_clean::clean;
use std::{
    borrow::Cow,
    collections::HashMap,
    io::{BufRead, Write},
    path::PathBuf,
};

pub fn format(
    source: &mut impl BufRead,
    mut sink: &mut impl Write,
    locale: &Locale,
    move_policy: MovePolicy,
    prepend_lines: impl IntoIterator<Item = String>,
) -> Result<HashMap<PathBuf, Vec<String>>, Error> {
    let mut result: HashMap<PathBuf, Vec<String>> = HashMap::new();
    let mut machine = StateMachine::new(locale);

    for line in prepend_lines {
        write(&mut sink, machine.feed(line))?;
    }

    for line in source.lines() {
        let line = line.map_err(ReadFailed)?;

        if let Some(line) = try_move(line, move_policy, &mut result)? {
            write(&mut sink, machine.feed(line))?;
        }
    }

    write(&mut sink, machine.flush())?;

    sink.flush().map_err(WriteFailed)?;

    Ok(result)
}

fn try_move(
    mut line: String,
    move_policy: MovePolicy,
    result: &mut HashMap<PathBuf, Vec<String>>,
) -> Result<Option<String>, Error> {
    if !move_policy.marker.is_empty()
        && let Some((first, second)) = line.split_once(move_policy.marker)
    {
        let path: String = second.trim().into();

        match (path.is_empty(), move_policy) {
            (
                true,
                MovePolicy {
                    allow_deletions: false,
                    ..
                },
            ) => Err(DeletionForbidden { line }),
            (
                false,
                MovePolicy {
                    allow_external_writes: false,
                    ..
                },
            ) => Err(ExternalWriteForbidden { line, path }),
            (is_empty, _) => {
                if !is_empty {
                    line.truncate(first.len());
                    result.entry(clean(path)).or_default().push(line);
                }
                Ok(None)
            }
        }
    } else {
        Ok(Some(line))
    }
}

fn write(
    stdout: &mut impl Write,
    lines: impl IntoIterator<Item = Cow<'static, str>>,
) -> Result<(), Error> {
    for line in lines {
        for chunk in [line.as_bytes(), b"\n"] {
            stdout.write_all(chunk).map_err(WriteFailed)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{HashMap, Locale, MovePolicy, format};
    use std::io::BufReader;

    #[test]
    fn test_format() {
        let locale: Locale = "uk-UA".parse().unwrap();
        let prepend_lines = vec![String::new(), "another".into(), "Перший рядок   ".into()];
        let content = [
            "second line",
            "3 three",
            "Another  ",
            "move me \t:> destination   ",
            "delete me     :> \t",
            "move me as well:>./destination ",
            "move me not there:>  .././../destination2",
            "   ",
            "",
            "",
            "\n",
            "",
            "x",
            "",
            "a X",
            "Є d",
            "b   ",
            "   ",
            "",
        ];
        let content = content.join("\n").into_bytes();

        let mut source = BufReader::new(&content[..]);
        let mut sink: Vec<u8> = Vec::new();

        let mut lines = HashMap::new();
        lines.insert(
            "destination".into(),
            vec!["move me \t".into(), "move me as well".into()],
        );
        lines.insert(
            "../../destination2".into(),
            vec!["move me not there".into()],
        );

        assert_eq!(
            format(
                &mut source,
                &mut sink,
                &locale,
                MovePolicy {
                    marker: ":>",
                    allow_external_writes: true,
                    allow_deletions: true
                },
                prepend_lines
            )
            .unwrap(),
            lines
        );

        let expected = [
            "3 three",
            "Перший рядок",
            "Another",
            "Second line",
            "",
            "",
            "x",
            "",
            "є d",
            "a X",
            "b",
            "",
        ];

        assert_eq!(String::from_utf8(sink).unwrap(), expected.join("\n"));
    }
}
