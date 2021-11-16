use std::fs;
use std::process::Command;

use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

pub fn compile(name: &str) -> Vec<u8> {
    Command::new("cargo")
        .current_dir(format!("./tests/{}", name))
        .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
        .status()
        .unwrap();
    fs::read(format!(
        "./tests/{}/target/wasm32-unknown-unknown/release/out.wasm",
        name
    ))
    .unwrap()
}

#[test]
fn test_package() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(&compile("package"));

    let transaction1 = TransactionBuilder::new(&executor)
        .call_function(
            package,
            "PackageTest",
            "publish_package",
            vec![],
            Some(account),
        )
        .build(vec![key])
        .unwrap();
    let receipt1 = executor.run(transaction1, true).unwrap();
    assert!(receipt1.success);
}

#[test]
fn test_context() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(&compile("context"));

    let transaction1 = TransactionBuilder::new(&executor)
        .call_function(package, "ContextTest", "query", vec![], Some(account))
        .build(vec![key])
        .unwrap();
    let receipt1 = executor.run(transaction1, true).unwrap();
    assert!(receipt1.success);
}

#[test]
fn test_component() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(&compile("component"));

    // Create component
    let transaction1 = TransactionBuilder::new(&executor)
        .call_function(
            package,
            "ComponentTest",
            "create_component",
            vec![],
            Some(account),
        )
        .build(vec![key])
        .unwrap();
    let receipt1 = executor.run(transaction1, true).unwrap();
    assert!(receipt1.success);

    // Find the component address from receipt
    let component = receipt1.component(0).unwrap();

    // Call functions & methods
    let transaction2 = TransactionBuilder::new(&executor)
        .call_function(
            package,
            "ComponentTest",
            "get_component_blueprint",
            vec![component.to_string()],
            Some(account),
        )
        .call_method(component, "get_component_state", vec![], Some(account))
        .call_method(component, "put_component_state", vec![], Some(account))
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2, true).unwrap();
    assert!(receipt2.success);
}

#[test]
fn test_lazy_map() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(&compile("lazy_map"));

    let transaction = TransactionBuilder::new(&executor)
        .call_function(
            package,
            "LazyMapTest",
            "test_lazy_map",
            vec![],
            Some(account),
        )
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    assert!(receipt.success);
}

#[test]
fn test_resource_def() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(&compile("resource_def"));

    let transaction = TransactionBuilder::new(&executor)
        .call_function(
            package,
            "ResourceTest",
            "create_mutable_token",
            vec![],
            Some(account),
        )
        .call_function(
            package,
            "ResourceTest",
            "create_fixed_token",
            vec![],
            Some(account),
        )
        .call_function(
            package,
            "ResourceTest",
            "create_mutable_badge",
            vec![],
            Some(account),
        )
        .call_function(
            package,
            "ResourceTest",
            "create_fixed_badge",
            vec![],
            Some(account),
        )
        .call_function(package, "ResourceTest", "query", vec![], Some(account))
        .call_function(package, "ResourceTest", "burn", vec![], Some(account))
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    println!("{:?}", receipt);
    assert!(receipt.success);

    let transaction = TransactionBuilder::new(&executor)
        .call_function(
            package,
            "ResourceTest",
            "create_fixed_badge_should_fail",
            vec![],
            Some(account),
        )
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    println!("{:?}", receipt);
    assert!(!receipt.success);
}

#[test]
fn test_bucket() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(&compile("bucket"));

    let transaction = TransactionBuilder::new(&executor)
        .call_function(package, "BucketTest", "combine", vec![], Some(account))
        .call_function(package, "BucketTest", "split", vec![], Some(account))
        .call_function(package, "BucketTest", "borrow", vec![], Some(account))
        .call_function(package, "BucketTest", "query", vec![], Some(account))
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    assert!(receipt.success);
}

#[test]
fn test_badge() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(&compile("badge"));

    let transaction = TransactionBuilder::new(&executor)
        .call_function(package, "BadgeTest", "combine", vec![], Some(account))
        .call_function(package, "BadgeTest", "split", vec![], Some(account))
        .call_function(package, "BadgeTest", "borrow", vec![], Some(account))
        .call_function(package, "BadgeTest", "query", vec![], Some(account))
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    assert!(receipt.success);
}

#[test]
fn test_call() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(&compile("call"));

    let transaction = TransactionBuilder::new(&executor)
        .call_function(package, "MoveTest", "move_bucket", vec![], Some(account))
        .call_function(
            package,
            "MoveTest",
            "move_bucket_ref",
            vec![],
            Some(account),
        )
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    assert!(receipt.success);
}
