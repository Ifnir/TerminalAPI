use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};

// Example function to send HTTP requests
async fn send_request(method: &str, url: &str, body: Option<String>, headers: Option<HeaderMap>) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let request_builder = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "PATCH" => client.patch(url),
        "DELETE" => client.delete(url),
        _ => return Err(reqwest::Error::new(reqwest::Error::Kind::InvalidInput, "Invalid HTTP method")),
    };
    let mut request = match body {
        Some(body) => request_builder.body(body),
        None => request_builder,
    };
    if let Some(headers) = headers {
        request = request.headers(headers);
    }
    let response = request.send().await?.text().await?;
    Ok(response)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = send_request("GET", "https://jsonplaceholder.typicode.com/todos/1", None, None).await?;
    println!("{}", response);
    Ok(())
}

