# Petstore.rs

This project is a Rust implementation inspired by the [Swagger Petstore API](https://petstore.swagger.io). It aims to recreate the functionality of the Petstore as a learning journey and an introduction to building APIs using the Rust programming language.

![Rust Language](https://img.shields.io/badge/Rust-100%25-orange)
![Open Issues](https://img.shields.io/github/issues/vijayprakashpj/petstore-rs)

## Features

- A simple RESTful API interface for managing pets (add, update, delete operations).
- Illustrates how to design APIs in Rust, following best practices.
- Utilizes tools and crates to build lightweight, performant server-side applications.

## Prerequisites

Ensure you have the following installed:
1. [Rust](https://www.rust-lang.org/)
2. [Cargo](https://doc.rust-lang.org/cargo/)
3. Any HTTP client for sending API requests (e.g., [Postman](https://www.postman.com/) or `curl`).

## Installation and Running the Service

1. Clone the repository:
    ```bash
    git clone https://github.com/vijayprakashpj/petstore-rs
    cd petstore-rs
    ```

2. Build the project:
    ```bash
    cargo build
    ```

3. Run the service:
    ```bash
    cargo run
    ```

4. Open your browser, Postman, or terminal (`curl`) to interact with the API.

## Quickstart: Example API Endpoints (NOT IMPLEMENTED YET :pray:)

Below are some examples that illustrate how to interact with the API:

### Add a Pet
`POST /pets`
```bash
curl -X POST http://localhost:<PORT>/pets \
-H "Content-Type: application/json" \
-d '{"name": "Rusty", "age": 3, "kind": "dog"}'
```

### Get All Pets
`GET /pets`
```bash
curl http://localhost:<PORT>/pets
```

### Remove a Pet
`DELETE /pets/<ID>`
```bash
curl -X DELETE http://localhost:<PORT>/pets/1
```

## Project Roadmap

Here’s what you can expect in the next iterations:
1. Database and ORM setup.
2. HTTP app and controllers.
3. Advanced query filters for searching and pagination.
4. Infrastructure setup.
5. Dockerfile and containerization support.

## Contribution

Contributions are welcome! Here’s how you can help:
1. Open an issue to report a bug or request a feature.
2. Submit a pull request to fix an issue or add a new feature.

## License

TBD - Please add a license here! We recommend the MIT License for open-source projects.

## Feedback & Support

If you found this project helpful, drop a 🌟 on the repo!
