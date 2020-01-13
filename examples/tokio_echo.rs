/*!
Tokio running reply socket that accepts requests and responds with the same data.
## Examples
```
# Defaults to listening at tcp://127.0.0.1:9823
cargo run --example tokio_echo
cargo run --example runngcat -- --req0 --dial tcp://127.0.0.1:9823 --data hi
```
*/

use clap::{App, Arg, ArgMatches};
use env_logger::{Builder, Env};
use log::info;
use runng::{asyncio::*, *, factory};

#[tokio::main]
async fn main() -> Result<()> {
    Builder::from_env(Env::default().default_filter_or("debug"))
        .try_init()
        .unwrap_or_else(|err| println!("env_logger::init() failed: {}", err));

    let matches = get_matches();

    let handle = tokio::spawn(async move {
        let mut replier = create_echo(&matches).unwrap();
        loop {
            if let Ok(msg) = replier.receive().await {
                info!("Echo {:?}", msg);
                replier.reply(msg).await.unwrap().unwrap();
            }
        }
    });
    let res = handle.await;
    info!("Result = {:?}", res);
    Ok(())
}

fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("tokio_echo")
        .arg(
            Arg::with_name("listen")
                .long("listen")
                .help("Bind to and accept connections at specified address")
                .default_value("tcp://127.0.0.1:9823"),
        )
        .get_matches()
}

fn create_echo<'a>(matches: &ArgMatches<'a>) -> Result<ReplyAsyncHandle> {
    let url = matches.value_of("listen").unwrap();
    let factory = factory::latest::ProtocolFactory::default();
    let replier = factory
        .replier_open()?
        .listen(&url)?
        .create_async()?;
    Ok(replier)
}
