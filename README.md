## Gitlab Environment Variable Manager

A tool written in Rust to utilize the Gitlab APIs to manage the env variables for a project.

### Build

1. Use `Cargo` to build the application according to your platform (Linux,MacOS,Windows,etc.).
2. Use `cargo build --release` to make a release build. The binary should be avaialble under the `target/release` with the name of `rust-gitlab-var-management`.
3. Put a `.env` in the project root dir wth the following variables to locate your project on Gitlab.

   ```sh
   API_TOKEN=<VALUE>
   PROJECT_ID=<VALUE>

   ```

4. With the `multiple-vars` variants, the file provided should have the variables in the same format as the `.env`.
5. Use `-h/--help` with the application to get the info regarding the available commands.
