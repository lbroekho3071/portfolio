use warp::Filter;

#[tokio::main]
async fn main()
{
    let index = warp::path::end().and(warp::fs::file("www/static/index.html"));

    let assets = warp::path("static").and(warp::fs::dir("www/static"));

    warp::serve(index.or(assets))
        .run(([127, 0, 0, 1], 3000))
        .await;
}
