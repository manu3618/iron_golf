mod url_set;
use url_set::*;
use std::str::FromStr;

// TODO:
// simple web server (axum?)
// * GET id
//   - if exists -> 302
//   - else -> 404
// * POST url
//   - return id
fn main() {
    println!("Hello, world!");
    let _ : URL = dbg!("http://manu.hbrt.eu/get?id=machin".parse().unwrap());
    let mut url_set = UrlSet::new();
    let ids = (0..10)
        .map(|_| url_set.store_url(URL::from_str("http://example.com").unwrap()))
        .collect::<Vec<_>>();
    dbg!(&ids);
    dbg!(&url_set);
    dbg!(url_set.retrieve(&ids[3]));
    dbg!(url_set.retrieve(&ids[9]));
    dbg!(&url_set);
    dbg!(url_set.retrieve_refresh(&ids[7]));
    dbg!(&url_set);
}
