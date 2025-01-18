use reqwest::Client;
use sqlx::postgres::types::PgPoint;
use tracing::error;

struct CustomService {
    ctx: Client,
    db: PgPoint,
}

// Set up user agent
const USER_AGENT: &str = "Mozilla/5.0 (Linux x86_64; rv:115.0) Gecko/20100101 Firefox/115.0";

#[shuttle_runtime::main]
async fn main(
  #[shuttle_shared_db::Postgres] db: PgPool
) -> Result<CustomService, shuttle_runtime::Error> {
// automatically attempt to do migrations
// we only create the table if it doesn't exist which prevents data wiping
 sqlx::migrate!().run(&db).await.expect("Migrations failed");
// initialise Reqwest client here so we can add it in later on
    let ctx = Client::builder().user_agent(USER_AGENT).build().unwrap();
    Ok(CustomService { ctx, db })
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        scrape(self.ctx, self.db).await.expect("scraping should not finish");
        error!("The web scraper loop shouldn't finish!");
        Ok(())
    }
}

