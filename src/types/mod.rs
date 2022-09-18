#[derive(FromForm)]
pub struct ScanForm<'r> {
    #[field(default="https://wordpress.org")]
    pub url: &'r str,
}