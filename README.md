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

