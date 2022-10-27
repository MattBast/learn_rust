/// Documentation about the warp server

// #![deny(warnings)]
use warp::Filter;

/// Start a web server and listen for requests on all endpoints
#[tokio::main]
pub async fn start() {
    let routes = warp::any().map(|| "Hello, World!");
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
