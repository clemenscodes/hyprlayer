# Warcraft III memory reading

## w3api.exe

The [binary here](./crates/bin/w3api) reads the Observer API of the Warcraft process
to check several things.

- Is there an ongoing match?
- How much time has passed in the match?
- How many players are playing in the match?
- What is the name of the match?
- On which map is the match played?

The program now reads that exported observer API and waits for a match to start.
As long as a match is running, it will then read the specified memory entries
defined in the [cheat table here](./w3.CT)
and export the serialized results to another JSON file
which can be set with the environment variable `WARCRAFT_API_PATH`,
otherwise it will fallback to `warcraft_api.json` in the same directory of the `w3api.exe`.

The actual memory reading differs on linux and windows systems,
as the base address of the Warcraft III process
can not be determined using native windows APIs when running in wine.

## Downloads

The binary is built in GitHub Actions and can be downloaded from the [downloads section](https://github.com/clemenscodes/w3mem/releases/tag/latest)
which always contains the latest compiled binaries from the [master branch](https://github.com/clemenscodes/w3mem).
