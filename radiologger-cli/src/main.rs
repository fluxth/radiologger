mod backend;

fn test<Connection>(_conn: Connection)
where
    Connection: backend::Connection,
{
}

fn main() {
    println!("Hello, world!");
}
