#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    data: T,
    msg: String,
    count: i64,
}

impl<T> Response<T> {
    /// Returns a new instance of the Server record
    pub fn new<S>(data: T, msg: S, count: i64) -> Self
    where
        S: Into<String>,
    {
        Self {
            data,
            msg: msg.into(),
            count,
        }
    }
}

impl From<&str> for Response<String> {
    fn from(e: &str) -> Self {
        Response::new("".into(), e.to_string(), 1)
    }
}
