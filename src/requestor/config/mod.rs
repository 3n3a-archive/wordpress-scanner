use rand::{self, Rng};
use ::phf::{Map, phf_map};

// all collected from d2hhdGlzbXlicm93c2VyLmNvbQ==
static USER_AGENTS: Map<&'static str, &'static str> = phf_map! {
    "common" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36",
    "common-1" => "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36",
    "common-2" => "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36",
    "common-m" => "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_0_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36",
    "common-old" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36",
    "google-bot-d-1" => "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "google-bot-d-2" => "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html) Chrome/107.0.0.0 Safari/537.36",
    "google-bot-d-3" => "Googlebot/2.1 (+http://www.google.com/bot.html)",
    "google-bot-s-1" => "Mozilla/5.0 (Linux; Android 6.0.1; Nexus 5X Build/MMB29P) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Mobile Safari/537.36 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "google-favicon" => "Mozilla/5.0 (compatible; Google-Site-Verification/1.0)",
    "safari-m" => "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_0_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15",
    "safari-i-1" => "Mozilla/5.0 (iPhone; CPU iPhone OS 16_1_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Mobile/15E148 Safari/604.1",
    "safari-i-2" => "Mozilla/5.0 (iPad; CPU OS 16_1_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Mobile/15E148 Safari/604",
};

// for getting a specific user agent out of the set
pub fn get_user_agent(key: &str) -> &'static str {
    return USER_AGENTS[key];
}

// will return a random user agent
pub fn get_random_user_agent() -> &'static str {
    let mut rng = rand::thread_rng();
    let mut iter = USER_AGENTS.into_iter();
    let length = USER_AGENTS.len() - 1;
    let chosen = rng.gen_range(0..length);
    return iter.nth(chosen).unwrap_or(USER_AGENTS.get_entry("common").unwrap()).1;
}