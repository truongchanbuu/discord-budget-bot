use super::provisioning_tests::fixture;
use super::*;
#[tokio::test]
async fn unconfirmed_sharing_expires_once_without_changing_native_access() {
    let (spaces, store, directory) = fixture();
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let target = MemberRef::parse("22").unwrap();
    directory.members.lock().unwrap().push((
        spaces.settings.guild,
        MemberSnapshot {
            member: target,
            joined_at: Some(chrono::Utc::now()),
            human: true,
            pending: false,
            elevated: false,
        },
    ));
    let ready = spaces
        .onboard(owner, spaces.settings.onboarding[0], bot, "sharing:start")
        .await
        .unwrap();
    let op = spaces
        .prepare_viewer(
            ViewerRequest {
                member: owner,
                channel: ready.channel.unwrap(),
                space: ready.id,
                target,
                kind: OperationKind::GrantViewer,
                source: "sharing:grant".into(),
            },
            bot,
        )
        .await
        .unwrap();
    assert!(store.viewers(ready.clone()).await.unwrap().is_empty());
    store
        .operations
        .lock()
        .unwrap()
        .iter_mut()
        .find(|o| o.id == op.id)
        .unwrap()
        .created_at -= chrono::Duration::minutes(6);
    store.expire_prepared(ready.id).await.unwrap();
    store.expire_prepared(ready.id).await.unwrap();
    assert!(
        store
            .load(ready.id)
            .await
            .unwrap()
            .current_operation
            .is_none()
    );
    assert_eq!(
        store
            .events
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.action == AccessAction::Expired)
            .count(),
        1
    );
    assert!(
        spaces
            .confirm_viewer(
                owner,
                ready.channel.unwrap(),
                op.id,
                op.expected_revision,
                bot
            )
            .await
            .is_err()
    );
    assert_eq!(directory.channels.lock().unwrap()[0].overwrites.len(), 3);
}

#[tokio::test]
async fn confirmed_viewer_reads_only_and_revocation_restores_owner_only_profile() {
    let (spaces, store, directory) = fixture();
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let target = MemberRef::parse("22").unwrap();
    directory.members.lock().unwrap().push((
        spaces.settings.guild,
        MemberSnapshot {
            member: target,
            joined_at: Some(chrono::Utc::now()),
            human: true,
            pending: false,
            elevated: false,
        },
    ));
    let ready = spaces
        .onboard(owner, spaces.settings.onboarding[0], bot, "sharing:cycle")
        .await
        .unwrap();
    let channel = ready.channel.unwrap();
    let op = spaces
        .prepare_viewer(
            ViewerRequest {
                member: owner,
                channel,
                space: ready.id,
                target,
                kind: OperationKind::GrantViewer,
                source: "sharing:grant-cycle".into(),
            },
            bot,
        )
        .await
        .unwrap();
    spaces
        .confirm_viewer(owner, channel, op.id.clone(), op.expected_revision, bot)
        .await
        .unwrap();
    spaces
        .confirm_viewer(owner, channel, op.id, op.expected_revision, bot)
        .await
        .unwrap();
    assert_eq!(directory.channels.lock().unwrap()[0].overwrites.len(), 4);
    assert!(spaces.authorize_finance(owner, channel, bot).await.is_ok());
    assert!(
        spaces
            .authorize_finance(target, channel, bot)
            .await
            .is_err()
    );
    let op = spaces
        .prepare_viewer(
            ViewerRequest {
                member: owner,
                channel,
                space: ready.id,
                target,
                kind: OperationKind::RevokeViewer,
                source: "sharing:revoke-cycle".into(),
            },
            bot,
        )
        .await
        .unwrap();
    spaces
        .confirm_viewer(owner, channel, op.id, op.expected_revision, bot)
        .await
        .unwrap();
    assert_eq!(directory.channels.lock().unwrap()[0].overwrites.len(), 3);
    assert_eq!(
        store.viewers(ready).await.unwrap()[0].confirmed,
        ViewerConfirmed::Absent
    );
}
