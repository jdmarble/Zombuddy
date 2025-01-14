use poise::serenity_prelude as serenity;
use tokio::sync::Mutex;

// User data, which is stored and accessible in all command invocations
struct Data {
    rcon_connection: Mutex<rcon::Connection<rcon::AsyncStdStream>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn restart(ctx: Context<'_>) -> Result<(), Error> {
    match ctx.data().rcon_connection.lock().await.cmd("quit").await {
        Ok(_) => {
            ctx.say("Okay.").await?;
            Ok(())
        }
        Err(_) => todo!(),
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let rcon_address = std::env::var("RCON_ADDRESS").expect("missing RCON_ADDRESS");
    let rcon_password = std::env::var("RCON_PASSWORD").expect("missing RCON_PASSWORD");

    let rcon_connection = <rcon::Connection<rcon::AsyncStdStream>>::builder()
        .connect(rcon_address, rcon_password.as_str())
        .await
        .expect("Could not connect to rcon");

    let intents = serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![restart()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    rcon_connection: Mutex::new(rcon_connection),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
