use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "Geral")]
pub async fn db(ctx: Context<'_>) -> Result<(), Error> {
    let data = sqlx::query!(r#"SELECT 1 + 1 as num"#)
        .fetch_one(&ctx.data().pool)
        .await?;

    ctx.reply(format!("Result: `{:?}`", data)).await?;
    Ok(())
}
