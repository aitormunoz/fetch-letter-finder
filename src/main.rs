use clap::{App, Arg};
use hyper::{body, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let matches = App::new("fetch letter finder")
        .arg(
            Arg::with_name("url")
                .takes_value(true)
                .long("url")
                .required(true)
                .help("Url to fetch")
        )
        .arg(
            Arg::with_name("find")
                .takes_value(true)
                .long("find")
                .required(true)
                .help("Word to find")
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let url = url.parse::<hyper::Uri>()?;
    let find = matches.value_of("find").unwrap();
    let client = Client::new();
    let resp = client.get(url).await?;

    let body_bytes = body::to_bytes(resp.into_body()).await?;
    let body = String::from_utf8(body_bytes.to_vec()).expect("response was not valid utf-8");

    println!("Found {} matches", body.matches(find).count());

    Ok(())
}

