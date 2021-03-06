use near_primitives::version::ProtocolVersion;
use near_vm_logic::VMLogic;

use std::ffi::c_void;

#[derive(Clone, Copy)]
pub struct ImportReference(pub *mut c_void);
unsafe impl Send for ImportReference {}
unsafe impl Sync for ImportReference {}

#[cfg(feature = "wasmer2_vm")]
use wasmer::{Memory, WasmerEnv};

#[derive(WasmerEnv, Clone)]
#[cfg(feature = "wasmer2_vm")]
pub struct NearWasmerEnv {
    pub memory: Memory,
    pub logic: ImportReference,
}

const fn str_eq(s1: &str, s2: &str) -> bool {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    if s1.len() != s2.len() {
        return false;
    }
    let mut i = 0;
    while i < s1.len() {
        if s1[i] != s2[i] {
            return false;
        }
        i += 1;
    }
    true
}

macro_rules! wrapped_imports {
        ( $($(#[$stable_feature:ident])? $(#[$feature_name:literal, $feature:ident])* $func:ident < [ $( $arg_name:ident : $arg_type:ident ),* ] -> [ $( $returns:ident ),* ] >, )* ) => {

            #[cfg(feature = "wasmer2_vm")]
            pub mod wasmer2_ext {
            use near_vm_logic::VMLogic;
            use crate::imports::NearWasmerEnv;
            use crate::imports::str_eq;

            type VMResult<T> = ::std::result::Result<T, near_vm_logic::VMLogicError>;
            $(
                #[allow(unused_parens)]
                $(#[cfg(feature = $feature_name)])*
                pub fn $func(env: &NearWasmerEnv, $( $arg_name: $arg_type ),* ) -> VMResult<($( $returns ),*)> {
                    const IS_GAS: bool = str_eq(stringify!($func), "gas");
                    let _span = if IS_GAS {
                        None
                    } else {
                        Some(tracing::debug_span!(target: "host-function", stringify!($func)).entered())
                    };
                    let logic: &mut VMLogic = unsafe { &mut *(env.logic.0 as *mut VMLogic<'_>) };
                    logic.$func( $( $arg_name, )* )
                }
            )*
            }

            #[allow(unused_variables)]
            #[cfg(feature = "wasmer2_vm")]
            pub(crate) fn build_wasmer2(
                store: &wasmer::Store,
                memory: wasmer::Memory,
                logic: &mut VMLogic<'_>,
                protocol_version: ProtocolVersion,
            ) -> wasmer::ImportObject {
                let env = NearWasmerEnv {logic: ImportReference(logic as * mut _ as * mut c_void), memory: memory.clone()};
                let mut import_object = wasmer::ImportObject::new();
                let mut namespace = wasmer::Exports::new();
                namespace.insert("memory", memory);
                $({
                    $(#[cfg(feature = $feature_name)])*
                    if true $(&& near_primitives::checked_feature!($feature_name, $feature, protocol_version))* $(&& near_primitives::checked_feature!("stable", $stable_feature, protocol_version))? {
                        namespace.insert(stringify!($func), wasmer::Function::new_native_with_env(&store, env.clone(), wasmer2_ext::$func));
                    }
                })*
                import_object.register("env", namespace);
                import_object
            }
        }
    }

wrapped_imports! {
    // #############
    // # Registers #
    // #############
    read_register<[register_id: u64, ptr: u64] -> []>,
    register_len<[register_id: u64] -> [u64]>,
    write_register<[register_id: u64, data_len: u64, data_ptr: u64] -> []>,
    // ###############
    // # Context API #
    // ###############
    current_account_id<[register_id: u64] -> []>,
    signer_account_id<[register_id: u64] -> []>,
    signer_account_pk<[register_id: u64] -> []>,
    predecessor_account_id<[register_id: u64] -> []>,
    input<[register_id: u64] -> []>,
    // TODO #1903 rename to `block_height`
    block_index<[] -> [u64]>,
    block_timestamp<[] -> [u64]>,
    epoch_height<[] -> [u64]>,
    storage_usage<[] -> [u64]>,
    // #################
    // # Economics API #
    // #################
    account_balance<[balance_ptr: u64] -> []>,
    account_locked_balance<[balance_ptr: u64] -> []>,
    attached_deposit<[balance_ptr: u64] -> []>,
    prepaid_gas<[] -> [u64]>,
    used_gas<[] -> [u64]>,
    // ############
    // # Math API #
    // ############
    random_seed<[register_id: u64] -> []>,
    sha256<[value_len: u64, value_ptr: u64, register_id: u64] -> []>,
    keccak256<[value_len: u64, value_ptr: u64, register_id: u64] -> []>,
    keccak512<[value_len: u64, value_ptr: u64, register_id: u64] -> []>,
    #[MathExtension] ripemd160<[value_len: u64, value_ptr: u64, register_id: u64] -> []>,
    #[MathExtension] ecrecover<[hash_len: u64, hash_ptr: u64, sign_len: u64, sig_ptr: u64, v: u64, malleability_flag: u64, register_id: u64] -> [u64]>,
    // #####################
    // # Miscellaneous API #
    // #####################
    value_return<[value_len: u64, value_ptr: u64] -> []>,
    panic<[] -> []>,
    panic_utf8<[len: u64, ptr: u64] -> []>,
    log_utf8<[len: u64, ptr: u64] -> []>,
    log_utf16<[len: u64, ptr: u64] -> []>,
    abort<[msg_ptr: u32, filename_ptr: u32, line: u32, col: u32] -> []>,
    // ################
    // # Promises API #
    // ################
    promise_create<[
        account_id_len: u64,
        account_id_ptr: u64,
        method_name_len: u64,
        method_name_ptr: u64,
        arguments_len: u64,
        arguments_ptr: u64,
        amount_ptr: u64,
        gas: u64
    ] -> [u64]>,
    promise_then<[
        promise_index: u64,
        account_id_len: u64,
        account_id_ptr: u64,
        method_name_len: u64,
        method_name_ptr: u64,
        arguments_len: u64,
        arguments_ptr: u64,
        amount_ptr: u64,
        gas: u64
    ] -> [u64]>,
    promise_and<[promise_idx_ptr: u64, promise_idx_count: u64] -> [u64]>,
    promise_batch_create<[account_id_len: u64, account_id_ptr: u64] -> [u64]>,
    promise_batch_then<[promise_index: u64, account_id_len: u64, account_id_ptr: u64] -> [u64]>,
    // #######################
    // # Promise API actions #
    // #######################
    promise_batch_action_create_account<[promise_index: u64] -> []>,
    promise_batch_action_deploy_contract<[promise_index: u64, code_len: u64, code_ptr: u64] -> []>,
    promise_batch_action_function_call<[
        promise_index: u64,
        method_name_len: u64,
        method_name_ptr: u64,
        arguments_len: u64,
        arguments_ptr: u64,
        amount_ptr: u64,
        gas: u64
    ] -> []>,
    promise_batch_action_transfer<[promise_index: u64, amount_ptr: u64] -> []>,
    promise_batch_action_stake<[
        promise_index: u64,
        amount_ptr: u64,
        public_key_len: u64,
        public_key_ptr: u64
    ] -> []>,
    promise_batch_action_add_key_with_full_access<[
        promise_index: u64,
        public_key_len: u64,
        public_key_ptr: u64,
        nonce: u64
    ] -> []>,
    promise_batch_action_add_key_with_function_call<[
        promise_index: u64,
        public_key_len: u64,
        public_key_ptr: u64,
        nonce: u64,
        allowance_ptr: u64,
        receiver_id_len: u64,
        receiver_id_ptr: u64,
        method_names_len: u64,
        method_names_ptr: u64
    ] -> []>,
    promise_batch_action_delete_key<[
        promise_index: u64,
        public_key_len: u64,
        public_key_ptr: u64
    ] -> []>,
    promise_batch_action_delete_account<[
        promise_index: u64,
        beneficiary_id_len: u64,
        beneficiary_id_ptr: u64
    ] -> []>,
    // #######################
    // # Promise API results #
    // #######################
    promise_results_count<[] -> [u64]>,
    promise_result<[result_idx: u64, register_id: u64] -> [u64]>,
    promise_return<[promise_idx: u64] -> []>,
    // ###############
    // # Storage API #
    // ###############
    storage_write<[key_len: u64, key_ptr: u64, value_len: u64, value_ptr: u64, register_id: u64] -> [u64]>,
    storage_read<[key_len: u64, key_ptr: u64, register_id: u64] -> [u64]>,
    storage_remove<[key_len: u64, key_ptr: u64, register_id: u64] -> [u64]>,
    storage_has_key<[key_len: u64, key_ptr: u64] -> [u64]>,
    storage_iter_prefix<[prefix_len: u64, prefix_ptr: u64] -> [u64]>,
    storage_iter_range<[start_len: u64, start_ptr: u64, end_len: u64, end_ptr: u64] -> [u64]>,
    storage_iter_next<[iterator_id: u64, key_register_id: u64, value_register_id: u64] -> [u64]>,
    // Function for the injected gas counter. Automatically called by the gas meter.
    gas<[gas_amount: u32] -> []>,
    // ###############
    // # Validator API #
    // ###############
    validator_stake<[account_id_len: u64, account_id_ptr: u64, stake_ptr: u64] -> []>,
    validator_total_stake<[stake_ptr: u64] -> []>,
}
