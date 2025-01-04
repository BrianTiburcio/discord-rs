pub struct ClientBuilder {
    token: Option<String>,
    intents: Vec<GatewayIntents>,
    // cache_settings: CacheSettings,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            token: None,
            intents: Vec::new(),
            // cache_settings: CacheSettings::default(),
        }
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn intents(mut self, intents: Vec<GatewayIntents>) -> Self {
        self.intents = intents;
        self
    }

    // pub fn cache_settings(mut self, settings: CacheSettings) -> Self {
    //     self.cache_settings = settings;
    //     self
    // }

    pub fn build(self) -> Result<Client> {
        let token = self.token.ok_or(DiscordError::Gateway("Token not provided".to_string()))?;
        let intents = self.intents;
        // let cache_settings = self.cache_settings;

        Ok(Client {
            token,
            intents,
            // cache: Cache::new(cache_settings),
            // event_handler: None,
        })
    }
}