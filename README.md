# Product Item Microservice

## Overview
This is a Rust-based microservice for managing product items, built with Rust tools and libraries.

## Tech Stack
- **Web Framework**: Axum
- **ORM**: SeaORM
- **Database**: PostgreSQL
- **Async Runtime**: Tokio
- **Serialization**: Serde
- **Logging**: Tracing

## Prerequisites
- Rust (latest stable version)
- Cargo
- PostgreSQL

## Installation

### 1. Clone the Repository
```bash
git clone https://github.com/ris-jeewa/project_2
cd practice-rust
```

### 2. Environment Setup
Create a `.env` file in the project root with the following variables:
```
DATABASE_URL=postgres://username:password@localhost/your_database
```

### 3. Install Dependencies
```bash
cargo build
```

## Running the Microservice

### Development Mode
```bash
cargo run
```

## API Endpoints
- GET /api/product/getall: get all products.
- POST /api/product/create: create a product
- PUT /api/product/{id}/update: Update a product
- DELETE /api/product/{id}/delete: Delete a product by ID.
- GET /api/item/{id} : get a item by id
- POST /api/item/create: create a item
- PUT /api/item/{id}/update : update a item by id
- DELETE /api/item/{id}/delete: delete a item by id

## Configuration
The service uses environment variables and supports configuration via `.env` file.

### Key Configuration Options
- `DATABASE_URL`: PostgreSQL connection string

Project Link:https://github.com/ris-jeewa/project_2
