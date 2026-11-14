use super::provisioning_tests::fixture;
use super::*;
#[tokio::test]
async fn departing_viewer_is_removed_natively_and_rejoin_never_restores_access() {
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
        .onboard(owner, spaces.settings.onboarding[0], bot, "depart:start")
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
                source: "depart:grant".into(),
            },
            bot,
        )
        .await
        .unwrap();
    spaces
        .confirm_viewer(owner, channel, op.id, op.expected_revision, bot)
        .await
        .unwrap();
    spaces
        .invalidate(LifecycleEvent::MemberRemoved(target))
        .await
        .unwrap();
    directory.members.lock().unwrap()[1].1.joined_at =
        Some(chrono::Utc::now() + chrono::Duration::seconds(1));
    spaces
        .reconcile(store.load(ready.id).await.unwrap(), bot)
        .await
        .unwrap();
    assert_eq!(directory.channels.lock().unwrap()[0].overwrites.len(), 3);
    assert_eq!(
        store
            .viewers(store.load(ready.id).await.unwrap())
            .await
            .unwrap()[0]
            .confirmed,
        ViewerConfirmed::Absent
    );
    spaces
        .invalidate(LifecycleEvent::MemberChanged(target))
        .await
        .unwrap();
    spaces
        .reconcile(store.load(ready.id).await.unwrap(), bot)
        .await
        .unwrap();
    assert_eq!(directory.channels.lock().unwrap()[0].overwrites.len(), 3);
    assert!(spaces.authorize_finance(owner, channel, bot).await.is_ok());
    assert!(
        spaces
            .authorize_finance(target, channel, bot)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn elevated_viewer_is_removed_and_uncertain_writes_cannot_be_reversed() {
    for uncertain in [false, true] {
        let (spaces, store, directory) = fixture();
        let member = MemberRef::parse("11").unwrap();
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
            .onboard(
                member,
                spaces.settings.onboarding[0],
                bot,
                "elevated:initial",
            )
            .await
            .unwrap();
        let channel = ready.channel.unwrap();
        let op = spaces
            .prepare_viewer(
                ViewerRequest {
                    member,
                    channel,
                    space: ready.id,
                    target,
                    kind: OperationKind::GrantViewer,
                    source: "elevated:grant".into(),
                },
                bot,
            )
            .await
            .unwrap();
        if uncertain {
            directory
                .fail_change
                .store(true, std::sync::atomic::Ordering::Relaxed);
            assert!(
                spaces
                    .confirm_viewer(member, channel, op.id, op.expected_revision, bot)
                    .await
                    .is_err()
            );
            let calls = directory.changes.load(std::sync::atomic::Ordering::Relaxed);
            assert!(
                spaces
                    .reconcile(store.load(ready.id).await.unwrap(), bot)
                    .await
                    .is_err()
            );
            assert_eq!(
                directory.changes.load(std::sync::atomic::Ordering::Relaxed),
                calls
            );
            assert!(
                spaces
                    .authorize_finance(member, channel, bot)
                    .await
                    .is_err()
            );
        } else {
            spaces
                .confirm_viewer(member, channel, op.id, op.expected_revision, bot)
                .await
                .unwrap();
            directory.members.lock().unwrap()[1].1.elevated = true;
            spaces.invalidate(LifecycleEvent::VerifyAll).await.unwrap();
            spaces
                .reconcile(store.load(ready.id).await.unwrap(), bot)
                .await
                .unwrap();
            assert_eq!(directory.channels.lock().unwrap()[0].overwrites.len(), 3);
        }
    }
}
