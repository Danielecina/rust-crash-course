# Interactive Chat project in RUST

The idea behind this project is confusing. I wanted to do something in rust and try from the basics to something more complex.

This is a simple terminal-based chat application that I created to learn how to work with rust. It focuses on exploring key features such as channels for message passing. The program allows users to send and receive messages asynchronously, providing an opportunity to experience competition and communication in rust.

```bash
cd ./hello/chat

`cargo run --release --bin server localhost:8080`

# You can run X times this command in multiple terminal window and see whats appened
`cargo run --release --bin client localhost:8080`

# For each terminal window use this interactive command
join {CHAT_NAME}

# Every chat connected will receive the message
post {CHAT_NAME} something
```
