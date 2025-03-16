#!/bin/bash

# Replace with your GitHub Pages URL
BASE_URL="https://afasari.github.io/rust-workspace"

# Create a new task
echo "Creating new task..."
curl -X POST -H "Content-Type: application/json" \
     -d '{"title":"Buy groceries","content":"banana,milk", "is_done": false}' \
     $BASE_URL/tasks

echo -e "\n\nListing all tasks..."
curl $BASE_URL/tasks

echo -e "\n\nGetting task with ID 1..."
curl $BASE_URL/tasks/1

echo -e "\n\nUpdating task..."
curl -X POST -H "Content-Type: application/json" \
     -d '{"title":"Buy Groceries", "content": "banana", "is_done": true}' \
     $BASE_URL/tasks/1

echo -e "\n\nDeleting task..."
curl -X DELETE $BASE_URL/tasks/1
