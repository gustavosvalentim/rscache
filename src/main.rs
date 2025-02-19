use rscache::{
    database::{MemoryDatabase, ValueConvert},
    resp::Parser,
};

fn main() {
    // let input = "set key \"string value\"";
    let input =
        "*4\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$44\r\nThis is a multiline\r\nstring stored in Redis.\r\n$2\r\nPX\r\n";
    let mut db = MemoryDatabase::new();
    let mut parser = Parser::new(input);
    let tokens = parser.parse();

    for token in &tokens {
        println!("{:?}", token);
    }

    // let response = dispatch(&mut db, tokens);
    // assert_eq!(response, "+OK".to_string());

    // let value: &String = db.get("mykey").unwrap().to().unwrap();
    // assert_eq!(value, "This is a multiline\r\nstring stored in Redis.");
}
