use rocket::serde::Serialize;

#[derive(FromForm)]
pub struct ScanForm<'r> {
    #[field(default = "https://wordpress.org")]
    pub url: &'r str,
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
pub struct UrlInfo<'r> {
    pub original_url: &'r str,
    pub host: &'r str, // url.host_str()
    pub scheme: &'r str, // url.scheme (is already str)
    pub port: &'r str, // url.port_or_known_default(), can be None, .to_string().as_str()
}

// represents a singular header
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResHeader<'r> {
    pub name: &'r str,
    pub value: &'r str,
}

// represents a singular status code
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResStatus<'r> {
    pub status_code: &'r str,
    pub status_reason: &'r str,
}

// represents an object
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResTiming<'r> {
    pub response_time: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqInfo<'r> {
    pub headers: Vec<ResHeader<'r>>,
    pub status: ResStatus<'r>,
    pub is_alive: bool,
    pub timing: ResTiming<'r>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SourceUrl<'r> {
    pub url: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ImageUrl<'r> {
    pub url: &'r str,
    pub alt: &'r str, // if none present just empty string
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DocumentInfo<'r> {
    pub source_code: &'r str, // as base64
    pub page_title: &'r str, // for now only one, multiple could exits
    pub css_urls: Vec<SourceUrl<'r>>,
    pub js_urls: Vec<SourceUrl<'r>>,
    pub img_urls: Vec<ImageUrl<'r>>,
    pub link_urls: Vec<SourceUrl<'r>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TimeInfo<'r> {
    pub created_at: &'r str,
    pub timezone: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DetectedVulnerability<'r> {
    pub cve: &'r str,
    pub severity: &'r str, // should this be a number
    pub description: &'r str,
    // pub associated_packages: &'r str, // not sure if i need this
    // pub layer_id: &'r str, // only for images
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FrameworkInfo<'r> {
    pub name: &'r str,    // detected by generator tag, robots.txt, or admin url
    pub version: &'r str,
    pub detected_vulnerabilities: Vec<DetectedVulnerability<'r>>,
    pub server: &'r str,
}

// this will be output to the client
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ScanResult<'r> {
    pub url_info: UrlInfo<'r>,
    pub req_info: ReqInfo<'r>,
    pub document_info: DocumentInfo<'r>,
    pub time_info: TimeInfo<'r>,
    pub framework_info: FrameworkInfo<'r>,
}