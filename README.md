# Rust Project with Docker and MySQL

This project demonstrates how to set up a Rust application using Docker, with MySQL as the database.

## Prerequisites

- Docker
- Docker Compose

## Setup Instructions

1. **Clone the repository**

   ```bash
   git clone <repository-url>
   cd rust
   ```

2. **Build and run the Docker containers**

   ```bash
   docker-compose up --build
   ```

3. **Access the application**

   The application will be running on [http://localhost:8080](http://localhost:8080).

4. **MySQL Database**

   - MySQL is running on port 3306.
   - The root password is set to `example`.

## Notes

- Ensure Docker is running on your machine before executing the commands.
- Modify the `docker-compose.yml` file to change configurations as needed.
