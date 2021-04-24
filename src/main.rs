use actix_web::{get, web, App, HttpServer, Responder};
use serde_json::Value;
use serde_json::json;
use thesaurus::{Thesaurus, WordType};

#[get("/{word}/all")]
async fn all(web::Path(word): web::Path<String>) -> impl Responder {
    let synonyms = match Thesaurus::synonym(&word, None) {
        Ok(synonyms) => synonyms,
        Err(error) => return json!({"error": error.to_string()}).to_string(),
    };

    let mut values: Vec<Value> = Vec::new();

    for synonym in synonyms.words {
        values.push(Value::String(synonym.name));
    };

    let value = Value::Array(values);

    format!("{}", value)
}

#[get("/{word}/verb")]
async fn verb(web::Path(word): web::Path<String>) -> impl Responder {
    let synonyms = match thesaurus::Thesaurus::synonym(&word, Some(WordType::Verb)) {
        Ok(synonyms) => synonyms,
        Err(error) => return json!({"error": error.to_string()}).to_string(),
    };

    let mut values: Vec<Value> = Vec::new();

    for synonym in synonyms.words {
        values.push(Value::String(synonym.name));
    };

    let value = Value::Array(values);

    format!("{}", value)
}

#[get("/{word}/noun")]
async fn noun(web::Path(word): web::Path<String>) -> impl Responder {
    let synonyms = match thesaurus::Thesaurus::synonym(&word, Some(WordType::Noun)) {
        Ok(synonyms) => synonyms,
        Err(error) => return json!({"error": error.to_string()}).to_string(),
    };

    let mut values: Vec<Value> = Vec::new();

    for synonym in synonyms.words {
        values.push(Value::String(synonym.name));
    };

    let value = Value::Array(values);

    format!("{}", value)
}

#[get("/{word}/adjective")]
async fn adjective(web::Path(word): web::Path<String>) -> impl Responder {
    let synonyms = match thesaurus::Thesaurus::synonym(&word, Some(WordType::Adjective)) {
        Ok(synonyms) => synonyms,
        Err(error) => return json!({"error": error.to_string()}).to_string(),
    };

    let mut values: Vec<Value> = Vec::new();

    for synonym in synonyms.words {
        values.push(Value::String(synonym.name));
    };

    let value = Value::Array(values);

    format!("{}", value)
}

#[get("/{word}/adverb")]
async fn adverb(web::Path(word): web::Path<String>) -> impl Responder {
    let synonyms = match thesaurus::Thesaurus::synonym(&word, Some(WordType::Adverb)) {
        Ok(synonyms) => synonyms,
        Err(error) => return json!({"error": error.to_string()}).to_string(),
    };

    let mut values: Vec<Value> = Vec::new();

    for synonym in synonyms.words {
        values.push(Value::String(synonym.name));
    };

    let value = Value::Array(values);

    format!("{}", value)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
            .service(all)
            .service(verb)
            .service(noun)
            .service(adjective)
            .service(adverb)
        )
        .bind("127.0.0.1:2021")?
        .run()
        .await
}