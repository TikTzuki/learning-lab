fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}

#[cfg(test)]
mod tests {
    use crate::tests::runtime_args::RuntimeArgs;
use casper_engine_test_support::{DEFAULT_ACCOUNT_ADDR, ExecuteRequestBuilder, InMemoryWasmTestBuilder, PRODUCTION_RUN_GENESIS_REQUEST};
    use casper_types::{ContractHash, runtime_args};

    //
    // // Define `KEY` constant to match that in the contract.
    // const KEY: &str = "my-key-name";
    // const VALUE: &str = "hello world";
    // const RUNTIME_ARG_NAME: &str = "message";
    // const CONTRACT_WASM: &str = "contract.wasm";
    //
    // #[test]
    // fn should_store_hello_world() {
    //     let mut builder = InMemoryWasmTestBuilder::default();
    //     builder
    //         .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
    //         .commit();
    //
    //     // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
    //     // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
    //     // absolute paths.
    //     let session_code = PathBuf::from(CONTRACT_WASM);
    //     let session_args = runtime_args! {
    //         RUNTIME_ARG_NAME => VALUE,
    //     };
    //
    //     let deploy_item = DeployItemBuilder::new()
    //         .with_empty_payment_bytes(runtime_args! {
    //             ARG_AMOUNT => *DEFAULT_PAYMENT
    //         })
    //         .with_session_code(session_code, session_args)
    //         .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
    //         .with_address(*DEFAULT_ACCOUNT_ADDR)
    //         .build();
    //
    //     let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();
    //
    //     // prepare assertions.
    //     let result_of_query = builder.query(
    //         None,
    //         Key::Account(*DEFAULT_ACCOUNT_ADDR),
    //         &[KEY.to_string()],
    //     );
    //     assert!(result_of_query.is_err());
    //
    //     // deploy the contract.
    //     builder.exec(execute_request).commit().expect_success();
    //
    //     // make assertions
    //     let result_of_query = builder
    //         .query(
    //             None,
    //             Key::Account(*DEFAULT_ACCOUNT_ADDR),
    //             &[KEY.to_string()],
    //         )
    //         .expect("should be stored value.")
    //         .as_cl_value()
    //         .expect("should be cl value.")
    //         .clone()
    //         .into_t::<String>()
    //         .expect("should be string.");
    //
    //     assert_eq!(result_of_query, VALUE);
    // }
    //
    // #[test]
    // fn should_error_on_missing_runtime_arg() {
    //     let session_code = PathBuf::from(CONTRACT_WASM);
    //     let session_args = RuntimeArgs::new();
    //
    //     let deploy_item = DeployItemBuilder::new()
    //         .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
    //         .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
    //         .with_address(*DEFAULT_ACCOUNT_ADDR)
    //         .with_session_code(session_code, session_args)
    //         .build();
    //
    //     let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();
    //
    //     let mut builder = InMemoryWasmTestBuilder::default();
    //     builder
    //         .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
    //         .commit();
    //     builder.exec(execute_request).commit().expect_failure();
    //
    //     let actual_error = builder.get_error().expect("must have error");
    //     assert!(
    //         matches!(
    //             actual_error,
    //             EngineStateError::Exec(execution::Error::Revert(ApiError::MissingArgument))
    //         ),
    //         "Expected {:?}, received {:?}",
    //         EngineStateError::Exec(execution::Error::Revert(ApiError::MissingArgument)),
    //         actual_error
    //     );
    // }

    // Contract Wasm File Paths (Constants)
    const COUNTER_V1_WASM: &str = "counter-v1.wasm";
    const COUNTER_V2_WASM: &str = "counter-v2.wasm";
    const COUNTER_V3_WASM: &str = "counter-v3.wasm";
    const COUNTER_CALL_WASM: &str = "counter-call.wasm";

    // Contract Storage Keys (Constants)
    const CONTRACT_KEY: &str = "counter";
    const COUNT_KEY: &str = "count";
    const LAST_UPDATED_KEY: &str = "last_updated";
    const CONTRACT_VERSION_KEY: &str = "version";

    // Contract Entry Points (Constants)
    const ENTRY_POINT_COUNTER_DECREMENT: &str = "counter_decrement";
    const ENTRY_POINT_COUNTER_INC: &str = "counter_inc";
    const ENTRY_POINT_COUNTER_LAST_UPDATED_AT: &str = "counter_last_updated_at";

    #[test]
    fn install_version1_and_check_entry_points() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST).commit();

        let contract_v1_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_V1_WASM,
            runtime_args! {},
        )
            .build();

        builder
            .exec(contract_v1_installation_request)
            .expect_success()
            .commit();

        let contract_v1_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("should be a valid contract hash");

        // Call the decrement entry point, which should not be in version 1 before the upgrade.
        let contract_decrement_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v1_hash,
            ENTRY_POINT_COUNTER_DECREMENT,
            runtime_args! {},
        )
            .build();

        // Try executing the decrement entry point and expect an error.
        builder
            .exec(contract_decrement_request)
            .expect_failure()
            .commit();

        let session_code_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! {
                CONTRACT_KEY => contract_v1_hash,
            },
        )
            .build();

        builder.exec(session_code_request)
            .expect_success()
            .commit();

        // Verify the value of count is now 1.
        // let incremented_count = builder
        //     .query(None, COUNT_KEY, &[])
        //     .expect("should be stored value.")
        //     .as_cl_value()
        //     .expect("should be cl value.")
        //     .clone()
        //     .into_t::<i32>()
        //     .expect("should be i32.");
        //
        // assert_eq!(incremented_count, 1);
    }
}
