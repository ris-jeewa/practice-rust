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
- **Containerization**: Docker
- **Cloud Platform**: Google Cloud Platform (GCP)
- **CI/CD**: GitHub Actions

## Prerequisites
- Rust (latest stable version)
- Cargo
- PostgreSQL

## Features
- Layered Architecture: The application is structured with the following layers for better separation of concerns:
    - Repository Layer: Handles all interactions with the database using SeaORM.
    - Handler Layer: Defines the routes and controls the flow of requests and responses.
    - Service Layer: Contains the business logic, acting as the intermediary between the handler and repository layers.
- Deployment on Google Cloud Run: The application is deployed on Google Cloud Platform's Cloud Run, leveraging Docker containers. It uses Google Artifact Registry for storing container images.
- CI/CD Pipeline: Continuous Integration, Delivery, and Deployment are implemented using GitHub Actions. This automates the process of building, testing, and deploying the application to Google Cloud Run.
- Automated Deployments with GitHub Actions: GitHub Actions is set up to automate the deployment process to Google Cloud Run, ensuring smooth and reliable updates to the production environment.

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
 **Product Endpoints**:
- GET /product         - Lists all products.
- POST /product        - Create a new product.
- PUT /product/{id}    - Update a product by ID.
- DELETE /product/{id} - Delete a product by ID.

  **Item Endpoints**:
- GET /item/{id}     - get an item by ID .
- POST /item         - Create a new item.
- PUT /item/{id}     - Update an item by ID.
- DELETE /items/{id} - Delete an item by ID.

## Configuration
The service uses environment variables and supports configuration via `.env` file.

## Deployed Version
You can access the deployed version of the service at: https://newproj-288242518278.us-central1.run.app

Project Link: https://github.com/ris-jeewa/project_2
