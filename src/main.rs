extern crate iron;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;

fn main() {
    println!("Servidor escuchando en http://localhost:3000");

    Iron::new(get_welcome).http("localhost:3000").unwrap();
}

fn get_welcome(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let content_type = "text/html".parse::<Mime>().unwrap();

    response.set_mut(status::Ok);
    response.set_mut(content_type);

    response.set_mut(
        r#"
        <title>Mi primer webserver en Rust</title>
        <h1>Bienvenidos a mi webserver en rust :)</h1>
        "#,
    );

    Ok(response)
}
