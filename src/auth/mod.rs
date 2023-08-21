use reqwest::header::{ACCEPT, CONTENT_TYPE, DNT, HeaderMap, ORIGIN, REFERER, USER_AGENT};

pub mod sms;

pub fn generate_header_map() -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();

    headers.append(USER_AGENT, "Mozilla/5.0 (X11; GNU/Linux) AppleWebKit/537.36 (KHTML, like Gecko) Chromium/88.0.4324.150 Chrome/88.0.4324.150 Safari/537.36 Tesla/DEV-BUILD-db8799708f22".parse().unwrap());
    headers.append("authority","auth.privy.io".parse().unwrap());
    headers.append(ACCEPT,"application/json".parse().unwrap());
    headers.append(CONTENT_TYPE,"application/json".parse().unwrap());
    headers.append(DNT,"1".parse().unwrap());
    headers.append(ORIGIN,"https://www.friend.tech".parse().unwrap());
    headers.append("privy-app-id","cll35818200cek208tedmjvqp".parse().unwrap());
    headers.append("privy-client","react-auth:1.34.0".parse().unwrap());
    headers.append(REFERER,"https://www.friend.tech/".parse().unwrap());

    headers
}