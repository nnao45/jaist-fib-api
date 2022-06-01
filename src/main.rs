use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FibParams {
    n: usize,
}

#[get("/hc")]
async fn hc() -> impl Responder {
    "OK"
}

fn fib_exec(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_exec(n - 2) + fib_exec(n - 1)
    }
}

#[get("/fib")]
async fn  fib(param: web::Query<FibParams>) -> impl Responder {
    fib_exec(param.n).to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(hc)
        .service(fib))
        .bind("0.0.0.0:8081")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use crate::fib_exec;

    #[test]
    fn it_works() {
        assert_eq!(fib_exec(10), 55);
        assert_eq!(fib_exec(20), 6765);
        assert_eq!(fib_exec(27), 196418);
        assert_eq!(fib_exec(30), 832040);
        assert_eq!(fib_exec(40), 102334155);
    }
}