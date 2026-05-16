use csv;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Read;

mod api;

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    api::fetch_api();

    if let Err(e) = read_from_file("./agreement.csv") {
        eprintln!("{}", e);
    }

    let new_article: Article = Article {
        article: String::from("How to write rust"),
        author: String::from("shaji"),
        paragraph: vec![
            Paragraph {
                name: String::from("macro"),
            },
            Paragraph {
                name: String::from("micro"),
            },
        ],
    };

    let new_json = serde_json::to_string(&new_article).unwrap();

    println!("The created json is {:?}", new_json);

    let json = r#"
    {
    "article":"how to learn rust",
    "author":"hrishik",
    "paragraph":[
    {
    "name":"one"
    },
    {
    "name":"two"
    }
    ]
    }
    "#;

    let parsed: Article = read_json_typed(json);

    println!(
        "\n\n The name of first paragraph is :{}",
        parsed.paragraph[0].name
    );
}

fn read_json_typed(raw_json: &str) -> Article {
    serde_json::from_str(raw_json).unwrap()
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
}
