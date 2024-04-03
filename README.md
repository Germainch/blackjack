# Blackjack Game

This is a simple web-based blackjack game where you can play against the computer. The project is written in HTML for the frontend and Rust for the backend server.

## Features

- Play classic blackjack against the computer.
- Simple and intuitive user interface.
- Server-side logic implemented in Rust for security and efficiency.

## Prerequisites

Before running this project, ensure you have the following installed:

- Rust (stable version)
- Web browser

## Installation and Setup

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/your_username/blackjack-game.git
   
2. Navigate to the project directory
   ```bash
   cd blackjack
   
3. Build the Rust server
    ```bash
   cargo build --release
4. Run the server
    ```bash
   cargo run --release
5. Open your browser and navigate to 'http://localhost:8000'

## How to Play
1. login to your account or register
2. click "Draw" to draw a card
3. click "Fold" to end your turn
4. click "Bet" to increase your bet by 100
5. try to get as close to 21 possible without going over !