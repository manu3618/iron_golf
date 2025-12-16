use rand::distr::Alphanumeric;
use rand::prelude::*;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
struct URL(String);

#[derive(Debug)]
enum URLParseError {
    NoScheme,
    InvalidScheme(String),
    NoHost,
    InvalidHost,
    InvalidPath,
}

const KNOWN_SCHEME: [&str; 4] = ["http", "https", "ftp", "ftps"];

impl FromStr for URL {
    type Err = URLParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('/');
        let scheme = match parts.next() {
            Some(v) => v.to_lowercase(),
            None => return Err(URLParseError::NoScheme),
        };

        if scheme.chars().last() != Some(':') {
            return Err(URLParseError::InvalidScheme(
                "scheme should end with a ':'".into(),
            ));
        }
        if scheme.chars().filter(|&c| c == ':').count() != 1 {
            return Err(URLParseError::InvalidScheme(
                "scheme should not contain ':'".into(),
            ));
        }

        let scheme = scheme.split(':').next().unwrap();
        if !KNOWN_SCHEME.contains(&scheme) {
            return Err(URLParseError::InvalidScheme(format!(
                "{} is not a known scheme",
                &scheme
            )));
        }
        let _ = parts.next();
        let host = match parts.next() {
            Some(v) => v.to_lowercase(),
            None => return Err(URLParseError::NoHost),
        };
        let path = parts.into_iter().collect::<Vec<_>>().join("/");
        Ok(Self(format!("{scheme}://{host}/{path}")))
    }
}

impl fmt::Display for URL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
struct ID(String);

impl ID {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let size = 5;
        Self(
            (0..size)
                .map(|_| rng.sample(Alphanumeric) as char)
                .collect(),
        )
    }
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default)]
struct UrlSet {
    set: VecDeque<(ID, URL)>,
    max_size: usize,
}

impl UrlSet {
    fn new() -> Self {
        let s = 1024;
        let s = 4;
        Self {
            set: VecDeque::with_capacity(s),
            max_size: s,
        }
    }

    fn store_url(&mut self, url: URL) -> ID {
        if self.set.len() > self.max_size - 1 {
            let _ = self.set.pop_front();
        }
        let ids = self.set.iter().map(|(elt, _)| elt).collect::<HashSet<_>>();
        let mut id = ID::new();
        while ids.contains(&id) {
            eprintln!("regenrating id {}", id);
            id = ID::new();
        }
        self.set.push_back((id.clone(), url));
        id
    }

    fn retrieve(&self, id: &ID) -> Option<URL> {
        let p = self.set.iter().position(|(a, _)| a == id)?;
        Some(self.set.get(p).unwrap().1.clone())
    }

    /// Retrieve an url and put it back in queue
    fn retrieve_refresh(&mut self, id: &ID) -> Option<URL> {
        let p = self.set.iter().position(|(a, _)| a == id)?;
        let (id, url) = self.set.remove(p)?;
        self.set.push_back((id, url.clone()));
        Some(url.clone())
    }
}


// TODO:
// simple web server (axum?)
// * GET id
//   - if exists -> 302
//   - else -> 404
// * POST url
//   - return id
fn main() {
    println!("Hello, world!");
    let _ = dbg!(URL::from_str("http://manu.hbrt.eu/get?id=machin".into()));
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
