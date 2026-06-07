use self::error::Error::{self, ReadFailed, WriteFailed};
use crate::{locale::Locale, paragraph::Paragraph};
use std::{
    borrow::Cow,
    collections::HashMap,
    io::{BufRead, Write},
};

pub mod error;

pub fn format(
    source: &mut impl BufRead,
    mut sink: &mut impl Write,
    locale: &Locale,
    move_marker: &str,
) -> Result<HashMap<String, Vec<String>>, Error> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    let mut paragraph = Paragraph::new(locale);

    for line in source.lines() {
        let mut line = line.map_err(ReadFailed)?;

        if !move_marker.is_empty()
            && let Some((body, destination)) = line.split_once(move_marker)
        {
            let destination: String = destination.trim().into();

            if !destination.is_empty() {
                line.truncate(body.len());
                result.entry(destination).or_default().push(line);
            }
        } else {
            write(&mut sink, paragraph.feed(line))?;
        }
    }

    write(&mut sink, paragraph.flush())?;

    Ok(result)
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
    use super::{HashMap, Locale, format};
    use std::io::BufReader;

    #[test]
    fn test_format() {
        let locale: Locale = "uk-UA".parse().unwrap();
        let content = vec![
            "",
            "Перший рядок   ",
            "second line",
            "Another  ",
            "move me \t:> destination   ",
            "delete me     :> \t",
            "another",
            "move me as well:>destination ",
            "move me not there:>  destination2",
            "3 three",
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
        ]
        .join("\n")
        .into_bytes();

        let mut source = BufReader::new(&content[..]);
        let mut sink: Vec<u8> = Vec::new();

        let mut result = HashMap::new();
        result.insert(
            "destination".into(),
            vec!["move me \t".into(), "move me as well".into()],
        );
        result.insert("destination2".into(), vec!["move me not there".into()]);

        assert_eq!(
            format(&mut source, &mut sink, &locale, ":>").unwrap(),
            result
        );

        assert_eq!(
            String::from_utf8(sink).unwrap(),
            vec![
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
            ]
            .join("\n")
        );
    }
}
