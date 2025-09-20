use serenity::all::{Context, Interaction};
use utils::{info, warning};

use crate::{
    ElapsedTime,
    handler::commands::{autocomplete, command, component, modal, ping},
};

pub async fn create(ctx: Context, interaction: Interaction) {
    let timer = ElapsedTime::new();
    let interaction_identification = match interaction {
        Interaction::Command(command) => command::handle(&ctx, command).await,
        Interaction::Autocomplete(autocomplete) => autocomplete::handle(&ctx, autocomplete).await,
        Interaction::Component(component) => component::handle(&ctx, component).await,
        Interaction::Modal(modal) => modal::handle(&ctx, modal).await,
        Interaction::Ping(ping) => ping::handle(&ctx, ping).await,
        _ => None,
    };
    if let Some(name) = interaction_identification {
        info!("Interaction {} handled ({}ms)", name, timer.elapsed_ms())
    } else {
        warning!("Interaction not handled ({}ms)", timer.elapsed_ms());
    }
}
