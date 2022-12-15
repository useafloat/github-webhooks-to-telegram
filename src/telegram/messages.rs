use regex::Regex;

use crate::payloads::{
    issue::IssuePayload, issue_comment::CommentIssuePayload, push::PushPayload,
    repository::RepositoryPayload,
};
use std::{str, vec};

// TODO: Add support for more code
// https://github.com/teloxide/teloxide/blob/master/src/utils/markdown.rs
fn clean_string(s: String) -> String {
    s.replace('_', r"\_")
        .replace('*', r"\*")
        .replace('[', r"\[")
        .replace(']', r"\]")
        .replace('(', r"\(")
        .replace(')', r"\)")
        .replace('~', r"\~")
        .replace('`', r"\`")
        .replace('>', r"\>")
        .replace('#', r"\#")
        .replace('+', r"\+")
        .replace('-', r"\-")
        .replace('=', r"\=")
        .replace('|', r"\|")
        .replace('{', r"\{")
        .replace('}', r"\}")
        .replace('.', r"\.")
        .replace('!', r"\!")
}

const IGNORED_ACTIONS: &'static [&'static str] = &["label"];

enum EmojiType {
    PartyPopper,
    RaisingHand,
    Label,
    Tools,
    Rocket,
    Memo,
    Building,
}

pub struct Message {
    pub text: String,
    pub images: Vec<String>,
}

fn get_emoji(emoji: EmojiType) -> String {
    let utf8_bytes = match emoji {
        EmojiType::PartyPopper => vec![240, 159, 142, 137],
        EmojiType::RaisingHand => vec![240, 159, 153, 139],
        EmojiType::Label => vec![240, 159, 143, 183],
        EmojiType::Tools => vec![240, 159, 155, 160],
        EmojiType::Rocket => vec![240, 159, 154, 128],
        EmojiType::Memo => vec![240, 159, 147, 157],
        EmojiType::Building => vec![240, 159, 143, 151],
    };
    str::from_utf8(&utf8_bytes).unwrap().to_string()
}

fn get_images(text: &str) -> Vec<String> {
    let regex = Regex::new(r"(\.com/\d{7}/[\d\w]{9}-[\d\w]{8}-[\d\w]{4}-[\d\w]{4}-[\d\w]{4}-[\d\w]{12}\.(?:png|jpg|jpeg))").unwrap();

    regex
        .captures_iter(text)
        .map(|capture| format!("https://user-images.githubusercontent{}", &capture[1]))
        .collect()
}

pub fn get_issue_chat_message(payload: IssuePayload) -> Option<Message> {
    let images = get_images(&payload.issue.body);
    Some(Message {
        text: match payload.action.as_str() {
            "assigned" | "unassigned" => format!(
                "{}Issue *[\\#{}]({})* in [{}]({})\n\n{}\n\n__{}__ to *{}*\nBy *{}*",
                get_emoji(EmojiType::RaisingHand),
                payload.issue.number,
                clean_string(payload.issue.html_url),
                clean_string(payload.repository.name),
                clean_string(payload.repository.html_url),
                clean_string(payload.issue.title),
                payload.action,
                payload.assignee.unwrap().login,
                payload.sender.login
            ),
            "labeled" | "unlabeled" => format!(
                "{}Issue *[\\#{}]({})* in [{}]({})\n\n{}\n\n__{}__ *{}*\nBy *{}*",
                get_emoji(EmojiType::Label),
                payload.issue.number,
                clean_string(payload.issue.html_url),
                clean_string(payload.repository.name),
                clean_string(payload.repository.html_url),
                clean_string(payload.issue.title),
                payload.action,
                clean_string(payload.label.unwrap().name),
                payload.sender.login
            ),
            "closed" => format!(
                "{}Issue *[\\#{}]({})* __{}__ in [{}]({}){}\n\n__{}__\n{}\n\nBy *{}*",
                get_emoji(EmojiType::PartyPopper).repeat(3),
                payload.issue.number,
                clean_string(payload.issue.html_url),
                payload.action,
                clean_string(payload.repository.name),
                clean_string(payload.repository.html_url),
                get_emoji(EmojiType::PartyPopper).repeat(3),
                clean_string(payload.issue.title),
                clean_string(payload.issue.body),
                payload.sender.login
            ),
            _ => format!(
                "{}Issue *[\\#{}]({})* __{}__ in [{}]({})\n\n__{}__\n{}\n\nBy *{}*",
                get_emoji(EmojiType::Tools),
                payload.issue.number,
                clean_string(payload.issue.html_url),
                payload.action,
                clean_string(payload.repository.name),
                clean_string(payload.repository.html_url),
                clean_string(payload.issue.title),
                clean_string(payload.issue.body),
                payload.sender.login
            ),
        },
        images,
    })
}

pub fn get_issue_comment_chat_message(payload: CommentIssuePayload) -> Option<Message> {
    let images = get_images(&payload.comment.body);
    Some(Message {
        text: format!(
            "{}Comment on isssue *[\\#{}]({})*\nIn {}\nBy *{}*\n\n{}",
            get_emoji(EmojiType::Memo),
            payload.issue.number,
            clean_string(payload.issue.html_url),
            clean_string(payload.repository.name),
            payload.sender.login,
            clean_string(payload.comment.body)
        ),
        images,
    })
}

pub fn get_repository_chat_message(payload: RepositoryPayload) -> Option<Message> {
    Some(Message {
        text: format!(
            "{}Repository *[\\{}]({})* __{}__\nBy *{}*\n\n{}",
            get_emoji(EmojiType::Building),
            clean_string(payload.repository.full_name),
            clean_string(payload.repository.html_url),
            clean_string(payload.action),
            payload.sender.login,
            clean_string(payload.repository.description)
        ),
        images: vec![],
    })
}

pub fn get_push_chat_message(payload: PushPayload) -> Option<Message> {
    let commits: Vec<String> = payload
        .commits
        .into_iter()
        .map(|commit| {
            format!(
                "\t\t__[{}]({})__",
                clean_string(commit.message),
                clean_string(commit.url)
            )
        })
        .collect();
    Some(Message {
        text: format!(
            "{}__Push__ in *[\\{}]({})*\nCommts:\n{}\n\nBy *{}*",
            get_emoji(EmojiType::Rocket),
            clean_string(payload.repository.name),
            clean_string(payload.repository.html_url),
            commits.join("\n"),
            clean_string(payload.sender.login),
        ),
        images: vec![],
    })
}
//
// TODO: Add ignored actions, such as label
pub fn get_unimplemented_action_message(action: String) -> Option<Message> {
    if IGNORED_ACTIONS.contains(&action.as_str()) {
        None
    } else {
        Some(Message {
            text: format!("Received uimplemented action: {}", action),
            images: vec![],
        })
    }
}
