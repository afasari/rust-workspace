# Rust Todo API

A simple RESTful API for managing todos built with Rust, Axum, and Diesel.

## Features

- CRUD operations for todos
- PostgreSQL database
- RESTful API endpoints
- Docker support
  
## Prerequisites

- Rust
- PostgreSQL
- Diesel CLI

## Setup

1. Clone the repository
2. Copy `.env.example` to `.env` and update the database URL
3. Run migrations:

```bash
diesel migration run

## API Endpoints
- POST /tasks - Create a new task
- GET /tasks - List all tasks
- GET /tasks/{id} - Get a specific task
- POST /tasks/{id} - Update a task
- DELETE /tasks/{id} - Delete a task

## Roadmap

- create frontend
