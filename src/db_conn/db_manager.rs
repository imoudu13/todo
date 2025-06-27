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

pub async fn remove_task(id: i32) -> Result<i32, Error> {
    match get_client().await {
        Ok(client) => {
            client.execute("DELETE FROM tasks_todo WHERE id = $1;", &[&id]).await?;
            Ok(id)
        },
        Err(e) => {
            eprintln!("error getting client: {}", e);
            Err(e)
        }
    }
}

pub async fn mark_done(id: i32) -> Result<i32, Error> {
    match get_client().await {
        Ok(client) => {
            let rows = client
                .query("SELECT title, description FROM tasks_todo WHERE id = $1;", &[&id])
                .await?;

            if rows.is_empty() {
                eprintln!("No task found with id {}", id);
                return Ok(0)
            }

            client.query("DELETE FROM tasks_todo WHERE id = $1;", &[&id]).await?;

            let title: &str = rows[0].get(0);
            let description: &str = rows[0].get(1);
            client.execute(
                "INSERT INTO tasks_complete (title, description) VALUES ($1, $2);",
                &[&title, &description]).await?;

            Ok(id)
        },
        Err(e) => {
            eprintln!("error getting client: {}", e);
            Err(e)
        }
    }
}
