use super::*;
use std::future::pending;

#[tokio::test(start_paused = true)]
async fn interrupt_stops_acceptance_before_bounded_cleanup() {
    for ready in [false, true] {
        let (sender, receiver) = oneshot::channel();
        if ready {
            sender.send(Ok(())).unwrap();
        } else {
            drop(sender);
        }
        let accepting = Arc::new(AtomicBool::new(true));
        let observed = accepting.clone();
        let result = supervise(
            tokio::spawn(pending()),
            receiver,
            async { Ok(()) },
            accepting,
            Instant::now() + Duration::from_secs(30),
            || async { true },
            || async {
                assert!(!observed.load(Ordering::Acquire));
                Ok(())
            },
        )
        .await;
        assert_eq!(result, Ok(()));
    }
}

#[tokio::test(start_paused = true)]
async fn pending_cleanup_is_failure_with_no_surviving_client() {
    let (_sender, receiver) = oneshot::channel();
    let client = tokio::spawn(pending());
    let handle = client.abort_handle();
    let start = Instant::now();
    let result = supervise(
        client,
        receiver,
        async { Ok(()) },
        Arc::new(AtomicBool::new(true)),
        start + Duration::from_secs(30),
        || async { true },
        pending,
    )
    .await;
    assert_eq!(result, Err(SessionError::ShutdownTimeout));
    assert!(start.elapsed() <= Duration::from_secs(10));
    assert!(handle.is_finished());
}

#[tokio::test(start_paused = true)]
async fn fatal_outcome_survives_successful_cleanup() {
    let (sender, receiver) = oneshot::channel();
    sender.send(Err(SessionError::Registration)).unwrap();
    let result = supervise(
        tokio::spawn(pending()),
        receiver,
        pending(),
        Arc::new(AtomicBool::new(true)),
        Instant::now() + Duration::from_secs(30),
        || async { true },
        || async { Ok(()) },
    )
    .await;
    assert_eq!(result, Err(SessionError::Registration));
    let (_sender, receiver) = oneshot::channel();
    let result = supervise(
        tokio::spawn(pending()),
        receiver,
        async { Ok(()) },
        Arc::new(AtomicBool::new(true)),
        Instant::now() + Duration::from_secs(30),
        || async { true },
        || async { Err(SessionError::Shutdown) },
    )
    .await;
    assert_eq!(result, Err(SessionError::Shutdown));
}

#[tokio::test(start_paused = true)]
async fn connected_status_resets_the_observed_loss_deadline() {
    let (sender, receiver) = oneshot::channel();
    sender.send(Ok(())).unwrap();
    let samples = std::cell::Cell::new(0);
    let start = Instant::now();
    let result = supervise(
        tokio::spawn(pending()),
        receiver,
        pending(),
        Arc::new(AtomicBool::new(true)),
        start + Duration::from_secs(30),
        || {
            let sample = samples.get();
            samples.set(sample + 1);
            async move { sample == 5 }
        },
        || async { Ok(()) },
    )
    .await;
    assert_eq!(result, Err(SessionError::RecoveryTimeout));
    assert_eq!(start.elapsed(), Duration::from_secs(36));
}
