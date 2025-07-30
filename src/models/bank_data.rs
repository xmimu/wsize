use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct BankEntry {
    bnk_path: PathBuf,
    json_path: PathBuf,
    language: String,
    wem_paths: Vec<PathBuf>,
    bnk_size: u64,
    wem_size: u64,
}

#[derive(Deserialize)]
struct BankJson {
    SoundBanksInfo: SoundBanksInfo,
}

#[derive(Deserialize)]
struct SoundBanksInfo {
    Platform: String,
    SoundBanks: Vec<SoundBank>,
}

#[derive(Deserialize)]
struct SoundBank {
    Id: String,
    r#Type: String,
    GUID: String,
    Language: String,
    ShortName: String,
    Path: PathBuf,
    Media: Option<Vec<MediaEntry>>,
}

#[derive(Deserialize)]
struct MediaEntry {
    Id: String,
    Language: String,
    ShortName: String,
    Path: PathBuf,
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_parse_json() {
        let json_path = PathBuf::from(r"E:\qc_publish_WwiseBank\PS5\SoundbanksInfo.json");
        let content = read_to_string(&json_path).unwrap();
        let result: BankJson = serde_json::from_str(&content).expect("解析失败！");
        println!("{:?}", result.SoundBanksInfo.Platform);
    }
}
