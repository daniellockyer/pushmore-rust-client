extern crate reqwest;

use std::env;

/// Base webhook URL for the Push More service.
static WEBHOOK_URL: &str = "https://pushmore.io/webhook/";

#[derive(Default)]
pub struct PushMore {
    key: String
}

impl PushMore {
    /// Create a PushMore instance, taking the key from the `PUSH_MORE_KEY` environment variable.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let client = PushMore::new();
    /// ```
    ///
    /// # Panics
    ///
    /// If the environment value is missing.
    pub fn new() -> Self {
        let env_key = "PUSH_MORE_KEY";

        match env::var(env_key) {
            Ok(val) => PushMore::new_with_key(val),
            Err(e) => panic!("Invalid value for {}: `{}`", env_key, e),
        }
    }

    /// Create a PushMore instance with the supplied key.
    ///
    /// # Arguments
    ///
    /// * `key`: Push More API key from [Push More](https://pushmore.io)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let client = PushMore::new_with_key("<pushmore key>".to_string());
    /// ```
    pub fn new_with_key(key: String) -> Self {
        PushMore {
            key
        }
    }

    /// Send a String through the Push More service to Telegram.
    ///
    /// Returns whether it was successful or not.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let success = client.send("hello!".to_string());
    /// ```
    pub fn send(&self, body: String) -> bool {
        let url = format!("{}{}", WEBHOOK_URL, self.key);
        let client = reqwest::Client::new();

        if let Ok(mut r) = client.post(&url).body(body).send() {
            if let Ok(b) = r.text() {
                return b.contains("OK");
            }
        }
        false
    }
}
