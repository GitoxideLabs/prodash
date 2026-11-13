use prodash::{Count, NestedProgress, Progress, progress::ThroughputOnDrop};

#[test]
fn dyn_safe() {
    fn needs_mut_dyn(_p: &mut dyn Progress) {}
    fn needs_dyn(_p: &dyn Progress) {}
    let root = prodash::tree::Root::new();
    let mut child = root.add_child("hello");
    needs_mut_dyn(&mut child);
    needs_dyn(&child);
    let mut child_of_child = child.add_child("there");
    needs_mut_dyn(&mut child_of_child);
    needs_dyn(&child);
}

#[test]
fn thread_safe() {
    fn needs_send_sync<'a, T: Sync + Send + 'a>(_p: T) {}
    let root = prodash::tree::Root::new();
    let mut child = root.add_child("hello");
    needs_send_sync(&child);
    let child_of_child = child.add_child("there");
    needs_send_sync(&child_of_child);
    needs_send_sync(child_of_child);
    needs_send_sync(child);
}

#[test]
fn throughput_on_drop_with_boxed_progress() {
    let root = prodash::tree::Root::new();
    let leaf: prodash::BoxedProgress = Box::new(root.add_child("leaf"));
    let mut progress = ThroughputOnDrop::new(leaf);
    progress.init(Some(10), None);
    assert_eq!(progress.max(), Some(10));
    assert_eq!(progress.set_max(Some(20)), Some(10));
    assert_eq!(progress.max(), Some(20));
    progress.set_name("renamed leaf".into());
    assert_eq!(progress.name().as_deref(), Some("renamed leaf"));

    progress.set(2);
    progress.inc_by(2);
    progress.inc();
    assert_eq!(progress.step(), 5);
    assert_eq!(progress.counter().load(std::sync::atomic::Ordering::Relaxed), 5);

    let mut messages = Vec::new();
    root.copy_messages(&mut messages);
    assert!(messages.is_empty());

    drop(progress);
    root.copy_messages(&mut messages);
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].origin, "renamed leaf");
    assert_eq!(messages[0].level, prodash::messages::MessageLevel::Info);
    assert!(messages[0].message.starts_with("done 5 items in "));
}

#[test]
fn throughput_on_drop_preserves_dynamic_nesting() {
    let root = prodash::tree::Root::new();
    let mut progress = prodash::BoxedDynNestedProgress::new(ThroughputOnDrop::new(root.add_child("parent")));
    let mut child = prodash::DynNestedProgress::add_child(&mut progress, "child".into());
    let mut grandchild = child.add_child_with_id("grandchild", *b"GRND");
    grandchild.init(Some(10), None);
    grandchild.inc();
    assert_eq!(grandchild.id(), *b"GRND");
    assert_eq!(grandchild.step(), 1);

    drop(grandchild);
    drop(child);
    drop(progress);
    let mut messages = Vec::new();
    root.copy_messages(&mut messages);
    let origins: Vec<_> = messages.iter().map(|message| message.origin.as_str()).collect();
    assert_eq!(origins, ["grandchild", "child", "parent"]);
}
