#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::content;
use rocket::request::Form;

trait Renderable {
    fn render(&self) -> String;
}

struct IndexPage {
    count: u64,
}

impl Renderable for IndexPage {
    fn render(&self) -> String {
        let template = "
        Hello,<br />

        The button has been clicked %COUNT% times.<br />

        <form action=\"/\" method=\"post\">
            <input type=\"submit\" name=\"click\" value=\"click\">
        </form>
        ";

        template.replace("%COUNT%", &self.count.to_string())
    }
}


#[get("/")]
fn index() -> content::Html<String> {
    let page = IndexPage{
        count: 0
    };

    // TODO: load

    content::Html(page.render())
}

#[derive(FromForm)]
struct IndexPost {
    click: String,
}

#[post("/", data = "<form>")]
fn index_post(form: Form<IndexPost>) -> content::Html<String>{
    let page = IndexPage{
        count: 0
    };

    // TODO increment and store

    content::Html(page.render())
}

fn main() {
    rocket::ignite().mount("/", routes![index, index_post]).launch();
}
