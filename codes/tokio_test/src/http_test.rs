use http::header::{HeaderMap,HeaderValue,HeaderName};

pub fn http_header_eq_test_main() {
    let mut h1 = HeaderMap::new();
    let mut h2 = HeaderMap::new();
    h1.insert(HeaderName::from_static("123"),HeaderValue::from_static("456"));
    h2.insert(HeaderName::from_static("123"),HeaderValue::from_static("456"));
    assert_eq!(h1,h2);
}

pub fn http_header_ne_test_main() {
    let mut h1 = HeaderMap::new();
    let mut h2 = HeaderMap::new();
    h1.insert(HeaderName::from_static("123"),HeaderValue::from_static("456"));
    h2.insert(HeaderName::from_static("123"),HeaderValue::from_static("4567"));
    assert_eq!(h1,h2);
}
