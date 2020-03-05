#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use rocket::response::content;
use rocket::request::Form;

use diesel::{prelude::*};

mod schema;

#[database("sqlite_clicks")]
struct ClicksDbConn(diesel::SqliteConnection);

#[derive(Queryable)]
pub struct Click {
    pub id: i32,
}

trait Renderable {
    fn render(&self) -> String;
}

struct IndexPage {
    count: i64,
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
fn index(conn: ClicksDbConn) -> content::Html<String> {
    use self::schema::clicks::dsl::*;
    use diesel::dsl::*;

    let count = clicks.select(count(id)).first(&*conn);

    let page = IndexPage{
        count: count.unwrap()
    };

    content::Html(page.render())
}

#[derive(FromForm)]
struct IndexPost {
    click: String,
}

#[post("/", data = "<form>")]
fn index_post(form: Form<IndexPost>, conn: ClicksDbConn) -> content::Html<String>{
    use self::schema::clicks::dsl::*;
    use diesel::dsl::*;

    diesel::insert_into(clicks).default_values().execute(&*conn).expect("Error saving click.");

    let count = clicks.select(count(id)).first(&*conn);

    let page = IndexPage{
        count: count.unwrap()
    };

    content::Html(page.render())
}

fn main() {
    rocket::ignite()
        .attach(ClicksDbConn::fairing())
        .mount("/", routes![index, index_post]).launch();
}
