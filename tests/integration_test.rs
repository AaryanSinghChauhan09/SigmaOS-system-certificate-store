//! Integration tests for SigmaOS-system-certificate-store.

use sigmaos_system_certificate_store_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
