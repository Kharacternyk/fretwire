use self::MarkerBoundary::{Absent, Closed, Open};

// TODO: Attach to the string and the marker?
#[derive(Copy, Clone)]
pub enum MarkerBoundary {
    Absent,
    Open(usize),
    Closed(usize),
}

impl MarkerBoundary {
    #[must_use]
    pub fn find(line: &str, marker: &str) -> Self {
        if marker.is_empty() {
            Absent
        } else if let Some(i) = line.find(marker) {
            if i + marker.len() == line.len() {
                Open(i)
            } else {
                Closed(i)
            }
        } else {
            let mut indices: Vec<_> = marker.char_indices().skip(1).collect();

            indices.reverse();

            let mut result = Absent;

            for (i, _) in indices {
                if line.ends_with(&marker[..i]) {
                    result = Open(line.len() - i);
                    break;
                }
            }

            result
        }
    }

    #[must_use]
    pub fn before_marker(self, line: &str) -> &str {
        match self {
            Absent => line,
            Open(i) | Closed(i) => &line[..i],
        }
    }
}
