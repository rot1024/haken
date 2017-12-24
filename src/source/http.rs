use std::fmt;
use std::error::Error as StdError;
use hyper;
use hyper_tls;
use futures::*;

#[derive(Debug)]
pub enum Error {
    StatusCode(u16),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            &Error::StatusCode(_) => "invalid status code",
            // _ => "invalid error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self {
            _ => None,
        }
    }
}

pub fn get(url: &str) -> Result<String, Box<StdError>> {
    let u = url.parse::<hyper::Uri>();
    if u.is_err() {
        return Err(Box::new(u.unwrap_err()));
    }
    let u = u.unwrap();

    let mut core = ::tokio_core::reactor::Core::new().unwrap();

    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(1, &core.handle()).unwrap())
        .build(&core.handle());

    let r = core.run(
        client
            .get(u)
            .map_err(|e| to_boxed_std_error(e))
            .and_then(|res| {
                if res.status() != hyper::StatusCode::Ok {
                    // let err: Box<StdError> = Box::new(Error::StatusCode(res.status().as_u16()));
                    // return future::err::<String, Box<StdError>>(err);
                }
                res.body()
                    .concat2()
                    .map_err(|e| to_boxed_std_error(e))
                    .map(|c| String::from_utf8_lossy(&c.to_vec()).to_string())
            }),
    );
    if r.is_err() {
        return Err(r.unwrap_err());
    }
    Ok(r.unwrap())
}

fn to_boxed_std_error(e: hyper::Error) -> Box<StdError> {
    Box::new(e)
}
