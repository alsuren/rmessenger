extern crate core;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use self::futures::future::Future;
use self::futures::Stream;
use self::hyper::header::ContentType;
use self::hyper::mime::APPLICATION_JSON;
use self::hyper::Post;
use self::hyper::client::Request;

pub struct UrlRequest {}

impl UrlRequest {
    pub fn new() -> UrlRequest {
        UrlRequest {}
    }

    pub fn post(
        self,
        client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
        url: String,
        data: String,
        body: String,
    ) -> Box<Future<Item = String, Error = hyper::Error>> {
        let request_url = format!("{}{}{}", url, "?", data).parse().unwrap();
        let mut request = Request::new(Post, request_url);
        request.headers_mut().set(ContentType(APPLICATION_JSON));
        request.set_body(body.to_owned());

        let fut = client
            .request(request)
            .and_then(|res| {
                res.body().fold(Vec::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, hyper::Error>(acc)
                })
            })
            .map(|vec| String::from_utf8(vec).unwrap());
        Box::new(fut)
    }
}
