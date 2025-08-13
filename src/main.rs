use wreq::Client;
use wreq_util::Emulation;

#[tokio::main]
async fn main() -> wreq::Result<()> {
    let cookie = "csrftoken=; cf_clearance=; techaro.lol-anubis-auth-auth=; session-affinity=; sessionid=";
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:139.0) Gecko/20100101 Firefox/139.0";
    let url = "";

    let mut headers: wreq::header::HeaderMap = wreq::header::HeaderMap::new();

    match wreq::header::HeaderValue::from_str(cookie)
            {
                Ok(o) => _ = headers.insert(wreq::header::COOKIE, o),
                Err(e) => println!("Cookie error: {e}"),
            }

    match wreq::header::HeaderValue::from_str(ua)
            {
                Ok(o) => _ = headers.insert(wreq::header::USER_AGENT, o),
                Err(e) => println!("User-Agent error: {e}"),
            }

    let client = Client::builder()
        .emulation(Emulation::Firefox139)
        .default_headers(headers)
        .build()?;

    let resp = client.get(url)
    .send()
    .await?;

    println!("{}", resp.text().await?);
    Ok(())
}
