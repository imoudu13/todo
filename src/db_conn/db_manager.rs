use tokio_postgres::{NoTls, Error};

pub async fn add_task(title: &str, description: &str) -> Result<u64, Error> {
    println!("pre");
    let (client, connection)
        = tokio_postgres::connect("host=localhost user=postgres dbname=todo", NoTls).await?;

    println!("conn");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("no error");

    let rows: u64 = client.execute(
        "INSERT INTO tasks_todo (title, description) VALUES ($1, $2);",
        &[&title, &description]).await?;

    println!("inserted");
    Ok(rows)
}
