use serenity::all::{
    CacheHttp, ComponentInteraction, ComponentInteractionDataKind, Context,
    CreateInteractionResponse, EditMessage, UserId,
};
use utils::{PaginationAction, error, parse_button_id};

use crate::Paginations;

pub async fn handle(ctx: &Context, component: ComponentInteraction) -> Option<String> {
    let Some(member) = &component.member else {
        return None;
    };

    let mut message = component.clone().message;

    let custom_id = component.data.custom_id.as_str();

    tokio::spawn({
        let ctx = ctx.clone();
        let component = component.clone();
        async move {
            if let Err(why) = component
                .create_response(ctx.http(), CreateInteractionResponse::Acknowledge)
                .await
            {
                error!("Error creating interaction response: {:?}", why);
            }
        }
    });

    match &component.data.kind {
        ComponentInteractionDataKind::Button => {
            let (section, id, user_i, action) = parse_button_id(custom_id)?;

            let pages = {
                let data = ctx.data.read().await;
                data.get::<Paginations>()?.clone()
            };

            if let Some(page) = pages.get(&id).await {
                let response = match action {
                    PaginationAction::Next => page.write().await.next_page(),
                    PaginationAction::Previous => page.write().await.prev_page(),
                };

                if let Some((embed, components)) = response {
                    let edit_message = EditMessage::new()
                        .embeds(vec![embed])
                        .components(vec![components]);
                    if let Err(why) = message.edit(ctx.http(), edit_message).await {
                        error!("Error editing message: {:?}", why);
                    }
                }
            }

            Some(custom_id.to_string())
        }
        _ => None,
    }
}
