use crate::model::Game;
use serde_json;
use std::{collections::HashMap, fs, sync::Arc};
use trie_rs::Trie;

/// Class for loading game related data and returning Arc references to games
pub struct GameLoader {
    games_trie: Trie<u8>,
    games: HashMap<String, Arc<Game>>,
}

impl GameLoader {
    /*pub fn new(args: GameLoaderArgs) -> Self {
        /// check if already saved locally
        if let Ok(s) = fs::read_to_string(args.data_file_path) {
            if let Ok(res) = serde_json::from_str(&s) {
                return res;
            }
        }

        // create from steamSpy

        // create games trie

    }*/
}

/// Args for GameLoader
pub struct GameLoaderArgs {
    game_lookup_depth: i32,
    search_suggestions_depth: i32,
    data_file_path: String,
}

impl GameLoaderArgs {
    pub fn default() -> Self {
        GameLoaderArgs {
            game_lookup_depth: 1000,
            search_suggestions_depth: 5,
            data_file_path: "game_data.json".to_owned(),
        }
    }
}
