use tokio_postgres::{NoTls, Error, Row, Client};

async fn get_client() -> Result<Client, Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=todo", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn add_task(title: &str, description: &str) -> Result<u64, Error> {
    match get_client().await {
        Ok(client) => {
            let rows: u64 = client.execute(
                "INSERT INTO tasks_todo (title, description) VALUES ($1, $2);",
                &[&title, &description]).await?;
            Ok(rows)
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

pub async fn list_tasks() -> Result<Vec<Row>, Error> {
    match get_client().await {
        Ok(client) => {
            let results = client.query(
                "SELECT * FROM tasks_todo;",
                &[]).await?;
            Ok(results)
        }
        Err(e) => {
            eprintln!("error getting client: {}", e);
            Err(e)
        }
    }
}
