use ic_cdk_macros::*;

// DOCS: https://internetcomputer.org/docs/current/developer-docs/build/candid/candid-howto/
#[import(canister = "counter")]
pub struct CandidCounter;

fn main() {
    // CandidCounter::inc()
    // let _call_result: Result<(), _> =
    //     ic_cdk::call(publisher_id, "subscribe", (subscriber,)).await;
}
