use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn oracle(
    ctx: Context<'_>,
    #[description = "Take a decision"] b: bool,
) -> Result<(), Error> {
    if b {
        ctx.say("You seem to be an optimistic kind of person...")
            .await?;
    } else {
        ctx.say("You seem to be a pessimistic kind of person...")
            .await?;
    }
    Ok(())
}

/// Say Hello!
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hi!").await?;
    Ok(())
}


/// Fully useless command! Sends an animation meme!
#[poise::command(slash_command)]
pub async fn sashley(ctx: Context<'_>) -> Result<(), Error> {
    let sashley_link = "https://cdn.discordapp.com/attachments/1010464562434285640/1012690887429591120/HIP___ANIMATION_MEME.mp4";

    ctx.say(sashley_link).await?;
    Ok(())
}

/// Kazakhstan Grozi nam Bombardowaniem!
#[poise::command(slash_command)]
pub async fn kazakhstan(ctx: Context<'_>) -> Result<(), Error> {
    let kazakh_link = "https://cdn.discordapp.com/attachments/1020620787289423892/1058706507073589268/kazakh.mp4";

    ctx.say(kazakh_link).await?;
    Ok(())
}