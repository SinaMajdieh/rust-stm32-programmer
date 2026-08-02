#[derive(Debug, Clone)]
pub enum Report {
    Progress(u8),
    Log(String),
}