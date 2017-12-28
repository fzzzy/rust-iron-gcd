// Example code from Programming Rust

extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    res.set_mut(status::Ok);
    res.set_mut(mime!(Text/Html; Charset=Utf8));
    res.set_mut(r#"
<title>GCD</title>
<form action="/gcd" method="POST">
    <input type="text" name="n" />
    <input type="text" name="n" />
    <button type="submit">Compute GCD</button>
</form>
"#);

    Ok(res)
}

fn post_gcd(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let form_data = match req.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            res.set_mut(status::BadRequest);
            res.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(res);
        }
        Ok(map) => map
    };

    let unparsed = match form_data.get("n") {
        None => {
            res.set_mut(status::BadRequest);
            res.set_mut(format!("Form data has no 'n' parameters\n"));
            return Ok(res);
        }
        Some(n) => n
    };

    let mut nums = Vec::new();
    for u in unparsed {
        match u64::from_str(&u) {
            Err(_) => {
                res.set_mut(status::BadRequest);
                res.set_mut(format!("Value is not a number: {:?}\n", u));
                return Ok(res);
            }
            Ok(n) => { nums.push(n); }
        }
    }

    let mut d = nums[0];
    for m in &nums[1..] {
        d = gcd(d, *m);
    }

    res.set_mut(status::Ok);
    res.set_mut(mime!(Text/Html; Charset=Utf8));
    res.set_mut(format!("The GCD of {:?} is {}\n", nums, d));
    Ok(res)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
