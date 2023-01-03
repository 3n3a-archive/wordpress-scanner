use std::collections::HashMap;

use rocket::{serde::{Serialize, Deserialize}, request::{FromRequest, self, Outcome}, http::Status};

use crate::Config;

// what the server receives from a client
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(crate = "rocket::serde")]
pub struct WebScanInput<'r> {
    pub url: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

// Related to Scan Result

// === DUMMY DATA ===
// let scan_result: ScanResult<'_> = ScanResult {
//     url_info: types::UrlInfo {
//         original_url: "https://example.com/index.html",
//         host: "example.com",
//         scheme: "https",
//         port: "443",
//     },
//     req_info: types::ReqInfo {
//         headers: vec![types::ResHeader {
//             name: "Content-Type",
//             value: "application/json",
//         }],
//         status: types::ResStatus {
//             status_code: "200",
//             status_reason: "OK",
//         },
//         is_alive: true,
//         timing: types::ResTiming {
//             response_time: "2 ms",
//         },
//     },
//     document_info: types::DocumentInfo {
//         source_code: "JTNDIURPQ1RZUEUlMjBodG1sJTNFJTNDaHRtbCUyMGxhbmc9JTIyZW4lMjIlM0UlM0NoZWFkJTNFJTNDbWV0YSUyMGNoYXJzZXQ9JTIyVVRGLTglMjIlM0UlM0NtZXRhJTIwaHR0cC1lcXVpdj0lMjJYLVVBLUNvbXBhdGlibGUlMjIlMjBjb250ZW50PSUyMklFPWVkZ2UlMjIlM0UlM0NtZXRhJTIwbmFtZT0lMjJ2aWV3cG9ydCUyMiUyMGNvbnRlbnQ9JTIyd2lkdGg9ZGV2aWNlLXdpZHRoLCUyMGluaXRpYWwtc2NhbGU9MS4wJTIyJTNFJTNDdGl0bGUlM0VEb2N1bWVudCUzQy90aXRsZSUzRSUzQy9oZWFkJTNFJTNDYm9keSUzRSUzQy9ib2R5JTNFJTNDL2h0bWwlM0U=",
//         page_title: "Document",
//         css_urls: vec![types::SourceUrl {
//             url: "style.css"
//         }],
//         js_urls: vec![types::SourceUrl {
//             url: "/script.js"
//         }],
//         img_urls: vec![types::ImageUrl {
//             url: "image.jpg",
//             alt: ""
//         }],
//         link_urls: vec![types::SourceUrl {
//             url: "https://www.google.com"
//         }],
//     },
//     time_info: types::TimeInfo {
//         created_at: "Mon 21 November 2022 19:47:22.143 UTC",
//         timezone: "UTC"
//     },
//     framework_info: types::FrameworkInfo {
//         name: "Wordpress",
//         version: "6.2.1",
//         server: "Nginx/123",
//         detected_vulnerabilities: vec![]
//     },
// };
// ==== END DUMMY ====

// since it's just for returning, i'll use
// the static str (&str)
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UrlInfo {
    pub original_url: String,
    pub host: String, // url.host_str()
    pub scheme: String, // url.scheme (is already str)
    pub port: String, // url.port_or_known_default(), can be None, .to_string().as_str()
}

// represents a singular header
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResHeader {
    pub name: String,
    pub value: String,
}

// represents a singular status code
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResStatus {
    pub status_code: String,
    pub status_reason: String,
}

// represents an object
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResTiming {
    pub response_time: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqInfo {
    pub headers: Vec<ResHeader>,
    pub status: ResStatus,
    pub is_alive: bool,
    pub timing: ResTiming,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SourceUrl {
    pub url: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ImageUrl {
    pub url: String,
    pub alt: String, // if none present just empty string
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DocumentInfo {
    pub source_code: String, // as base64
    pub page_title: String, // for now only one, multiple could exits
    pub css_urls: Vec<SourceUrl>,
    pub js_urls: Vec<SourceUrl>,
    pub img_urls: Vec<ImageUrl>,
    pub link_urls: Vec<SourceUrl>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TimeInfo {
    pub created_at: String,
    pub timezone: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DetectedVulnerability {
    pub cve: String,
    pub severity: String, // should this be a number
    pub description: String,
    // pub associated_packages: String, // not sure if i need this
    // pub layer_id: String, // only for images
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FrameworkInfo {
    pub name: String,    // detected by generator tag, robots.txt, or admin url
    pub version: String,
    pub detected_vulnerabilities: Vec<DetectedVulnerability>,
    pub server: String,
}

// this will be output to the client
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ScanResult {
    pub url_info: UrlInfo,
    pub req_info: ReqInfo,
    pub document_info: DocumentInfo,
    pub time_info: TimeInfo,
    pub framework_info: FrameworkInfo,
}

// Error
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResult {
    /// The title of the error message
    pub err: String,
    /// The description of the error
    pub msg: Option<String>,
    // HTTP Status Code returned
    #[serde(skip)]
    pub http_status_code: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiKeyVerifyResponse {
    pub message: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiKeyToken {
    pub token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiKeyLoginResponse {
    pub message: String,
    pub data: ApiKeyToken,
}




// Implement the actual checks for the authentication
pub struct ApiKey(String);

// Function for Verifying API Key
async fn verify_key(key: String, config: &Config) -> bool {
    let client = reqwest::Client::new();

    // Login as user, to get JWT
    let mut login_map = HashMap::new();
    login_map.insert("username_or_email", &config.username);
    login_map.insert("password", &config.password);
    let aklg = client.post(format!("{}{}", config.base_url, "/api/auth/login"))
        .json(&login_map)
        .send()
        .await
        .unwrap()
        .json::<ApiKeyLoginResponse>()
        .await
        .unwrap();


    //println!("login: {:?}", aklg);

    // Verify API Key with JWT in Header
    let mut verify_map = HashMap::new();
    verify_map.insert("api_key", key);
    let akvr: ApiKeyVerifyResponse = client.post(format!("{}{}", config.base_url, "/api/api-key/verify"))
        .json(&verify_map)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", aklg.data.token))
        .send()
        .await
        .unwrap()
        .json::<ApiKeyVerifyResponse>()
        .await
        .unwrap();

    //println!("verify: {:?}", akvr);

    match akvr.message.as_str() {
        "ok" => true,
        _ => false,
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for ApiKey {
    type Error = &'static str;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        // Get the key from the http header
        match request.headers().get_one("x-api-key") {
            Some(key) => {
                let config = request.rocket().state::<Config>().unwrap();
                if verify_key(key.to_string(), config).await {
                    Outcome::Success(ApiKey(key.to_owned()))
                } else {
                    Outcome::Failure((Status::Unauthorized, "Api key is invalid."))
                }
            }
            None => Outcome::Failure((Status::BadRequest, "Missing `x-api-key` header.")),
        }
        // For more info see: https://rocket.rs/v0.5-rc/guide/state/#within-guards
    }
}