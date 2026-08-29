use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    temperature: f32,
    top_k: i32,
    min_p: f32,
    repeat_penalty: f32,
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

fn request(text: &str, system_prompt: &str) -> Option<String> {
    let cfg = config::get();
    if !cfg.enabled {
        return None;
    }

    let mut full_system_prompt = system_prompt.to_owned();
    if let Some(glossary) = crate::dictionary::build_glossary(text) {
        full_system_prompt.push_str("\n\n");
        full_system_prompt.push_str(&glossary);
    }
    let user_content = format!("[Source Text]\n{text}");

    let agent = ureq::Agent::new_with_defaults();

    let req = ChatRequest {
        model: &cfg.model,
        messages: vec![
            ChatMessage {
                role: "system",
                content: &full_system_prompt,
            },
            ChatMessage {
                role: "user",
                content: &user_content,
            },
        ],
        temperature: cfg.temperature,
        top_k: cfg.top_k,
        min_p: cfg.min_p,
        repeat_penalty: cfg.repetition_penalty,
    };

    let mut response = match agent.post(&cfg.endpoint).header("Content-Type", "application/json").send_json(&req) {
        Ok(r) => r,
        Err(e) => {
            crate::logging::warn(&format!("llm::request: request failed: {e}"));
            return None;
        }
    };

    let body_str = match response.body_mut().read_to_string() {
        Ok(s) => s,
        Err(e) => {
            crate::logging::warn(&format!("llm::request: failed to read response body: {e}"));
            return None;
        }
    };

    let parsed: ChatResponse = match serde_json::from_str(&body_str) {
        Ok(p) => p,
        Err(e) => {
            crate::logging::warn(&format!("llm::request: failed to parse response JSON: {e}"));
            return None;
        }
    };

    parsed.choices.into_iter().next().map(|c| c.message.content)
}

pub fn translate(text: &str) -> Option<String> {
    let system_prompt = config::get().system_prompt;
    request(text, &system_prompt)
}

pub fn translate_name(name: &str) -> Option<String> {
    let name_prompt = config::get().name_prompt;
    request(name, &name_prompt)
}
