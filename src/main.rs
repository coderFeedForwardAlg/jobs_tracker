

// https://xata.io/docs/rust-sqlx
use sqlx::FromRow;
use sqlx::{Connection, PgConnection};
use std::io;


#[tokio::main]
async fn main()  -> Result<(), sqlx::Error> {
    println!("Hello, world!");
    let db_url = "postgres://max:password@localhost:5432/jobs";
    let mut pool = PgConnection::connect(db_url).await?;

    let table = "users";
    let name = "bob3";
    let email = "bobyy@gmail.com";
    let hash = "aaaaa";

    let query = format!("INSERT INTO {} (username, email, password_hash) VALUES($1, $2, $3) RETURNING email", table);

    #[derive(Debug, FromRow)] 
    struct UserRow {
        username: String,
        email: String,
        password_hash: String
    }

    impl UserRow {
        pub fn as_vec(&self) -> [String; 3] {
            [self.username.clone(), self.email.clone(), self.password_hash.clone()]
        }
    }
    let questions = vec!["whats you username", "what your email", "whats you password"];
    let mut answers = Vec::new();
    let mut new_row = UserRow {username: String::new(), email: String::new(), password_hash: String::new()};
    for info in questions {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("cant read lined");
        answers.push(user_input);
    }
    new_row.username = answers[0].clone();
    new_row.email = answers[1].clone();
    new_row.password_hash = answers[2].clone();
    
    // let Vec<String> // make vecter of strings and bind each one into the querey (use loop to get
    // all infor for querey add push to vec) 
    
    let _res: (i32,) = sqlx::query_as(&query)
        .bind(new_row.username)
        .bind(new_row.email)
        .bind(new_row.password_hash)
        .fetch_one(&mut pool)
        .await?;


    let select_query = sqlx::query_as::<_, UserRow>("SELECT * FROM Users");
    let user_rows: Vec<UserRow> = select_query.fetch_all(&mut pool).await?;

    println!("got rows: {:?}", user_rows);

    pool.close().await;
    Ok(())
    
}
// password = password 
// user = user
