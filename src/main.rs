use std::{sync::Arc, time::Duration};

use poise::serenity_prelude::{self as serenity, GatewayIntents};

pub mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let prefix = std::env::var("BOT_PREFIX").expect("No BOT_PREFIX was found");
    let token = std::env::var("DISCORD_TOKEN").expect("No DISCORD_TOKEN was found");
    let database_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL was found");

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let pool = sqlx::sqlite::SqlitePool::connect(&database_url)
        .await
        .expect("Couldn't connect to database");

    let migrations_path = std::path::Path::new("./migrations");

    if migrations_path.exists() {
        let migrator = sqlx::migrate::Migrator::new(migrations_path)
            .await
            .expect("Failed to create Migrator instance");

        migrator
            .run(&pool)
            .await
            .expect("Couldn't run database migrations");
    }

    let commands = vec![commands::general::db()];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(prefix),
                case_insensitive_commands: true,
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    Duration::from_secs(3600),
                ))),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("Executing command `{}`...", ctx.command().qualified_name);
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Executed command `{}`!", ctx.command().qualified_name);
                })
            },
            command_check: Some(|ctx| Box::pin(async move { Ok(!ctx.author().bot) })),
            skip_checks_for_owners: false,
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(async move {
                    println!("New event in event handler: {:?}", event.snake_case_name());
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { pool })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
