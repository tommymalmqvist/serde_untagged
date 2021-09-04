use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{AuthorizeEnum, AuthorizeRequest, CallActionEnum};

    #[test]
    fn json_to_json_value() {
        let data = r#"
        {
            "auth_request": "this is a auth request"
        }"#;
        // ofc works
        let v: Value = serde_json::from_str(data).unwrap();
        println!("{:?}", v);
    }

    #[test]
    fn json_authorize_request() {
        let data = r#"
        {
            "auth_request": "this is a auth request"
        }"#;
        // ofc works
        let ar: AuthorizeRequest = serde_json::from_str(data).unwrap();
        println!("{:?}", ar);
    }

    #[test]
    fn json_authorize_enum() {
        let data = r#"
        {
            "auth_request": "this is a auth request"
        }"#;
        // broken, why?
        let ae: AuthorizeEnum = serde_json::from_str(data).unwrap();
        println!("{:?}", ae);
    }

    #[test]
    fn json_callactionenum() {
        let auth_req = r#"
        {
            "auth_request": "this is a auth request"
        }"#;

        let bnr_req = r#"
        {
            "bnr_request": "this is a bootnotification request"
        }"#;
        // broken, why?
        let cae1: CallActionEnum = serde_json::from_str(auth_req).unwrap();
        println!("{:?}", cae1);
        let cae2: CallActionEnum = serde_json::from_str(bnr_req).unwrap();
        println!("{:?}", cae2);
    }
}

fn main() {
    println!("Nothing to see here, run the tests!")
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
enum CallActionEnum {
    Authorize(AuthorizeEnum),
    BootNotification(BootNotificationEnum),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
enum AuthorizeEnum {
    AuthorizeRequest(AuthorizeRequest),
    AuthorizeResponse(AuthorizeResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
enum BootNotificationEnum {
    BootNotificationRequest(BootNotificationRequest),
    BootNotificationResponse(BootNotificationResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct AuthorizeRequest {
    auth_request: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct AuthorizeResponse {
    auth_response: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct BootNotificationRequest {
    bnr_request: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct BootNotificationResponse {
    bnr_response: String,
}
