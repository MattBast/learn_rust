/// Documentation about the warp server

// #![deny(warnings)]

use serde_derive::{Deserialize, Serialize};
use warp::Filter;


#[derive(Deserialize, Serialize)]
struct Response {
    message: String,
    status: u32,
}

/// Start a web server and listen for requests on all endpoints
#[tokio::main]
pub async fn start() {
    // GET /
    let root = warp::path::end().map(|| "Ping");

    // POST /event  {"type":"page_view"}
    let event = warp::post()
        .and(warp::path("event"))
        .and(warp::body::json())
        .map(|mut resp: Response| {
            resp.status = 200;
            resp.message = String::from("hello world!");
            warp::reply::json(&resp)
        });

    // POST /employees/:rate  {"name":"Sean","rate":2}
    // let promote = warp::post()
    //     .and(warp::path("employees"))
    //     .and(warp::path::param::<u32>())
    //     // Only accept bodies smaller than 16kb...
    //     .and(warp::body::content_length_limit(1024 * 16))
    //     .and(warp::body::json())
    //     .map(|rate, mut employee: Employee| {
    //         employee.rate = rate;
    //         warp::reply::json(&employee)
    //     });

    // combine the routes/endpoints into a single api
    let routes = root
        .or(event);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
