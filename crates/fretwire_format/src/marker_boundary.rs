pub enum MarkerBoundary {
    Open(usize),
    Closed(usize, usize),
}

impl MarkerBoundary {
    pub fn find(line: &str, marker: &str) -> Option<Self> {
        if marker.is_empty() {
            None
        } else if let Some(i) = line.find(marker) {
            Some(Self::Closed(i, marker.len()))
        } else {
            let mut indices: Vec<_> = marker.char_indices().skip(1).collect();

            indices.reverse();

            let mut result = None;

            for (i, _) in indices {
                if line.ends_with(&marker[..i]) {
                    result = Some(Self::Open(i));
                    break;
                }
            }

            result
        }
    }
}
