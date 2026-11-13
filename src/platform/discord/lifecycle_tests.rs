use super::*;
use std::{future::pending, sync::atomic::Ordering};

async fn run_case(
    setup: oneshot::Receiver<Result<(), SessionError>>,
    client: JoinHandle<Result<(), SessionError>>,
    interrupt: impl Future<Output = Result<(), std::io::Error>>,
    connected: bool,
) -> (Result<(), SessionError>, Arc<AtomicBool>) {
    let accepting = Arc::new(AtomicBool::new(true));
    let result = supervise(
        client,
        setup,
        interrupt,
        accepting.clone(),
        Instant::now() + Duration::from_secs(30),
        || async { connected },
        || async { Ok(()) },
    )
    .await;
    assert!(!accepting.load(Ordering::Acquire));
    (result, accepting)
}

#[tokio::test(start_paused = true)]
async fn setup_failure_with_pending_client_is_terminal() {
    for value in [Some(Err(SessionError::Registration)), None] {
        let (sender, receiver) = oneshot::channel();
        if let Some(value) = value {
            sender.send(value).unwrap();
        } else {
            drop(sender);
        }
        let task = tokio::spawn(pending());
        let result = run_case(receiver, task, pending(), true).await.0;
        assert!(matches!(
            result,
            Err(SessionError::Registration | SessionError::Setup)
        ));
    }
}

#[tokio::test(start_paused = true)]
async fn startup_and_observed_recovery_have_absolute_limits() {
    let (_sender, receiver) = oneshot::channel();
    let start = Instant::now();
    assert_eq!(
        run_case(receiver, tokio::spawn(pending()), pending(), true)
            .await
            .0,
        Err(SessionError::StartupTimeout)
    );
    assert_eq!(start.elapsed(), Duration::from_secs(30));
    let (sender, receiver) = oneshot::channel();
    sender.send(Ok(())).unwrap();
    let start = Instant::now();
    assert_eq!(
        run_case(receiver, tokio::spawn(pending()), pending(), false)
            .await
            .0,
        Err(SessionError::RecoveryTimeout)
    );
    assert_eq!(start.elapsed(), Duration::from_secs(30));
}

#[tokio::test(start_paused = true)]
async fn unexpected_client_completion_is_always_failure() {
    for output in [Ok(()), Err(SessionError::Connection)] {
        let (_sender, receiver) = oneshot::channel();
        assert!(
            run_case(
                receiver,
                tokio::spawn(async move { output }),
                pending(),
                true
            )
            .await
            .0
            .is_err()
        );
    }
    let (_sender, receiver) = oneshot::channel();
    let task = tokio::spawn(pending());
    task.abort();
    assert_eq!(
        run_case(receiver, task, pending(), true).await.0,
        Err(SessionError::Task)
    );
}

#[tokio::test(start_paused = true)]
async fn builder_is_bounded_and_interruptible() {
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut interrupt = Box::pin(pending());
    assert!(matches!(
        prepare(pending::<Result<(), ()>>(), interrupt.as_mut(), deadline).await,
        Err(SessionError::StartupTimeout)
    ));
    let mut interrupt = Box::pin(async { Ok(()) });
    assert!(
        prepare(
            pending::<Result<(), ()>>(),
            interrupt.as_mut(),
            Instant::now() + Duration::from_secs(30)
        )
        .await
        .unwrap()
        .is_none()
    );
    let mut interrupt = Box::pin(pending());
    assert_eq!(
        prepare(
            async { Err::<(), _>("provider-marker") },
            interrupt.as_mut(),
            Instant::now() + Duration::from_secs(30)
        )
        .await
        .unwrap_err(),
        SessionError::Build
    );
}

#[tokio::test(start_paused = true)]
async fn task_panic_and_signal_failure_are_terminal() {
    let (_sender, receiver) = oneshot::channel();
    let task = tokio::spawn(async { panic!("controlled task failure") });
    assert_eq!(
        run_case(receiver, task, pending(), true).await.0,
        Err(SessionError::Task)
    );
    let (_sender, receiver) = oneshot::channel();
    let signal = async { Err(std::io::Error::other("synthetic-signal-marker")) };
    assert_eq!(
        run_case(receiver, tokio::spawn(pending()), signal, true)
            .await
            .0,
        Err(SessionError::Signal)
    );
}
