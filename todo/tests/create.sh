#!/bin/bash

curl -X POST -H "Content-Type: application/json" -d '{"title":"Buy groceries","content":"banana,milk", "is_done": false}' http://localhost:8081/tasks

curl http://localhost:8081/tasks

curl http://localhost:8081/tasks/1

curl -X POST -H "Content-Type: application/json" -d '{"title":"Buy Groceries", "content": "banana", "is_done": true}' http://localhost:8081/tasks/1

curl -X DELETE http://localhost:8081/tasks/1
