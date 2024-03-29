use poise::serenity_prelude::{User, Mentionable};
use crate::helpers::misc::random_choice;

use crate::{Context, Error};


/// Bite someone
#[poise::command(slash_command, category = "Fun")]
pub async fn bite(
    ctx: Context<'_>,
    #[description = "Someone to bite"]
    user: User
) -> Result<(), Error> {
    let bite_gifs = vec![
        "https://tenor.com/view/cat-bite-funny-chomp-gif-16986241",
        "https://tenor.com/view/mikisi-kisi-kiss-gif-27218966",
        "https://tenor.com/view/funny-cat-bit-video-gif-14264780414888402835"
    ];

    let gif = random_choice(&bite_gifs).copied().unwrap_or("https://tenor.com/view/cat-bite-funny-chomp-gif-16986241");
    
    ctx.say(format!("{} bites {}", ctx.author().mention(), user.mention())).await?;
    ctx.say(gif).await?;

    Ok(())
}

/// Say Hello!
#[poise::command(slash_command, category = "Fun")]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hi!").await?;
    Ok(())
}


/// Fully useless command! Sends an animation meme!
#[poise::command(slash_command, category = "Fun")]
pub async fn sashley(ctx: Context<'_>) -> Result<(), Error> {
    let sashley_link = "https://cdn.discordapp.com/attachments/1010464562434285640/1012690887429591120/HIP___ANIMATION_MEME.mp4";

    ctx.say(sashley_link).await?;
    Ok(())
}

/// Kazakhstan Grozi nam Bombardowaniem!
#[poise::command(slash_command, category = "Fun")]
pub async fn kazakhstan(ctx: Context<'_>) -> Result<(), Error> {
    let kazakh_link = "https://cdn.discordapp.com/attachments/1020620787289423892/1058706507073589268/kazakh.mp4";

    ctx.say(kazakh_link).await?;
    Ok(())
}