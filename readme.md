# Rust CRUD API

This is a simple CRUD (Create, Read, Update, Delete) REST API written in Rust using clean architecture. It provides endpoints to manage todo items stored in an in-memory repository.

## Deployment

This API is deployed on Render using a **free instance**. Note that the instance may **sleep after a period of inactivity** and may take a moment to wake up upon request.

## Endpoints

The API is hosted on Render and can be accessed at:

- Base URL: [https://lab-rust-crud-api.onrender.com/todo](https://lab-rust-crud-api.onrender.com/todo)

### Available Endpoints:

- **GET /todo**: Get all todo items.
- **GET /todo/{id}**: Get a specific todo item by ID.
- **POST /todo**: Create a new todo item.
- **PUT /todo/{id}**: Update an existing todo item by ID.
- **DELETE /todo/{id}**: Delete a todo item by ID.

## Usage

To interact with the API, you can use any HTTP client such as curl, Postman, or your web browser.

### Example curl Commands:

1. **Get All Todos**:
```shell
curl https://lab-rust-crud-api.onrender.com/todo
```


2. **Get Todo by ID**:
```shell
curl https://lab-rust-crud-api.onrender.com/todo/{id}
```

3. **Create a Todo**:
```shell
curl -X POST -H "Content-Type: application/json" -d '{"title": "Learn Rust", "completed": false}' https://lab-rust-crud-api.onrender.com/todo
```

4. **Update a Todo**:
```shell
curl -X PUT -H "Content-Type: application/json" -d '{"title": "Learn Rust in depth", "completed": true}' https://lab-rust-crud-api.onrender.com/todo/{id}
```

5. **Delete a Todo**:
```shell
curl -X DELETE https://lab-rust-crud-api.onrender.com/todo/{id}
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request for any improvements or bug fixes.
