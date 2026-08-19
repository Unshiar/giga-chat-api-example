use giga_chat_api::serde_json::json;
use giga_chat_api::ureq::Agent;
use giga_chat_api::ureq::tls::{Certificate, RootCerts, TlsConfig};
use giga_chat_api::uuid::Uuid;
use std::fs::read;

const TOKEN_URL: &str = "https://ngw.devices.sberbank.ru:9443/api/v2/oauth";
const MODELS_URL: &str = "https://api.giga.chat/v1/models";
const COMPLETION_URL_V1: &str = "https://api.giga.chat/v1/chat/completions";
const GIGA_CHAT_API: &str = "GIGACHAT_API_PERS";
const PEM_FILE_PATH: &str = "russian_trusted_root_ca_pem.crt";

fn main() -> Result<(), ureq::Error> {
    // Load .env for local development (no-op if not present)
    dotenv::dotenv().ok();

    // PEM path can be overridden with GIGA_PEM env var; default is russian_trusted_root_ca_pem.crt
    let pem_path = std::env::var("GIGA_PEM").unwrap_or_else(|_| PEM_FILE_PATH.into());
    let pem_bytes = read(&pem_path)?;

    // Configuring ureq Agent with loaded PEM certificate
    let certificate = Certificate::from_pem(&pem_bytes)?;
    let tls_config = TlsConfig::builder()
        .root_certs(RootCerts::from(vec![certificate]))
        .build();
    let agent: Agent = Agent::config_builder()
        .tls_config(tls_config)
        .build()
        .into();

    // Read AUTHORIZATION_KEY from environment
    let authorization_key = std::env::var("AUTHORIZATION_KEY")
        .expect("AUTHORIZATION_KEY must be set. See .env.example and CI secrets.");

    // 1. Getting access token
    let payload = format!("scope={}", GIGA_CHAT_API);
    let response_token: serde_json::Value = agent
        .clone()
        .post(TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .header("RqUID", Uuid::new_v4().to_string())
        .header("Authorization", format!("Basic {}", authorization_key))
        .send(payload)?
        .body_mut()
        .read_json()?;

    let token_access = response_token["access_token"]
        .as_str()
        .expect("No access token found");

    println!("Your access token:");
    println!("{}\n", serde_json::to_string_pretty(&response_token)?);

    // 2. Getting list of models
    let response_model: serde_json::Value = agent
        .clone()
        .get(MODELS_URL)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token_access))
        .call()?
        .body_mut()
        .read_json()?;

    println!("List of models:");
    println!("{}\n", serde_json::to_string_pretty(&response_model)?);

    // 3. Request to AI via v1 api, and response from it
    let request_to_ai = json!({
      "model": "GigaChat-2",
      "messages": [
        {
          "role": "user",
          "content": "Hello, GigaChat! How are you?"
        }
      ],
      "stream": false,
      "repetition_penalty": 1
    });
    let response_ai: serde_json::Value = agent
        .clone()
        .post(COMPLETION_URL_V1)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token_access))
        .send(request_to_ai.to_string())?
        .body_mut()
        .read_json()?;

    println!("Answer from AI:");
    println!("{}\n", serde_json::to_string_pretty(&response_ai)?);

    Ok(())
}
