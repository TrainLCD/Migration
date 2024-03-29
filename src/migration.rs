use std::{
    env,
    fs::File,
    process::{Command, Stdio},
};

pub fn insert_data(generated_sql_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let generated_sql_file = File::open(generated_sql_path)?;
    let disable_memcached_flush: bool = env::var("DISABLE_MEMCACHED_FLUSH")?
        .parse()
        .unwrap_or(false);

    Command::new("mysql")
        .arg(format!("-u{}", env::var("MYSQL_USER").unwrap()))
        .arg(format!("-p{}", env::var("MYSQL_PASSWORD").unwrap()))
        .arg(format!("-h{}", env::var("MYSQL_HOST").unwrap()))
        .arg("--default-character-set=utf8mb4")
        .arg("-e")
        .arg(format!(
            "CREATE DATABASE IF NOT EXISTS {}",
            env::var("MYSQL_DATABASE").unwrap()
        ))
        .spawn()
        .expect("Failed to create database.")
        .wait()?;

    Command::new("mysql")
        .arg(format!("-u{}", env::var("MYSQL_USER").unwrap()))
        .arg(format!("-p{}", env::var("MYSQL_PASSWORD").unwrap()))
        .arg(format!("-h{}", env::var("MYSQL_HOST").unwrap()))
        .arg("--default-character-set=utf8mb4")
        .arg(env::var("MYSQL_DATABASE").unwrap())
        .stdin(Stdio::from(generated_sql_file))
        .spawn()
        .expect("Failed to insert.")
        .wait()?;

    if !disable_memcached_flush {
        let memcached_url = env::var("MEMCACHED_URL")?;
        let cache_client = memcache::connect(memcached_url)?;
        cache_client.flush()?;
    }

    Ok(())
}
