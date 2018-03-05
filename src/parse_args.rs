use std::time::Duration;
use std::env;

#[derive(Debug)]
pub struct Args {
    pub thread_count: i32,
    pub duration: Duration,
}

impl Args {
    pub fn parse_from_commandline_args() -> Option<Self> {
        let args = env::args().collect::<Vec<String>>();
        Self::parse_from_vec(&args)
    }

    fn parse_from_vec(args: &Vec<String>) -> Option<Self> {
        // I wish I could write something like
        //
        // do!({
        //   let thread_count <- parse_thread_count(args);
        //   let duration <- parse_duration(args);
        //   pure(Args { thread_count: thread_count, duration: duration })
        // })

        parse_thread_count(args).and_then(|thread_count| {
            parse_duration(args).map(|duration| {
                Args {
                    thread_count: thread_count,
                    duration: duration,
                }
            })
        })
    }
}

fn parse_thread_count(args: &Vec<String>) -> Option<i32> {
    let idx = index_of_flag(args, "--thread-count");
    idx.and_then(|idx| args.get(idx + 1).and_then(|count| count.parse().ok()))
}

fn parse_duration(args: &Vec<String>) -> Option<Duration> {
    let idx = index_of_flag(args, "--duration");
    idx.and_then(|idx| {
        args.get(idx + 1).and_then(|count| {
            count.parse().ok().map(|sec| Duration::from_secs(sec))
        })
    })
}

fn index_of_flag(args: &Vec<String>, flag: &'static str) -> Option<usize> {
    let mut index_of_flag = None;
    for (idx, s) in args.iter().enumerate() {
        if s == flag {
            index_of_flag = Some(idx);
        }
    }
    index_of_flag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_args_as_a_vector() {
        let cmd_args = vec![
            "./path/to/file",
            "--thread-count",
            "200",
            "--duration",
            "10",
        ].iter()
            .map(ToString::to_string)
            .collect();
        let args = Args::parse_from_vec(&cmd_args).expect("Parse failed");

        assert_eq!(args.thread_count, 200);
        assert_eq!(args.duration, Duration::from_secs(10));
    }
}
