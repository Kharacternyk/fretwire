#[derive(Copy, Clone)]
pub struct MovePolicy<'a> {
    pub marker: &'a str,
    pub allow_external_writes: bool,
    pub allow_deletions: bool,
}
