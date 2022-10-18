Personal Multi-Page-Website with client server separation using a REST API for Christopher Scholz build with [Rocket](https://rocket.rs/).
* Using `React`, `ReactRouter` to create the different pages
* REST API via Rocket (including CORS Headers)

The app can be run locally or via docker.

For the `local` setup run
* server
    ```
    cd server
    cargo run
    ```
* client
    ```
    cd client
    npm start
    ```

For the `docker` setup run
```
docker-compose up
```
