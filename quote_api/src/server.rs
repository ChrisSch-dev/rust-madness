use warp::Filter;
use rand::seq::SliceRandom;

pub async fn run() {
    let quotes = vec![
        "The only way to do great work is to love what you do. – Steve Jobs",
        "You miss 100% of the shots you don’t take. – Wayne Gretzky",
        "Stay hungry, stay foolish. – Steve Jobs",
        "Whether you think you can or you think you can’t, you’re right. – Henry Ford",
    ];

    let quotes_filter = warp::any().map(move || {
        let quote = quotes.choose(&mut rand::thread_rng()).unwrap();
        warp::reply::json(&serde_json::json!({ "quote": quote }))
    });

    println!("Quote API running at http://127.0.0.1:3030/quote");
    warp::serve(warp::path("quote").and(quotes_filter))
        .run(([127, 0, 0, 1], 3030))
        .await;
}