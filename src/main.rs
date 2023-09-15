use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user_name = String::from("testuser");
    let password: Option<String> = None;

    let res = client
        .get("https://httpbin.org/")
        .basic_auth(user_name, password)
        .send();
    
    println!("\n{:#?}", res);
    
    Ok(()) 
}