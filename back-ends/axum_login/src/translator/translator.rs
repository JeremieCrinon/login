use std::{collections::HashMap, fs};

/// This will be instantiated at app's startup, get and parse the translations json files, and then, have methods that returns the parsed translation in the asked language, these methods will then be called in handler or anywhere there is need for a translation
#[derive(Debug, Clone)]
pub struct Translator {
    en: serde_json::Value,
    fr: serde_json::Value,
}

impl Translator {
    pub fn new() -> Self {
        let en_file = fs::read_to_string("./translations/en.json").unwrap();
        let en_json: serde_json::Value = serde_json::from_str(&en_file).unwrap();

        let fr_file = fs::read_to_string("./translations/fr.json").unwrap();
        let fr_json: serde_json::Value = serde_json::from_str(&fr_file).unwrap();

        Translator { en: en_json, fr: fr_json }
    }

    pub fn get_translation (&self, lang: &str) -> HashMap<String, String> {
        let json = match lang {
            "fr" => &self.fr,
            _ => &self.en
        };
        
        return json
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();
    }
}