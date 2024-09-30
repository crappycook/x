# Rust Trading Bot

This is a simple trading bot written in Rust that tracks cryptocurrency prices using the OKX WebSocket API. It stores data in a SQLite database and provides a command-line interface for user interaction.

## Features

- Tracks real-time cryptocurrency prices using OKX WebSocket API
- Stores data in a SQLite database
- Command-line interface for selecting cryptocurrency pairs
- Configurable logging
- Database migrations for schema management

## Prerequisites

To run this bot, you need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

## Configuration

The bot uses a configuration file located at `config/dev.toml`. You can modify this file to change logging and database settings.

## Getting Started

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/rust-trading-bot.git
   cd rust-trading-bot
   ```

2. Build the project:

   ```sh
   cargo build
   ```

3. Run the bot:

   ```sh
   cargo run
   ```

   Or use the provided Makefile:

   ```sh
   make run
   ```

4. Follow the prompts to enter the base and quote cryptocurrencies you want to track.

## Development

- Use `make run-release` for optimized builds
- Use `make run-release-watch` for development with auto-reloading
- Use `make clean` to remove log files

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
