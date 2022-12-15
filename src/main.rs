use std::{str::FromStr, string::ParseError};

use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use payloads::{
    issue::IssuePayload, issue_comment::CommentIssuePayload, push::PushPayload,
    repository::RepositoryPayload,
};
use telegram::bot::post_chat_message;

use crate::telegram::messages::*;

mod payloads;
mod telegram;

#[derive(Debug)]
enum PayloadType {
    Issue,
    IssueComment,
    Push,
    Repository,
    UnimplementedAction(String),
}

impl FromStr for PayloadType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "issues" => PayloadType::Issue,
            "issue_comment" => PayloadType::IssueComment,
            "push" => PayloadType::Push,
            "repository" => PayloadType::Repository,
            _ => PayloadType::UnimplementedAction(s.to_string()),
        };
        Ok(result)
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let event_type = event
        .headers()
        .get("X-GitHub-Event")
        .expect("Missing X-GitHub-Event header");

    let payload_type = PayloadType::from_str(event_type.to_str().unwrap())?;

    let message = match payload_type {
        PayloadType::Issue => get_issue_chat_message(event.payload::<IssuePayload>()?.unwrap()),
        PayloadType::IssueComment => {
            get_issue_comment_chat_message(event.payload::<CommentIssuePayload>()?.unwrap())
        }
        PayloadType::Push => get_push_chat_message(event.payload::<PushPayload>()?.unwrap()),
        PayloadType::Repository => {
            get_repository_chat_message(event.payload::<RepositoryPayload>()?.unwrap())
        }
        PayloadType::UnimplementedAction(action) => get_unimplemented_action_message(action),
    };

    let response_body = match message {
        Some(message) => post_chat_message(message).await.unwrap(),
        None => format!("Ignored action"),
    };
    let response = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(response_body.into())
        .map_err(Box::new)?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
