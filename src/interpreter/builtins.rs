pub mod api_functions;
pub mod reserved_functions;

use crate::error_format::data::ErrorInfo;
use crate::interpreter::{json_to_rust::*, message::*};
use crate::parser::{ast::*, literal::Literal};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::hash::BuildHasher;

pub fn create_submap<S: BuildHasher>(
    keys: &[&str],
    args: &HashMap<String, Literal, S>,
) -> Result<Map<String, Value>, ErrorInfo> {
    let mut map = Map::new();

    for elem in args.keys() {
        if keys.iter().find(|&&x| x == elem).is_none() {
            if let Some(value) = args.get(&*elem) {
                map.insert(elem.clone(), Message::lit_to_json(value.to_owned()));
            }
        }
    }
    Ok(map)
}

fn client_to_json(client: &Client) -> Map<String, Value> {
    let mut map = Map::new();

    map.insert("bot_id".to_owned(), Value::String(client.bot_id.to_owned()));
    map.insert(
        "channel_id".to_owned(),
        Value::String(client.channel_id.to_owned()),
    );
    map.insert(
        "user_id".to_owned(),
        Value::String(client.user_id.to_owned()),
    );

    map
}
