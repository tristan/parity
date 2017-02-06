use cid::{Cid, Codec};
use url::Url;
use endpoint::Handler;
use try_from::TryFrom;

pub fn resolve(url: &Url) -> Option<Box<Handler>> {
    if &url.path[2..] != &["block", "get"] {
        return None;
    }

    let cid = Cid::try_from(url.get_param("arg").unwrap()).unwrap();

    assert_eq!(cid.hash[0], 0x1b); // 0x1b == Keccak-256
    assert_eq!(cid.codec, Codec::EthereumBlock);

    let block = &cid.hash[2..];

    println!("{:?} {:?}", content_id, parsed);

    None
}
