

// https://xata.io/docs/rust-sqlx
use sqlx::FromRow;
use sqlx::{Connection, PgConnection};
#[tokio::main]
async fn main()  -> Result<(), sqlx::Error> {
    println!("Hello, world!");
    let db_url = "postgres://max:password@localhost:5432/jobs";
    let mut pool = PgConnection::connect(db_url).await?;

    let table = "users";
    let name = "bob3";
    let email = "boby@gmail.com";
    let hash = "aaaaa";

    let query = format!("INSERT INTO {} (username, email, password_hash) VALUES($1, $2, $3) RETURNING email", table);

    let _res: (i32,) = sqlx::query_as(&query)
        .bind(name)
        .bind(email)
        .bind(hash)
        .fetch_one(&mut pool)
        .await?;

    #[derive(Debug, FromRow)] 
    struct UserRow {
        username: String,
        email: String,
        password_hash: String

    }

    let select_query = sqlx::query_as::<_, UserRow>("SELECT * FROM Users");
    let user_rows: Vec<UserRow> = select_query.fetch_all(&mut pool).await?;

    println!("got rows: {:?}", user_rows);

    pool.close().await;
    Ok(())
    
}
// password = password 
// user = user
