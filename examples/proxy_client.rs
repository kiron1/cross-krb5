use cross_krb5::ClientCtx;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (proxy_host, proxy_port) = parse_arguments();

    let target_principal = format!("HTTP/{}", proxy_host);
    let (_pending, token) = ClientCtx::initiate(None, &target_principal)?;

    let mut stream = TcpStream::connect((&*proxy_host, proxy_port))?;

    // Send a HTTP request with the Kerberos token
    let request = http_request("http://example.org", "example.org", &*token);
    println!("{}", &request);
    stream.write_all(request.as_bytes())?;

    // Print the proxy response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", &response);

    Ok(())
}

// A simple function to generate a HTTP request, in a real world code
// we would use a crate like hyper or similar.
fn http_request(url: &str, host: &str, token: &[u8]) -> String {
    let lines = vec![
        format!("GET {} HTTP/1.1", url),
        format!("Host: {}", host),
        // This is the important part, we transmit the Kerberos token bas64 encoded
        // See also: https://datatracker.ietf.org/doc/html/rfc4559#section-3
        format!("Proxy-Authorization: Negotiate {}", base64::encode(&token)),
        String::from("Connection: close"),
        // Empty line to complete the HTTP request.
        String::from(""),
        String::from(""),
    ];
    lines.join("\r\n")
}

/// Very simply argument extraction.
fn parse_arguments() -> (String, u16) {
    let mut args = std::env::args();
    let _prog = args.next().unwrap_or_default();
    let proxy_host = args.next().expect("first argument must be a proxy host name");
    let proxy_port = args
        .next()
        .expect("second argument must be a proxy port")
        .parse::<u16>()
        .expect("port must be a valid number");
    (proxy_host, proxy_port)
}
