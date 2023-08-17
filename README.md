# PLACOGAVIKI

PLACOGAVIKI is a program that empowers participants in a chat to collectively control a game.

## Table of Contents

- [PLACOGAVIKI](#placogaviki)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Supported platforms](#supported-platforms)
  - [Configuration](#configuration)
  - [License](#license)

## Overview

PLACOGAVIKI enables a collaborative gaming experience by allowing participants in a chat environment to collectively control a game.
It can be integrated with various chat platforms, currently including Discord and Kick.

## Requirements

Before getting started, ensure you have the following prerequisites:

- [Rust](https://www.rust-lang.org/) installed for compiling this project
- [ViGEm](https://github.com/ViGEm/ViGEmBus/releases/latest) to manage the virtual gamepad

## Installation

To build and run PLACOGAVIKI, follow these steps:

1. Clone this repository to your local machine.
2. Navigate to the repository directory.
3. Open a terminal and execute the following command:

    ```sh
    cargo run --release
    ```

## Supported platforms

PLACOGAVIKI is designed as a modular project, making it easy to extend to different services and platforms.
Currently, it supports the following platforms:

- [Discord](https://discord.com/)
  - To integrate with Discord, you need a bot token. Set the `DISCORD_TOKEN` variable in the [.env](./.env) file.
  - The program will monitor all channels for user inputs while ignoring bot messages.
- [Kick](https://kick.com/)
  - For Kick chatrooms, you can specify the subscribed chatrooms in the [chatrooms file](./src/service/kick/chatrooms.rs#L8)

## Configuration

The [.env](./.env) file is automatically generated and updated when the program runs.
This file includes the necessary configuration variables.
You can find this file in the root directory.

## License

PLACOGAVIKI is released under the [GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0)
