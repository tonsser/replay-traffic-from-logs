Uses <https://github.com/tonsser/instant-replay> to benchmark [api.tonsser.com](http://api.tonsser.com).

## Usage

Configure using these environment variables:

- `LOGS_FILE`: URL of a logs files from the API.
- `DURATION`: How many seconds to run the benchmark.

Run the benchmark with `heroku run ./target/release/replay-traffic-from-logs 500 --size performance-l`, then check newrelic.
