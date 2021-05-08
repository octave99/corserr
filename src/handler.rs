use crate::{Error, Response, Result};

// Backend Model for working API
fn working_model() -> Result<String> {
    Ok(String::from("Hello World!"))
}

// Handler for working API
pub async fn working() -> Result<impl warp::Reply> {
    let resp_msg = working_model()?;

    let resp = Response::new(resp_msg, "Records fetched successfully", 1);

    Ok(warp::reply::json(&resp))
}

// Backend Model for not_working API
fn not_working_model() -> Result<String> {
    Err(warp::reject::custom(Error::BadReq(String::from(
        "Some error occurred",
    ))))
}

// Handler for not_working API
pub async fn not_working() -> Result<impl warp::Reply> {
    let resp_msg = not_working_model()?;

    let resp = Response::new(resp_msg, "Records fetched successfully", 1);

    Ok(warp::reply::json(&resp))
}
