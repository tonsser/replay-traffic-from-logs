extern crate instant_replay;
extern crate dotenv;
extern crate postgres;
extern crate openssl;
extern crate regex;

mod parse_args;
use parse_args::{Args};

use instant_replay::{AccessTokenLoader, InstantReplay};
use instant_replay::logs_provider::{LogsFromRemoteFile};
use std::env;
use postgres::{Connection, TlsMode};
use std::collections::HashMap;
use dotenv::dotenv;
use openssl::ssl::{SslMethod, SslConnectorBuilder, SSL_VERIFY_NONE};
use postgres::tls::openssl::OpenSsl;

struct LoadAccessTokenFromDatabase {
    connection: Connection,
    cache: HashMap<String, String>,
}

impl LoadAccessTokenFromDatabase {
    fn new() -> Self {
        let mut connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
        connector.builder_mut().set_verify(SSL_VERIFY_NONE);
        let openssl = OpenSsl::from(connector.build());

        let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env var");

        let connection = Connection::connect(
            db_url,
            TlsMode::Prefer(&openssl),
            ).expect("failed to connect");

        LoadAccessTokenFromDatabase {
            connection: connection,
            cache: HashMap::new(),
        }
    }
}

impl AccessTokenLoader for LoadAccessTokenFromDatabase {
    fn access_token_from_user_slug(&mut self, user_slug: &String) -> Option<String> {
        match self.cache.get(user_slug) {
            Some(token) => {
                println!("hit: {}", user_slug);
                return Some(token.clone());
            }
            _ => (),
        }

        println!("miss: {}", user_slug);

        let mut token = None;

        let sql = r#"
            SELECT
                users.slug AS slug,
                oauth_access_tokens.token AS token
            FROM users
            INNER JOIN oauth_access_tokens ON resource_owner_id = users.id
            WHERE slug = $1
            LIMIT 1
            "#;

        let rows = &self.connection.query(sql, &[&user_slug]).expect("query failed");

        for row in rows {
            let users_token: String = row.get("token");
            token = Some(users_token.clone());
            self.cache.insert(user_slug.clone(), users_token);
        }

        token
    }
}

fn main() {
    dotenv().ok();

    let args = Args::parse_from_commandline_args()
        .expect("Failed parsing cmd line args");

    InstantReplay {
        access_token_loader: LoadAccessTokenFromDatabase::new(),
        logs_provider: LogsFromRemoteFile {
            url: env::var("LOGS_FILE").unwrap()
        },
        thread_count: args.thread_count,
        run_for: args.duration,
        host: "http://api.tonsser.com".to_string(),
    }.run();
}
