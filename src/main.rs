use rscache::{
    database::{MemoryDatabase, ValueConvert},
    resp::{dispatch, Parser},
};

fn main() {
    let input = "set key \"string value\"".to_string();
    let mut db = MemoryDatabase::new();
    let mut parser = Parser::new(input);
    let tokens = parser.parse();

    let response = dispatch(&mut db, tokens);
    assert_eq!(response, "+OK".to_string());

    let value: &String = db.get("key").unwrap().to().unwrap();
    assert_eq!(value, "string value");
}
