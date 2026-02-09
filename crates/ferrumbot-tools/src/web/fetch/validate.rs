use std::net::IpAddr;

use reqwest::Url;

pub(super) fn validate_url(url: &str) -> Result<(), String> {
    let parsed = Url::parse(url).map_err(|e| format!("invalid url: {e}"))?;
    let scheme = parsed.scheme();
    if scheme != "http" && scheme != "https" {
        return Err("only http/https URLs are allowed".to_string());
    }

    let Some(host) = parsed.host_str() else {
        return Err("URL must include a host".to_string());
    };

    let lowered = host.to_ascii_lowercase();
    if lowered == "localhost" || lowered.ends_with(".local") {
        return Err("localhost/local domains are blocked".to_string());
    }

    if let Ok(ip) = host.parse::<IpAddr>()
        && is_private_ip(ip)
    {
        return Err("private or local IP ranges are blocked".to_string());
    }

    Ok(())
}

fn is_private_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            v4.is_private()
                || v4.is_loopback()
                || v4.is_link_local()
                || v4.is_broadcast()
                || v4.is_documentation()
                || v4.is_unspecified()
        }
        IpAddr::V6(v6) => {
            v6.is_loopback()
                || v6.is_unique_local()
                || v6.is_unicast_link_local()
                || v6.is_multicast()
                || v6.is_unspecified()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::validate_url;

    #[test]
    fn validate_url_allows_public_https() {
        assert!(validate_url("https://example.com").is_ok());
    }

    #[test]
    fn validate_url_blocks_non_http_scheme() {
        assert!(validate_url("ftp://example.com").is_err());
    }

    #[test]
    fn validate_url_blocks_localhost() {
        assert!(validate_url("http://localhost:8080/health").is_err());
    }

    #[test]
    fn validate_url_blocks_private_ip() {
        assert!(validate_url("http://127.0.0.1:8080/health").is_err());
    }
}
