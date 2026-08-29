use serde::{Deserialize, Serialize};

const ENDPOINT: &str = "http://127.0.0.1:1234/v1/chat/completions"; 
const MODEL: &str = "gemma4-12b-qat-uncensored-hauhaucs-balanced"; 

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Deserialize)]
struct ChatResponseMessage {
    content: String,
}

pub fn translate(text: &str) -> Option<String> {
    let agent = ureq::Agent::new_with_defaults();

    let request = ChatRequest {
        model: MODEL,
        messages: vec![
            ChatMessage {
                role: "system",
                content: "Translate the following Japanese dialogue into English. Reply with only the translation, no notes or explanation.",
            },
            ChatMessage {
                role: "user",
                content: text,
            },
        ],
    };

    let mut response = match agent.post(ENDPOINT).header("Content-Type", "application/json").send_json(&request) {
        Ok(r) => r,
        Err(e) => {
            crate::logging::warn(&format!("llm::translate: request failed: {e}"));
            return None;
        }
    };

    let body_str = match response.body_mut().read_to_string() {
        Ok(s) => s,
        Err(e) => {
            crate::logging::warn(&format!("llm::translate: failed to read response body: {e}"));
            return None;
        }
    };

    let parsed: ChatResponse = match serde_json::from_str(&body_str) {
        Ok(p) => p,
        Err(e) => {
            crate::logging::warn(&format!("llm::translate: failed to parse response JSON: {e}"));
            return None;
        }
    };

    parsed.choices.into_iter().next().map(|c| c.message.content)
}
