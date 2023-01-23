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
* Test: `cargo test`
* Run: `docker-compose-up`
* Integration test: `yarn install` (once) and then `yarn integration:local(dev/staging/prod)`

### WalletConnect Specific

- [ ] `/.github/workflows/**/*.yml`
  Change the `runs-on` to the `ubuntu-runners` group

## GitHub Secrets
Required GitHub secrets for Actions to run successfully
- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `PAT` Personal Access Token for Github to commit releases

### WalletConnect Specific
- `ASSIGN_TO_PROJECT_GITHUB_TOKEN`
