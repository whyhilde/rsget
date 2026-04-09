pub fn extract_filename(url: &str) -> String {
    url.split("/").last().unwrap_or("download").to_string()
}
