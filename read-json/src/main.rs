use serde::{de::Error, Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph{
    name: String
}
fn main() {
    let json =  r#"
    {
        "article": "how to work with json in rust",
        "author": "Williams",
        "paragraph": [
            {
                "name": "starting paragraph"
            },
            {
                "name": "body of the paragraph"
            },
            {
                "name": "end of the paragraph"
            }
        ]
    }
    "#;

    let article = Article {
        article: "How to work with json in rust".into(),
        author: "williams".into(),
        paragraph: vec![
            Paragraph{ name: "first sentence".into() },
            Paragraph{ name: "second sentence".into() },
            Paragraph{ name: "last sentence".into() },
        ]
    };

    match read_json_typed(json){
        Ok(parsed) => {
            println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[0].name);
        }
        Err(e) => {
            eprintln!("Failed to parse json: {}", e);
        }
    }

    match serde_json::to_string(&article) {
        Ok(parsed) => {println!("{parsed}");},
        Err(e) => {
            eprintln!("Failed to parse article: {}", e);
        }
    }
}

fn read_json_typed(raw_json: &str) -> Result<Article, serde_json::Error> {
    serde_json::from_str(raw_json)
}

// fn to_article(article: Article)