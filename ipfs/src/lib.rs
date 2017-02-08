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

extern crate ethcore;
extern crate hyper;
extern crate cid;
extern crate try_from;

use try_from::TryFrom;
use cid::{Cid, Codec};
use hyper::server::{Handler, Server, Request, Response};
use hyper::net::HttpStream;
use hyper::header::{ContentLength, ContentType};
use hyper::{Next, Encoder, Decoder, Method, RequestUri};
use ethcore::client::{BlockId, BlockChainClient};
use std::sync::Arc;
use std::thread;

struct IpfsHandler {
	client: Arc<BlockChainClient>,
	result: Option<Vec<u8>>,
}


/// Get a query parameter's value by name.
pub fn get_param<'a>(query: &'a str, name: &str) -> Option<&'a str> {
	query.split('&')
		.find(|part| part.starts_with(name) && part[name.len()..].starts_with("="))
		.map(|part| &part[name.len() + 1..])
}

impl Handler<HttpStream> for IpfsHandler {
	fn on_request(&mut self, req: Request<HttpStream>) -> Next {
		if *req.method() != Method::Get {
			return Next::end()
		}

		let cid = match *req.uri() {
			RequestUri::AbsolutePath {
				ref path,
				query: Some(ref query)
			} => {
				if path != "/api/v0/block/get" {
					return Next::end();
				}

				get_param(query, "arg")
			}
			_ => return Next::end(),
		};

		let cid = Cid::try_from(cid.unwrap()).unwrap();

		assert_eq!(cid.hash[0], 0x1b); // 0x1b == Keccak-256
		assert_eq!(cid.codec, Codec::EthereumBlock);

		let block_id = BlockId::Hash(cid.hash[2..].into());

		self.result = self.client.block(block_id).map(|block| block.into_inner());

		Next::write()
	}

	fn on_request_readable(&mut self, _decoder: &mut Decoder<HttpStream>) -> Next {
		Next::write()
	}

	fn on_response(&mut self, res: &mut Response) -> Next {
		match self.result {
			Some(ref bytes) => {
				let headers = res.headers_mut();

				headers.set(ContentLength(bytes.len() as u64));
				headers.set(ContentType("application/octet-stream".parse().unwrap()));

				Next::write()
			},
			None => Next::end(),
		}
	}

	fn on_response_writable(&mut self, transport: &mut Encoder<HttpStream>) -> Next {
		match self.result {
			Some(ref bytes) => {
				transport.write(&bytes).unwrap();

				Next::end()
			},
			None => Next::end(),
		}
	}
}

pub fn start_server(client: Arc<BlockChainClient>) {
	thread::spawn(move || {
		let addr = "0.0.0.0:5001".parse().unwrap();

		Server::http(&addr).unwrap().handle(move |_| IpfsHandler {
			client: client.clone(),
			result: None
		}).unwrap();
	});
}
