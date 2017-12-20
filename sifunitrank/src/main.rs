#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rusqlite;
extern crate r2d2;
extern crate r2d2_sqlite;

mod db;

#[get("/")]
fn index(conn: db::Conn) -> String {
    //let mut stmt = conn.prepare("SELECT * from unit_skill_m WHERE skill_effect_type = 11 AND (trigger_type = 3 OR trigger_type = 4 OR trigger_type = 6) ORDER BY unit_skill_id").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM unit_m WHERE (unit_type_id <= 9 OR (unit_type_id >= 101 AND unit_type_id <= 109)) AND max_removable_skill_capacity > 2 ORDER BY unit_number").unwrap();
    let mut rows = stmt.query(&[]).unwrap();
    let mut output: String = String::new();
    while let Some(result_row) = rows.next() {
        let row = result_row.unwrap();
        //let trigger: u8 = row.get(6);
        let subtitle: String = row.get(3);
        let name: String = row.get(4);
        output.push_str(&(format!("{} - {}\n", subtitle, name)));
    }
    output
}

fn main() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![index])
        .launch();
}
