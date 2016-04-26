
// A simple example shows how to access MySQL database in rust.
// You can use below environment variable for database connetion parameters.
/*
export MYSQL_SERVER_USER=
export MYSQL_SERVER_PASS=
export MYSQL_SERVER_HOST=
export MYSQL_SERVER_PORT=
export MYSQL_SERVER_DB_NAME=
*/

extern crate mysql;
use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;

// [Toad for MySQL](http://www.toadworld.com/products/toad-for-mysql)
// [MySQL commands](http://www.pantz.org/software/mysql/mysqlcommands.html)
// [Amazon RDS](http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_ConnectToInstance.html)

// Example of MySQL commands:
//  http://blog.csdn.net/jonathan_q_bo/article/details/1506325
// mysql> SHOW DATABASES;
// +----------+
// | Database |
// +----------+
// | mysql    |
// | test     |
// | tmp      |
// +----------+
// mysql> USE test
// Database changed
// mysql> SHOW TABLES;
// +---------------------+
// | Tables in menagerie |
// +---------------------+
// | pet                 |
// +---------------------+
// mysql> DESCRIBE pet;
// +---------+-------------+------+-----+---------+-------+
// | Field   | Type        | Null | Key | Default | Extra |
// +---------+-------------+------+-----+---------+-------+
// | name    | varchar(20) | YES  |     | NULL    |       |
// | owner   | varchar(20) | YES  |     | NULL    |       |
// | species | varchar(20) | YES  |     | NULL    |       |
// | sex     | char(1)     | YES  |     | NULL    |       |
// | birth   | date        | YES  |     | NULL    |       |
// | death   | date        | YES  |     | NULL    |       |
// +---------+-------------+------+-----+---------+-------+
// mysql> SELECT * FROM pet;
// +----------+--------+---------+------+------------+------------+
// | name     | owner  | species | sex  | birth      | death      |
// +----------+--------+---------+------+------------+------------+
// | Fluffy   | Harold | cat     | f    | 1993-02-04 | NULL       |
// | Claws    | Gwen   | cat     | m    | 1994-03-17 | NULL       |
// | Buffy    | Harold | dog     | f    | 1989-05-13 | NULL       |
// | Fang     | Benny  | dog     | m    | 1990-08-27 | NULL       |
// | Bowser   | Diane  | dog     | m    | 1998-08-31 | 1995-07-29 |
// | Chirpy   | Gwen   | bird    | f    | 1998-09-11 | NULL       |
// | Whistler | Gwen   | bird    | NULL | 1997-12-09 | NULL       |
// | Slim     | Benny  | snake   | m    | 1996-04-29 | NULL       |
// | Puffball | Diane  | hamster | f    | 1999-03-30 | NULL       |
// +----------+--------+---------+------+------------+------------+


fn execute(pool : &MyPool, query: &str) {
    let query_result = pool.prep_exec(query, ()).unwrap();
    for myresult in query_result {
        match myresult {
            Ok(ref values) => {
                for i in 0..values.len() {
                    println!("  {}", values[i].into_str());
                }
            },
            Err(e) => println!("  {}", e),
        }
    }
}


fn main() {

    let host: String = ::std::env::var("MYSQL_SERVER_HOST").unwrap_or("127.0.0.1".to_string());
    let port: u16 = ::std::env::var("MYSQL_SERVER_PORT").ok()
                                   .map(|my_port| my_port.parse::<u16>().ok().unwrap_or(3306))
                                   .unwrap_or(3307);
    let db: String = ::std::env::var("MYSQL_SERVER_DB_NAME").unwrap_or("test".to_string());
    let user: String = ::std::env::var("MYSQL_SERVER_USER").unwrap_or("root".to_string());
    let pwd: String = ::std::env::var("MYSQL_SERVER_PASS").unwrap_or("password".to_string());

    println!("  host:{}\n  db:{}\n  port:{}\n  user/pwd:{}/{} ", host, db, port, user, pwd);

    let opts = MyOpts {
        user: Some(user),
        pass: Some(pwd),
        tcp_addr: Some(host),
        tcp_port: port,
        db_name: Some(db),
        ..Default::default()
    };

    let pool = MyPool::new(opts).unwrap();
    println!("\nDB connectons is established.");

    println!("\ndatabases: ");
    execute(&pool, "SHOW DATABASES");

    println!("\nLet is go for everything is done.");
}