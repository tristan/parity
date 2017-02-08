// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::sync::Arc;
use unicase::UniCase;
use hyper::{server, net, Decoder, Encoder, Next, Control};
use hyper::header;
use hyper::header::AccessControlAllowOrigin;
use hyper::method::Method;
use cid::{Cid, Codec};
use try_from::TryFrom;

use url::Url;
use api::types::{App, ApiError};
use api::response;

use ethcore::types::encoded;
use ethcore::client::{BlockId, BlockchainClient};
use endpoint::{Endpoint, Endpoints, Handler, EndpointPath};
use handlers::extract_url;
use jsonrpc_http_server::cors;

#[derive(Clone)]
pub struct IpfsApi {
	client: Arc<BlockchainClient>,
	fetcher: Arc<Fetcher>,
}

impl IpfsApi {
	pub fn new(client: Arc<BlockchainClient>, fetcher: Arc<Fetcher>) -> Box<Endpoint> {
		Box::new(IpfsApi {
			client: client,
			fetcher: fetcher,
		})
	}
}

impl Endpoint for IpfsApi {
	fn to_async_handler(&self, path: EndpointPath, control: Control) -> Box<Handler> {
		Box::new(RestApiRouter::new(self.clone(), path, control))
	}
}

struct IpfsApiRouter {
	api: IpfsApi,
	origin: Option<String>,
	path: Option<EndpointPath>,
	control: Option<Control>,
	handler: Box<Handler>,
}

impl IpfsApiRouter {
	fn new(api: IpfsApi, path: EndpointPath, control: Control) -> Self {
		IpfsApiRouter {
			path: Some(path),
			origin: None,
			control: Some(control),
			api: api,
			handler: response::as_json_error(&ApiError {
				code: "404".into(),
				title: "Not Found".into(),
				detail: "Resource you requested has not been found.".into(),
			}),
		}
	}

	 // hash: Option<&str>, path: EndpointPath, control: Control)

	fn resolve_content(&mut self, url: &Url) -> Option<Box<Handler>> {
		let hash = url.path.get(2);

		let mut path = self.path.take().expect("on_request called only once, and path is always defined in new; qed");
		let control = self.control.take().expect("on_request called only once, and control is always defined in new; qed");

		self.api.fetcher.to_async_handler(path, control)
	}

	/// Returns basic headers for a response (it may be overwritten by the handler)
	fn response_headers(&self) -> header::Headers {
		let mut headers = header::Headers::new();
		headers.set(header::AccessControlAllowCredentials);
		headers.set(header::AccessControlAllowMethods(vec![
			Method::Options,
			Method::Post,
			Method::Get,
		]));
		headers.set(header::AccessControlAllowHeaders(vec![
			UniCase("origin".to_owned()),
			UniCase("content-type".to_owned()),
			UniCase("accept".to_owned()),
		]));

		headers
	}
}

impl server::Handler<net::HttpStream> for IpfsApiRouter {

	fn on_request(&mut self, request: server::Request<net::HttpStream>) -> Next {
		self.origin = cors::read_origin(&request);

		if let Method::Options = *request.method() {
			self.handler = response::empty();
			return Next::write();
		}

		let url = match extract_url(&request) {
			Some(url) => url,

			// Just return 404 if we can't parse URL
			None => return Next::write()
		};

		println!("{:?}", &url);

		let endpoint = url.path.get(1);

		let handler = endpoint.and_then(|v| match v.as_str() {
			"v0" => resolve(&url),
			_ => None
		});

		// Overwrite default
		if let Some(h) = handler {
			self.handler = h;
		}

		self.handler.on_request(request)
	}

	fn on_request_readable(&mut self, decoder: &mut Decoder<net::HttpStream>) -> Next {
		self.handler.on_request_readable(decoder)
	}

	fn on_response(&mut self, res: &mut server::Response) -> Next {
		*res.headers_mut() = self.response_headers();
		self.handler.on_response(res)
	}

	fn on_response_writable(&mut self, encoder: &mut Encoder<net::HttpStream>) -> Next {
		self.handler.on_response_writable(encoder)
	}

}

pub fn resolve(url: &Url) -> Option<Box<Handler>> {
	if &url.path[2..] != &["block", "get"] {
		return None;
	}

	let cid = Cid::try_from(url.get_param("arg").unwrap()).unwrap();

	assert_eq!(cid.hash[0], 0x1b); // 0x1b == Keccak-256
	assert_eq!(cid.codec, Codec::EthereumBlock);

	let block_id = BlockId::Hash(cid.hash[2..].into());

	println!("{:?}", block_id);

	None
}
