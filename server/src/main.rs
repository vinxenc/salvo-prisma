use salvo::prelude::*;
// use salvo::logging::Logger;
use salvo::{handler, Depot, Error, FlowCtrl, Handler, Listener, Request, Response, Result, Router, Server};

use std::sync::Arc;
use std::collections::HashMap;

pub mod prisma;

struct SetDB(Arc<prisma::PrismaClient>);
type PrismaClient = std::sync::Arc<prisma::PrismaClient>;

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[async_trait]
impl Handler for SetDB {
  async fn handle(&self, _req: &mut Request, _depot: &mut Depot, _res: &mut Response, _ctrl: &mut FlowCtrl) {
    _depot.inject(self.0.clone());
    _ctrl.call_next(_req, _depot, _res).await;
  }
}

#[handler]
async fn get(depot: &mut Depot, res: &mut Response) -> Result<()> {
    let db = depot.obtain::<PrismaClient>().unwrap();
    let users = db
        .user()
        .find_many(vec![])
        .exec()
        .await
        .map_err(|e| Error::Other(e.to_string().into()))?;
    res.render(Json(users));
    Ok(())
}

#[handler]
async fn post(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let db = depot.obtain::<PrismaClient>().unwrap();
    let user = req.parse_body::<HashMap<String, String>>().await.map_err(|e| {
        tracing::error!("{}", e);
        e
    })?;
    db.user()
        .create(
            user.get("username").unwrap().to_string(),
            user.get("email").unwrap().to_string(),
            vec![],
        )
        .exec()
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            Error::Other(e.to_string().into())
        })?;
    res.render("ok");
    Ok(())
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt().init();

  let prisma_client = Arc::new(prisma::new_client().await.unwrap());

  #[cfg(debug)]
  prisma_client._db_push(false).await.unwrap();

  let router = Router::with_hoop(SetDB(prisma_client))
    // .hoop(Logger::new())
    .get(get)
    .post(post);

 
  let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
  Server::new(acceptor).serve(router).await;
}