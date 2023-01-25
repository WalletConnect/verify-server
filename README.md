# Bouncer

This project also includes the standard CI/CD:
- Release
- Rust CI
- Terraform CI
- CD
- Intake
- Mocha (NodeJS) based integration tests

## Running the app

* Build: `cargo build`
* Test: `cargo test` (needs `docker run --name bouncer-redis -p 6379:6379 -d redis:6-alpine`)
* Run: `docker-compose-up`
* Integration test: `yarn install` (once) and then `yarn integration:local(dev/staging/prod)`
