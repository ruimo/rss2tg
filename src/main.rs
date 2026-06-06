use feed_rs::parser;
use rusqlite::{params, Connection};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_TOKEN")?;
    let chat_id = env::var("CHAT_ID")?;
    
    // キャッシュしたDBファイルを開く
    let conn = Connection::open("rss_data.db")?;
    conn.execute("CREATE TABLE IF NOT EXISTS history (id TEXT PRIMARY KEY)", [])?;

    let rss_url = "https://example.com/rss"; // 監視対象
    let content = reqwest::get(rss_url).await?.text().await?;
    let feed = parser::parse(content.as_bytes())?;

    for entry in feed.entries.iter().take(5) {
        let entry_id = entry.id.clone();
        
        // 未通知なら通知する
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM history WHERE id=?)",
            params![entry_id],
            |row| row.get(0),
        )?;

        if !exists {
            let msg = format!("📰 {}\n\n{}", entry.title.as_ref().unwrap().content, entry.links[0].href);
            let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}", 
                              token, chat_id, urlencoding::encode(&msg));
            reqwest::get(url).await?;

            conn.execute("INSERT INTO history (id) VALUES (?)", params![entry_id])?;
        }
    }
    Ok(())
}
