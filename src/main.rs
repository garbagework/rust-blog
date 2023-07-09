use warp::{Filter, Rejection, Reply};
use tera::Context;
use lazy_static::lazy_static;
use tokio::fs;

lazy_static! {
    static ref TEMPLATES: tera::Tera = {
        let mut tera = tera::Tera::new("templates/**/*").unwrap();
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

#[tokio::main]
async fn main() {
    let index_route = warp::path::end().and_then(index_handler);
    let post_route = warp::path("post").and_then(post_handler);
    let add_post_route = warp::path("add").and(warp::post()).and_then(add_post_handler);

    let routes = index_route.or(post_route).or(add_post_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn index_handler() -> Result<impl Reply, Rejection> {
    let mut context = Context::new();
    let posts = fetch_posts().await;
    let post_ids: Vec<_> = posts.iter().enumerate().map(|(i, _)| i.to_string()).collect();
    
    context.insert("posts", &posts);
    context.insert("post_ids", &post_ids);

    let rendered = TEMPLATES.render("index.html", &context).map_err(|e| {
        eprintln!("Failed to render 'index.html': {}", e);
        warp::reject::custom(e)
    })?;

    Ok(warp::reply::html(rendered))
}



async fn post_handler() -> Result<impl Reply, Rejection> {
    let mut context = Context::new();
    let post = fetch_posts().await;
    context.insert("posts", &post);

    let rendered = TEMPLATES.render("post.html", &context).unwrap();
    Ok(warp::reply::html(rendered))
}


async fn add_post_handler() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::html("Add post handler"))
}

async fn fetch_posts() -> Vec<String> {
    vec!["Post 1".to_string(), "Post 2".to_string()]
}
