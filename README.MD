## Installing

Install [rustup](https://www.rust-lang.org/tools/install).

##### Ubuntu

Run `lib-install.sh`. It will set up all the nessesary libraries.


##### Windows

Probably no additional installation needed, otherwise read the rust compiler output.


## Running

Run `cargo run` for testing.

Run `cargo run --release` for production.

To run on a new thread - `nohup cargo run --release &`.

To kill the thread - `kill -9 [procid]`.

To check `[procid]` use `ps -A` or `pidof [procname]`.

## Settings

Check `settings/api.json` for the API keys and tokens.