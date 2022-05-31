#[macro_use] extern crate rocket;
use rocket::Config;
use rocket::config::LogLevel;

#[get("/hc")]
fn hc() -> &'static str {
    "OK"
}

fn fib_exec(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_exec(n - 2) + fib_exec(n - 1)
    }
}

#[get("/fib?<n>")]
fn fib(n: usize) -> String {
    fib_exec(n).to_string()
}

#[launch]
fn rocket() -> _ {
    let mut config = Config::default();
    config.port = 8081;
    config.log_level = LogLevel::Critical;
    rocket::build()
        .mount("/", routes![hc])
        .mount("/", routes![fib])
        .configure(config)
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