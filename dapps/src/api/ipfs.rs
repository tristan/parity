use cid::Cid;
use url::Url;
use endpoint::Handler;
use try_from::TryFrom;

pub fn resolve(url: &Url) -> Option<Box<Handler>> {
    if &url.path[2..] != &["block", "get"] {
        return None;
    }

    let content_id = url.get_param("arg");

    let parsed = Cid::try_from(content_id.unwrap());

    println!("{:?} {:?}", content_id, parsed);

    None
}
