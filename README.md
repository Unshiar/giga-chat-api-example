# GigaChat API Rust Examples

Simple implementation examples of the [GigaChat API](https://developers.sber.ru/) in Rust using popular HTTP client libraries: `reqwest` and `ureq`.

Each example is a separate binary demonstrating how to:
1. Obtain an access token via OAuth
2. List available AI models
3. Send a chat completion request and receive a response

## Quick Start

### 1. Clone the repository
```bash
git clone git@github.com:Unshiar/giga-chat-api.git
cd giga-chat-api
```

### 2. Get your GigaChat Authorization Key
- Visit [GigaChat API Portal](https://developers.sber.ru/)
- Follow the setup instructions to obtain your Authorization key
- The key should be in Base64 format

### 3. Configure environment
```bash
cp .env.example .env
```

### 4. Set your Authorization key
Edit `.env` file and set:
```env
AUTHORIZATION_KEY=your_base64_encoded_key_here
```

## Running Examples

### Using reqwest (async-compatible HTTP client)
```bash
cargo run --example reqwest_example
```

### Using ureq (synchronous HTTP client)
```bash
cargo run --example ureq_example
```

For both examples, the output will be similar to the following:
```
Your access token:
{
  "access_token": "<token>",
  "expires_at": 1787151058823
}

List of models:
{
  "data": [
    {
      "id": "GigaChat-2",
      "object": "model",
      "owned_by": "salutedevices",
      "type": "chat"
    },
    {
      "id": "GigaChat-2-Max",
      "object": "model",
      "owned_by": "salutedevices",
      "type": "chat"
    },
    {
      "id": "GigaChat-2-Pro",
      "object": "model",
      "owned_by": "salutedevices",
      "type": "chat"
    },
    {
      "id": "GigaChat-3-Ultra",
      "object": "model",
      "owned_by": "salutedevices",
      "type": "chat"
    },
    {
      "id": "Embeddings",
      "object": "model",
      "owned_by": "salutedevices",
      "type": "embedder"
    },
    {
      "id": "Embeddings-2",
      "object": "model",
      "owned_by": "salutedevices",
      "type": "embedder"
    },
    {
      "id": "EmbeddingsGigaR",
      "object": "model",
      "owned_by": "salutedevices",
      "type": "embedder"
    },
    {
      "id": "GigaEmbeddings-3B-2025-09",
      "object": "model",
      "owned_by": "salutedevices",
      "type": "embedder"
    }
  ],
  "object": "list"
}

Answer from AI:
{
  "choices": [
    {
      "finish_reason": "stop",
      "index": 0,
      "message": {
        "content": "Hello! I'm doing great, thanks for asking. Ready for an interesting conversation or a challenge? What brings you here today?",
        "role": "assistant"
      }
    }
  ],
  "created": 1787149260,
  "model": "GigaChat-2:2.0.30.01",
  "object": "chat.completions",
  "usage": {
    "completion_tokens": 27,
    "precached_prompt_tokens": 0,
    "prompt_tokens": 29,
    "total_tokens": 56
  }
}
```


## Project Structure

- `examples/reqwest_example.rs` - Example using the `reqwest` blocking client
- `examples/ureq_example.rs` - Example using the `ureq` HTTP client
- `.env.example` - Environment variables template (copy to `.env` and fill in values)
- `russian_trusted_root_ca_pem.crt` - CA certificate for HTTPS connections

## Requirements

- Rust 1.90+
- A valid GigaChat Authorization key

## Dependencies

- `reqwest` - HTTP client with blocking support
- `ureq` - Lightweight synchronous HTTP client
- `serde_json` - JSON serialization/deserialization
- `uuid` - UUID generation for request IDs
- `dotenv` - Environment variable loading

## API Reference

The examples interact with these GigaChat API endpoints:
- `POST https://ngw.devices.sberbank.ru:9443/api/v2/oauth` - Token endpoint
- `GET https://api.giga.chat/v1/models` - List available models
- `POST https://api.giga.chat/v1/chat/completions` - Chat completion endpoint

## License

The unlicense - see the [LICENSE](LICENSE) file for details.

