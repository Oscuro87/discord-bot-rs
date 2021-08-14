use ron::de::from_reader;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct BotInfo {
    pub token: String,
    pub prefix: String,
    pub ignore_bots: bool,
    pub owners_ids: Vec<u64>,
}

pub fn read_bot_infos() -> BotInfo {
    let file_path = "data/info.ron";
    let file = std::fs::File::open(file_path).expect("Cannot open file data/info.ron");
    match from_reader(file) {
        Ok(result) => result,
        Err(err) => {
            println!("Failed to open info.ron: {}", err);
            std::process::exit(1);
        }
    }
}
