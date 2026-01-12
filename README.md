# Meditrack

**Meditrack** is a simple command-line tool built in Rust that helps users track their daily medications.

## Features

- **Add Medications:** Easily add a medicine with dosage and time.
- **List Medications:** View all medications you need to take for the day.
- **Mark as Taken:** Mark medications as taken so you can keep track.

## Installation

1. Download the binary for your platform from the [Releases](https://github.com/alanirawad-sketch/Meditrack/releases/tag/v1.0) page.
2. Place it in a directory included in your PATH.

## Usage

Run the following commands in your terminal:

```sh
# Show help
meditrack help

# Add a medication
meditrack add "Aspirin" "500mg" "08:00"

# List all medications
meditrack list


# Contributing

Feel free to submit issues or pull requests to improve Meditrack. We welcome contributions!

# License

This project is licensed under the MIT License.

# Mark a medication as taken
meditrack take 1
