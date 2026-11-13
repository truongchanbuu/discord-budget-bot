use super::{FinanceEvents, modals, parse_reference, send, serenity, views};
use crate::features::ledger::{
    application::{Action, edit},
    domain::{JournalError, State},
};
use std::time::Duration;

pub(super) async fn button(
    handler: &FinanceEvents,
    context: &serenity::Context,
    component: &serenity::ComponentInteraction,
) -> Result<(), JournalError> {
    if component.guild_id.is_some() || !handler.accepting.load(std::sync::atomic::Ordering::Acquire)
    {
        return Ok(());
    }
    let actor = match super::actor(&component.user, component.guild_id, &handler.binding) {
        Ok(actor) => actor,
        Err(error) => {
            send(component.create_response(&context.http, views::private_error(error.to_string())))
                .await?;
            return Ok(());
        }
    };
    let reference = match parse_reference(&component.data.custom_id) {
        Ok(reference) => reference,
        Err(error) => {
            send(component.create_response(&context.http, views::private_error(error.to_string())))
                .await?;
            return Ok(());
        }
    };
    let channel = component.channel_id.to_string();
    if reference.action == "edit" {
        return tokio::time::timeout(Duration::from_secs(2), async {
            let response = match handler
                .journal
                .draft(
                    handler.scope(&actor, &channel),
                    reference.id,
                    reference.revision,
                )
                .await
            {
                Ok(draft) if draft.state == State::Pending => {
                    serenity::CreateInteractionResponse::Modal(modals::builder(&draft))
                }
                Ok(_) => views::private_error(JournalError::Stale.to_string()),
                Err(error) => views::private_error(error.to_string()),
            };
            send(component.create_response(&context.http, response)).await?;
            Ok(())
        })
        .await
        .map_err(|_| JournalError::Timeout)?;
    }
    let action = match reference.action {
        "save" => Action::Confirm,
        "cancel" => Action::Cancel,
        _ => {
            send(component.create_response(
                &context.http,
                views::private_error(JournalError::InvalidInput.to_string()),
            ))
            .await?;
            return Ok(());
        }
    };
    let result =
        super::acknowledged_action(send(component.defer_ephemeral(&context.http)), async {
            if !handler.accepting.load(std::sync::atomic::Ordering::Acquire) {
                return Err(JournalError::Denied);
            }
            handler
                .journal
                .act(
                    handler.scope(&actor, &channel),
                    reference.id,
                    reference.revision,
                    action,
                )
                .await
        })
        .await?;
    let (text, buttons) = match result {
        Ok(result) => views::result(result),
        Err(error) => (error.to_string(), vec![]),
    };
    send(component.edit_response(&context.http, views::edited(text, buttons))).await?;
    Ok(())
}

pub(super) async fn modal(
    handler: &FinanceEvents,
    context: &serenity::Context,
    modal: &serenity::ModalInteraction,
) -> Result<(), JournalError> {
    if modal.guild_id.is_some() || !handler.accepting.load(std::sync::atomic::Ordering::Acquire) {
        return Ok(());
    }
    let actor = match super::actor(&modal.user, modal.guild_id, &handler.binding) {
        Ok(actor) => actor,
        Err(error) => {
            send(modal.create_response(&context.http, views::private_error(error.to_string())))
                .await?;
            return Ok(());
        }
    };
    let reference = parse_reference(&modal.data.custom_id)?;
    if reference.action != "modal" {
        return Err(JournalError::InvalidInput);
    }
    let channel = modal.channel_id.to_string();
    let result = super::acknowledged_action(send(modal.defer_ephemeral(&context.http)), async {
        if !handler.accepting.load(std::sync::atomic::Ordering::Acquire) {
            return Err(JournalError::Denied);
        }
        let draft = handler
            .journal
            .draft(
                handler.scope(&actor, &channel),
                reference.id,
                reference.revision,
            )
            .await?;
        let proposal = edit(draft.proposal, &modals::values(&modal.data)?)?;
        handler
            .journal
            .act(
                handler.scope(&actor, &channel),
                reference.id,
                reference.revision,
                Action::Edit(proposal),
            )
            .await
    })
    .await?;
    let (text, buttons) = match result {
        Ok(value) => views::result(value),
        Err(error) => (error.to_string(), vec![]),
    };
    send(modal.edit_response(&context.http, views::edited(text, buttons))).await?;
    Ok(())
}
