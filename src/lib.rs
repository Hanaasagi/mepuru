#![feature(specialization)]
#[allow(unused_imports)]

#[macro_use]
extern crate pyo3;
extern crate url;

use pyo3::prelude::*;
use std::collections::HashMap;
//use pyo3::Objectprotocol;

struct ParseError {
    error: url::ParseError
}

//impl From<url::ParseError> for PyErr {

//}


impl From<ParseError> for PyErr {
    fn from (e: ParseError) -> PyErr {
        PyErr::new::<pyo3::exc::ValueError, _>(format!("{}", e.error))
    }
}

#[pyclass]
struct ParseResult {
    url: url::Url
}

impl ParseResult {
    fn new(url: &str) -> Result<Self, ParseError> {
        let u = url::Url::parse(url);
        match u {
            Err(error) => Err(ParseError{ error }),
            Ok(url) => Ok(ParseResult{ url })
        }
        //Ok(Self { url: u })
    }
}

#[pymethods]
impl ParseResult {
    #[new]
    fn __new__(obj: &PyRawObject, url: &str) -> PyResult<()> {
        let r = ParseResult::new(url)?;
        obj.init(|_| r)
    }

    #[getter(scheme)]
    fn get_scheme(&self) -> PyResult<&str> {
        Ok(self.url.scheme())
    }


    /// Return whether the URL has an 'authority',
    /// which can contain a username, password, host, and port number.
    ///
    /// URLs that do *not* are either path-only like `unix:/run/foo.socket`
    /// or cannot-be-a-base like `data:text/plain,Stuff`.
    fn has_authority(&self) -> bool {
        self.url.has_authority()
    }


    /// Return the username for this URL (typically the empty string)
    /// as a percent-encoded ASCII string.
    #[getter(username)]
    fn username(&self) -> PyResult<&str> {
        Ok(self.url.username())
    }


    /// Return the password for this URL, if any, as a percent-encoded ASCII string.
    #[getter(password)]
    fn password(&self) -> PyResult<Option<&str>> {
        Ok(self.url.password())
    }


    /// Return whether the URL has an 'host',
    fn has_host(&self) -> bool {
        self.url.has_host()
    }


    /// Return the string representation of the host (domain or IP address) for this URL, if any.
    ///
    /// Non-ASCII domains are punycode-encoded per IDNA.
    /// IPv6 addresses are given between `[` and `]` brackets.
    ///
    /// Cannot-be-a-base URLs (typical of `data:` and `mailto:`) and some `file:` URLs
    /// don’t have a host.
    #[getter(host)]
    fn get_host(&self) -> PyResult<Option<&str>> {
        Ok(self.url.host_str())
    }


    /// If this URL has a host and it is a domain name (not an IP address), return it.
    #[getter(domain)]
    fn get_domain(&self) -> PyResult<Option<&str>> {
        Ok(self.url.domain())
    }


    /// Return the port number for this URL, if any.
    #[getter(port)]
    fn get_port(&self) -> PyResult<Option<u16>> {
        Ok(self.url.port())
    }

    /// Return the port number for this URL, or the default port number if it is known.
    ///
    /// This method only knows the default port number
    /// of the `http`, `https`, `ws`, `wss`, `ftp`, and `gopher` schemes.
    fn port_or_known_default(&self) -> Option<u16> {
        self.url.port_or_known_default()
    }

    //fn with_default_port(&self, ) -> {

    //}

    /// Return the path for this URL, as a percent-encoded ASCII string.
    /// For cannot-be-a-base URLs, this is an arbitrary string that doesn’t start with '/'.
    /// For other URLs, this starts with a '/' slash
    /// and continues with slash-separated path segments.
    #[getter(path)]
    fn get_path(&self) -> PyResult<&str> {
        Ok(self.url.path())
    }

    /// Unless this URL is cannot-be-a-base,
    /// return an list of '/' slash-separated path segments,
    /// each as a percent-encoded ASCII string.
    fn path_segments(&self) -> PyResult<Option<Vec<&str>>> {
        match self.url.path_segments() {
            Some(r) => Ok(Some(r.collect::<Vec<&str>>())),
            None => Ok(None)
        }
    }


    /// Return this URL’s query string, if any, as a percent-encoded ASCII string.
    #[getter(query)]
    fn get_query(&self) -> PyResult<&str> {
        Ok(self.url.query().unwrap_or(""))
    }


    /// Change this URL’s query string.
    #[setter(query)]
    fn set_query(&mut self, query: Option<&str>) -> PyResult<()> {
        self.url.set_query(query);
        Ok(())
    }


    /// Parse the URL’s query string, if any, as `application/x-www-form-urlencoded`
    /// and return an HashMap of (key, value) pairs.
    fn get_query_pairs(&self) -> HashMap<String, String> {
        let pairs = self.url.query_pairs().into_owned();
        let mut result: HashMap<String, String> = HashMap::new();
        for pair in pairs {
            let (k, v) = pair;
            result.insert(k, v);
        }
        result
    }

    // HashMap does not impl FromPyObject trait
    //#[setter(query_pairs)]
    //fn set_query_pairs(&mut self, pairs: HashMap<String, String>) -> PyResult<()> {
        ////for (k, v) in pairs {
            ////self.url.query_pairs_mut().clear().append_pair(&k, &v);
        ////}
        //Ok(())
    //}


    /// Return this URL’s fragment identifier, if any.
    ///
    /// A fragment is the part of the URL after the `#` symbol.
    /// The fragment is optional and, if present, contains a fragment identifier
    /// that identifies a secondary resource, such as a section heading
    /// of a document.
    ///
    /// In HTML, the fragment identifier is usually the id attribute of a an element
    /// that is scrolled to on load. Browsers typically will not send the fragment portion
    /// of a URL to the server.
    ///
    /// **Note:** the parser did *not* percent-encode this component,
    /// but the input may have been percent-encoded already.
    #[getter(fragment)]
    fn get_fragment(&self) -> PyResult<&str> {
        Ok(self.url.fragment().unwrap_or(""))
    }


    /// Change this URL’s fragment identifier.
    #[setter(fragment)]
    fn set_fragment(&mut self, fragment: Option<&str>) -> PyResult<()> {
        self.url.set_fragment(fragment);
        Ok(())
    }
}

#[pymodinit]
fn mepuru(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ParseResult>()?;
    Ok(())
}
