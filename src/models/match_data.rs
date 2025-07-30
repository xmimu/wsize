#[derive(Debug, Clone)]
pub struct MatchData {
    pub asset_type: String,
    pub asset_name: String,
    pub source_path: String,
    pub language: String,
    pub shor_id: String,
    pub size: u64,
}
