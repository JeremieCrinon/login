use crate::config::CONFIG;

pub fn get_token_from_keychain() -> Option<String> {
    let entry: Option<keyring::Entry> = match keyring::Entry::new(CONFIG.app_name.as_str(), "token") {
        Ok(e) => Some(e),
        Err(e) => {
            println!("Error getting the token keyring entry: {}", e);
            None
        }
    };

    let token = match entry {
        Some(e) => {
            match e.get_password() {
                Ok(t) => Some(t),
                Err(e) => {
                    println!("Failed to get password from token keyring entry, maybe the user isn't logged in: {}", e);
                    None
                }
            }
        }
        None => None
    };

    token
}
