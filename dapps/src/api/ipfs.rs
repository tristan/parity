use url::Url;
use endpoint::Handler;

pub fn resolve(url: &Url) -> Option<Box<Handler>> {
    if &url.path[1..] == &["v0", "block", "get"] {
        return None;
    }

    println!("{:?}", url.get_param("arg"));

    None
}
