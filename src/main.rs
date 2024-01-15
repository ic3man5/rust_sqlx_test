use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::{DateTime, Utc};
use std::io::{self, Write};

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    description: String,
    created: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create the connect to the database
    let pool = PgPoolOptions::new()
        .max_connections(15)
        .connect("postgres://test:test@localhost/test")
        .await?;
    // Lets create the user through user input
    print!("Please enter a user name: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read username!");
    print!("Please enter a description for {}: ", name.trim());
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read description!");
    let new_user = User {
        id: 0,
        name,
        description,
        created: Utc::now(),
    };
    sqlx::query!(
        "INSERT INTO users(name, description) VALUES($1, $2);",
        new_user.name.trim(),
        new_user.description.trim()
    )
    .execute(&pool)
    .await?;
    // Fetch all the entries
    let results = sqlx::query_as!(User, "SELECT id, name, description, created FROM users;")
        .fetch_all(&pool)
        .await?;
    // Print all of them now
    for row in &results {
        println!(
            "{}.\tName: {}\n\tDescription: {}\n\tCreated: {}",
            row.id, row.name, row.description, row.created
        );
    }
    println!("Found {} entries!", results.len());

    Ok(())
}
