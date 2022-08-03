use sqlite::Connection;

fn main() {
    let _db = open_and_initialize_the_db();
    // If we haven't crashed yet, then the db is open and ready
    println!("Hello, world!");
}

fn open_and_initialize_the_db() -> Connection {
    let connection = sqlite::open(":memory:").unwrap();
    connection.execute("CREATE TABLE hello_world (id INTEGER);").unwrap();
    let mut statement = connection.prepare("SELECT * FROM hello_world;").unwrap();
    let result = statement.next().unwrap();
    println!("{:?}", result);
    connection
}
