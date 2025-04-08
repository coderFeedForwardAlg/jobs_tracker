

// https://xata.io/docs/rust-sqlx
use sqlx::FromRow;
use sqlx::{Connection, PgConnection};
#[tokio::main]
async fn main()  -> Result<(), sqlx::Error> {
    println!("Hello, world!");
    let db_url = "postgres://max:password@localhost:1234/jobs";
    let mut pool = PgConnection::connect(db_url).await?;

    let table = "users";
    let name = "bob";
    let email = "bob@gmail.com";
    let hash = "aaaa";

    let query = format!("INSERT INTO {} (username, email, password_hash) VALUSE ($1, $2)", table);

    let _res: (i32,) = sqlx::query_as(&query)
        .bind(name)
        .bind(email)
        .bind(hash)
        .fetch_one(&mut pool)
        .await?;
    // println!("Rows affected: {}", res.rows_affected());
//    let see_data_query = format!("SELECT * FROM USERS)");
//    let see_data = sqlx::query(&see_data_query)
//        .execute(&pool);
//    println!("data was {}", see_data);

    pool.close().await;
    Ok(())
    
}
// password = password 
// user = user
