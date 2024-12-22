use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::fmt::Write as FmtWrite;

struct HttpClient {
    host: String,
    port: u16,
    path: String,
    timeout: Duration,
}

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

impl HttpClient {
    fn new(host: &str, port: u16, path: &str, timeout: Duration) -> Self {
        HttpClient {
            host: host.to_string(),
            port,
            path: path.to_string(),
            timeout,
        }
    }

    fn request(
        &self,
        method: &str,
        headers: Option<HashMap<&str, &str>>,
        body: Option<&str>,
    ) -> Result<String, Box<dyn  Error>> {
        let addr = format!("{}:{}", self.host, self.port);
        let mut addrs = addr.to_socket_addrs()?;
        let socket_addr = addrs.next().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Could not resolve address"))?;

        let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(2))?;

        stream.set_read_timeout(Some(self.timeout))?;
        stream.set_write_timeout(Some(self.timeout))?;

        let mut request = String::new();
        write!(&mut request, "{} {} HTTP/1.1\r\nHost: {}\r\n", method, self.path, self.host)?;
        write!(&mut request, "User-Agent: Rust HTTP Client\r\n")?;
        if let Some(headers) = headers {
            for (key, value) in headers {
                write!(&mut request, "{}: {}\r\n", key, value)?;
            }
        }
        write!(request, "Connection: close\r\n")?;
        if let Some(body) = body {
            write!(&mut request, "Content-Length: {}\r\n", body.len())?;
        }

        write!(&mut request, "\r\n")?;
        if let Some(body) = body {
            write!(&mut request, "{}", body)?;
        }

        stream.write_all(request.as_bytes())?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        let body = response.split("\r\n\r\n").nth(1).unwrap_or("");
        Ok(body.to_string())
    }
}

fn client(url: &str, timeout: Duration) -> Result<HttpClient, Box<dyn Error>> {
    // 127.0.0.1:9090/ping
    let url = url.strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .ok_or("Invalid URL: Missing protocol (http or https)")?;

    // ["127.0.0.1:9090", "ping"]
    let mut parts = url.splitn(2, '/');
    let host_and_port = parts.next().ok_or("Invalid URL: Missing host")?;
    let path = format!("/{}", parts.next().unwrap_or(""));

    // ["127.0.0.1", "9090"]
    let mut host_parts = host_and_port.splitn(2, ':');
    let host = host_parts.next().ok_or("Invalid URL: Missing host")?.to_string();
    let port = host_parts
        .next()
        .map(|p| p.parse::<u16>().map_err(|_| "Invalid port"))
        .transpose()?
        .unwrap_or(80);

    Ok(HttpClient::new(&host, port, &path, timeout))
}

/// GET Request
///
/// # Example
///
/// ```txt
/// use light_tool::http;
/// assert_eq!(http::get("http://example.com", None).is_ok(), true)
/// ```
pub fn get(url: &str, headers: Option<HashMap<&str, &str>>) -> Result<String, Box<dyn Error>> {
    let client = client(url, DEFAULT_TIMEOUT)?;
    client.request("GET", headers, None)
}

/// POST Request
///
/// # Example
///
/// ```txt
/// use light_tool::http;
/// assert_eq!(http::post("http://example.com", None, None).is_ok(), true)
/// ```
pub fn post(url: &str, headers: Option<HashMap<&str, &str>>, body: Option<&str>) -> Result<String, Box<dyn Error>> {
    let client = client(url, DEFAULT_TIMEOUT)?;
    client.request("POST", headers, body)
}

/// PUT Request
///
/// # Example
///
/// ```txt
/// use light_tool::http;
/// assert_eq!(http::put("http://192.168.110.106:9900/api/v1/sys/node/dtu", None,
///     Some("{\"dtu\": true, \"identity\": \"e540f857-704b-4985-bb69-3d6c935debb0\"}")).is_ok(), true)
/// ```
pub fn put(url: &str, headers: Option<HashMap<&str, &str>>, body: Option<&str>) -> Result<String, Box<dyn Error>> {
    let client = client(url, DEFAULT_TIMEOUT)?;
    client.request("PUT", headers, body)
}

/// DELETE Request
///
/// # Example
///
/// ```txt
/// use light_tool::http;
/// assert_eq!(http::delete("http://192.168.110.106:9900/api/v1/sys/param/quality/delete?identity=1", None).is_ok(), true)
/// ```
pub fn delete(url: &str, headers: Option<HashMap<&str, &str>>) -> Result<String, Box<dyn Error>> {
    let client = client(url, DEFAULT_TIMEOUT)?;
    client.request("DELETE", headers, None)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let response = get("http://example.com", None).expect("GET request failed");
        // <!doctype html>
        // <html> ...
        println!("Response: {}", response);
    }

    #[test]
    fn test_post() {
        let response = post("http://192.168.111.30:9610/api/v1/index/system/state/get", None, None).
            expect("POST request failed");
        // Response: {"code":200,"msg":"success","data":{"state_all":"CLOSE","processes":[]}}
        println!("Response: {}", response);
    }

    #[test]
    fn test_put() {
        let response = put("http://192.168.110.106:9900/api/v1/sys/node/dtu", None,
                           Some("{\"dtu\": true, \"identity\": \"e540f857-704b-4985-bb69-3d6c935debb0\"}")).
            expect("PUT request failed");
        // Response: {"code":200,"msg":"成功","data":null}
        println!("Response: {}", response);
    }

    #[test]
    fn test_delete() {
        let response = delete("http://192.168.110.106:9900/api/v1/sys/param/quality/delete?identity=1", None).
            expect("DELETE request failed");
        // Response: {"code":200,"msg":"成功","data":null}
        println!("Response: {}", response);
    }
}
