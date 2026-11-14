use super::provisioning_tests::fixture;
use super::*;
use std::sync::atomic::Ordering;
#[tokio::test]
async fn crash_after_provider_success_or_before_ready_recovers_without_another_create() {
    for boundary in 0..3 {
        let (spaces, store, directory) = fixture();
        let member = MemberRef::parse("11").unwrap();
        let bot = MemberRef::parse("99").unwrap();
        if boundary == 0 {
            store.fail_candidate_save.store(true, Ordering::Relaxed);
        }
        if boundary == 1 {
            store.fail_ready_save.store(true, Ordering::Relaxed);
        }
        if boundary == 2 {
            directory.fail_inspect.store(true, Ordering::Relaxed);
        }
        assert!(
            spaces
                .onboard(member, spaces.settings.onboarding[0], bot, "fault:create")
                .await
                .is_err()
        );
        let saved = store
            .find(spaces.settings.guild, member)
            .await
            .unwrap()
            .unwrap();
        assert_ne!(saved.state, SpaceState::Ready);
        directory.fail_inspect.store(false, Ordering::Relaxed);
        spaces.reconcile(saved, bot).await.unwrap();
        let recovered = spaces
            .onboard(member, spaces.settings.onboarding[0], bot, "fault:replay")
            .await
            .unwrap();
        assert!(recovered.is_bound_ready());
        assert_eq!(directory.creates.load(Ordering::Relaxed), 1);
    }
}
#[tokio::test]
async fn zero_multiple_hidden_or_replaced_candidates_stay_quarantined() {
    for boundary in 0..3 {
        let (spaces, store, directory) = fixture();
        let member = MemberRef::parse("11").unwrap();
        let bot = MemberRef::parse("99").unwrap();
        store.fail_ready_save.store(true, Ordering::Relaxed);
        assert!(
            spaces
                .onboard(
                    member,
                    spaces.settings.onboarding[0],
                    bot,
                    "fault:ambiguous"
                )
                .await
                .is_err()
        );
        let saved = store
            .find(spaces.settings.guild, member)
            .await
            .unwrap()
            .unwrap();
        {
            let mut channels = directory.channels.lock().unwrap();
            if boundary == 0 {
                channels.clear();
            } else {
                let mut other = channels[0].clone();
                other.channel = ChannelRef::parse("99999").unwrap();
                if boundary == 2 {
                    channels.clear();
                }
                channels.push(other);
            }
        }
        assert!(spaces.reconcile(saved.clone(), bot).await.is_err());
        assert_ne!(store.load(saved.id).await.unwrap().state, SpaceState::Ready);
        assert_eq!(directory.creates.load(Ordering::Relaxed), 1);
    }
}
#[tokio::test]
async fn unknown_incarnation_denies_before_reservation_or_provider_dispatch() {
    let (spaces, store, directory) = fixture();
    directory.members.lock().unwrap()[0].1.joined_at = None;
    assert!(
        spaces
            .onboard(
                MemberRef::parse("11").unwrap(),
                spaces.settings.onboarding[0],
                MemberRef::parse("99").unwrap(),
                "fault:missing"
            )
            .await
            .is_err()
    );
    assert!(store.spaces.lock().unwrap().is_empty());
    assert_eq!(directory.creates.load(Ordering::Relaxed), 0);
}
