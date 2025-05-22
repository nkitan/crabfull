#!/bin/bash

# Check if a log file path is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <log_file_path>"
  exit 1
fi

LOG_FILE="$1"

echo "Starting to write logs to: $LOG_FILE"
echo "Press Ctrl+C to stop."

# Create the log file if it doesn't exist, or append to it if it does
touch "$LOG_FILE"

# Infinite loop to generate log entries
while true; do
  # Get current timestamp
  TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")

  # Define an array of log levels
  LOG_LEVELS=("INFO" "WARNING" "ERROR" "DEBUG")
  # Get a random log level
  RANDOM_LEVEL_INDEX=$((RANDOM % ${#LOG_LEVELS[@]}))
  LOG_LEVEL=${LOG_LEVELS[$RANDOM_LEVEL_INDEX]}

  # Define an array of sample messages
  MESSAGES=(
    "User logged in successfully."
    "Payment processed for order #$((RANDOM % 1000 + 100))."
    "Database connection established."
    "Failed to retrieve user profile."
    "API request timed out."
    "System health check: OK."
    "Disk space low on /var/log."
    "New item added to cart."
    "Password reset request received."
    "Invalid input detected for field 'username'."
  )
  # Get a random message
  RANDOM_MESSAGE_INDEX=$((RANDOM % ${#MESSAGES[@]}))
  MESSAGE=${MESSAGES[$RANDOM_MESSAGE_INDEX]}

  # Construct the log entry
  LOG_ENTRY="$TIMESTAMP [$LOG_LEVEL] - $MESSAGE"

  # Append the log entry to the file
  echo "$LOG_ENTRY" >> "$LOG_FILE"

  # Wait for a short period (e.g., 1 second) before the next log entry
  # You can adjust this sleep duration as needed
  sleep 0.1
done
