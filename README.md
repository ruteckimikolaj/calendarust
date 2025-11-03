# Terminal Calendar

A terminal-based calendar application written in Rust, using the `ratatui` library for the UI.

## Features

- **Year, Month, Week, and Day Views:** Navigate through different time scales.
- **Event Management:** Create, view, edit, and delete events.
- **Offline-First:** All data is stored locally in an SQLite database.
- **Customizable:** Configure the application through a simple TOML file.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- A terminal that supports ANSI escape codes

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/terminal-calendar.git
   cd terminal-calendar
   ```

2. **Build and run the application:**
   ```bash
   cargo run
   ```

## Usage

- **`q`:** Quit the application
- **`Tab`:** Cycle through views (Year -> Month -> Week -> Day)
- **Arrow Keys:** Navigate within a view
- **`Enter`:** Select a date or time slot
- **`Esc`:** Go back or cancel an action

## Configuration

The configuration file is located at `~/.config/calendar-app/config.toml`. You can customize the default view, colors, and other settings in this file.
