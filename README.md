Uses <https://github.com/tonsser/instant-replay> to benchmark [api.tonsser.com](http://api.tonsser.com).

## Usage

Configure using these environment variables:

- `LOGS_FILE`: URL of a logs files from the API.

Run the benchmark with `heroku run --size performance-l ./target/release/replay-traffic-from-logs --thread-count THREAD_COUNT --duration 20`. `--thread-count` is the number of threads to open. `--duration` is the duration to run the script for in seconds.

You can also use `script/run`.
