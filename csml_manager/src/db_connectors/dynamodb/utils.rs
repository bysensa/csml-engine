use std::collections::HashMap;
use crate::{ManagerError, Client};
use rusoto_dynamodb::AttributeValue;
use serde::Serialize;

/**
 * Return the current datetime formatted as YYYY-MM-DDTHH:mm:ss.SSS[Z].
 * For example: 2020-03-12T12:33:42.123Z
 */
pub fn get_date_time() -> String {
  return chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S.%3f[Z]").to_string();
}

/**
 * Convert any data struct into an AttributeValue
 */
pub fn to_attribute_value_map<T: Serialize>(data: &T) -> Result<HashMap<String, AttributeValue>, ManagerError> {
  match serde_dynamodb::to_hashmap(data) {
      Ok(val) => Ok(val),
      Err(err) => Err(ManagerError::Manager(err.to_string())),
  }
}

/**
 * Convert any AttributeValue to a generic serde_json::Value
 */
pub fn from_attribute_value_map(data: &HashMap<String, AttributeValue>) -> Result<serde_json::Value, ManagerError> {
  match serde_dynamodb::from_hashmap(data.to_owned()) {
    Ok(val) => Ok(val),
    Err(err) => return Err(ManagerError::Manager(err.to_string())),
  }
}

/**
 * Return the table's name
 */
pub fn get_table_name() -> Result<String, ManagerError> {
  match std::env::var("AWS_DYNAMODB_TABLE") {
      Ok(val) => return Ok(val),
      _ => return Err(ManagerError::Manager("Missing AWS_DYNAMODB_TABLE env var".to_owned())),
  }
}

/**
 * Create a hash key from the client info
 */
pub fn make_hash(client: &Client) -> String {
  format!("bot_id:{}#channel_id:{}#user_id:{}", client.bot_id, client.channel_id, client.user_id)
}

/**
* Create a serialized range key from given arguments
*/
pub fn make_range(args: &[&str]) -> String {
  let mut res = "".to_owned();
  for arg in args.iter() {
      if res.len() > 0 {
          res = res + "#";
      }
      res = res + arg.to_owned();
  }
  res.to_owned()
}

