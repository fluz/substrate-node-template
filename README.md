# Substrate Governance

The purpose of this project is to experiment with the governance capabilities of the Substrate blockchain framework.

## Multi-Signature Accounts

Multi-signature accounts are accounts that cannot be controlled by single private key; in effect, a multi-signature
account is a way to require multiple individuals (or at least individual accounts) to sign an extrinsic before it is
executed. [The FRAME Utility pallet](https://crates.parity.io/pallet_utility/index.html) encapsulates Substrate's
multi-signature capabilities; there are a number of components that are relevant to multi-signature accounts:

* [`multi_account_id`](https://crates.parity.io/pallet_utility/struct.Module.html#method.multi_account_id): Use this
  method [from within Rust code](/node/src/chain_spec.rs#L50) to generate a multi-signature account ID from a _sorted_
  array of account IDs. In the linked example, a multi-signature account ID is being used as the
  [Sudo](https://crates.parity.io/pallet_sudo/index.html) account; this may be a desireable governance mechanism
  because it prevents a single account from controlling root access to the blockchain.
* [`approve_as_multi`](https://crates.parity.io/pallet_utility/enum.Call.html#variant.approve_as_multi) &
  [`as_multi`](https://crates.parity.io/pallet_utility/enum.Call.html#variant.as_multi): These two dispatchable
  functions are generally called from _outside_ the blockchain's runtime, such as by way of
  [the Polkadot Apps UI](https://github.com/polkadot-js/apps). Together, these two functions allow blockchain clients
  to dispatch calls from multi-signature accounts. This capability has been split into two functions in order to reduce
  the costs associated with dispatching calls from multi-signature accounts. In Substrate, dispatchable calls have 
  [fees](https://github.com/substrate-developer-hub/knowledgebase/blob/master/current/runtime/fees.md) associated with
  them, and a call's _length in bytes_ is one parameter that is used to calculate this fee. The `approve_as_multi`
  function accepts **the hash** of the call that you would like to dispatch from the multi-signature account, which is
  fixed in length, as opposed to the call itself, whose length is not bounded. The intention is that only the last
  signer will call `as_multi` and incur the fee associated with the length of the multi-signed call. The example that
  is documented below explains how to use these functions to dispatch a call from a multi-signature account.

## Example

As mentioned above, the genesis configuration for this project specifies a multi-signature Sudo account ID. The steps
provided below will guide you through the process of changing the Sudo key to the account for
[the well-known Alice account](https://github.com/substrate-developer-hub/knowledgebase/blob/master/current/integrate/subkey.md#well-known-keys)
before changing it _back_ to the multi-signature account. Finally, this example will illustrate how to use a
multi-signature account to invoke the `sudo` function from the Sudo pallet and dispatch a call (a runtime upgrade) from
the Root origin.

1. Follow
   [the steps to build and run the Substrate node](https://github.com/substrate-developer-hub/substrate-node-template/tree/v2.0.0-alpha.6#build).
   Note that the node in this project has been renamed to `substrate-governance`, so you will need to substitute that
   for `node-template` where applicable, (i.e. `./target/release/substrate-governance purge-chain --dev` &
   `./target/release/substrate-governance --dev`).
1. Navigate to the `front-end` directory and execute `yarn install && yarn start` to start
   [the included `substrate-front-end-template`](/front-end), which includes utilities for generating the hash of a
   dispatchable call (including a runtime upgrade). This should automatically open a browser tab with the front-end,
   which will be served at [http://localhost:8000](http://localhost:8000).
1. You will also need to use [the Polkadot Apps UI](https://github.com/polkadot-js/apps) to submit extrisics to the
   locally-running Substrate node. This often requires you to run the Polkadot Apps UI locally as well. Open up two
   Polkadot Apps UI browser tabs before moving onto the next step.
1. Use the [the Polkadot Apps UI Chain State app](http://localhost:3000/#/chainstate) to query the `sudo` module's
   `key` value; you should notice that it is the multi-signature account ID that is noted in
   [the chain specification](/node/src/chain_spec.rs#L52).
1. Use [the included Substrate front-end template](http://localhost:8000) to calculate the hash of the call needed to
   change the Sudo key to Alice's account ID. Use the front-end template's Extrinsic component to select the `setKey`
   function from the `sudo` module; put Alice's account ID (`5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY`) into
   the Input field and click the Hash button. Take note of the hexadecimal value that appears beneath the Call and Hash
   buttons; this is the hash of the call. You can examine
   [the source code of the Extrinsic component](/front-end/src/Extrinsics.js#L99) as well as
   [the button's source code](/front-end/src/substrate-lib/components/TxButton.js#L77) to see how the Polkadot JS API
   is used to calculate a call's hash in Javascript.
1. Navigate to [the Extrinsics app](http://localhost:3000/#/extrinsics) in one of the Polkadot UI tabs. Fill out the
   form with the values provided below and click the button marked Submit Transaction.
   * `using the selected account`: Alice - this is the account you use to initialize a dispatch from the
     multi-signature account.
   * `submit the following extrinsic`: `utility > approveAsMulti` - this is the dispatchable call from the Utility
     pallet that is used to initialize a dispatch from a multi-signature account.
   * `threshold`: 4 - this value was configured
     [when the multi-signature account ID was calculated](/node/src/chain_spec.rs#L58).
   * `other_signatories`: Use the five accounts listed below, keeping in mind that _order matters_!
      1. Ferdie
      1. Dave
      1. Bob
      1. Charlie
      1. Eve
   * `maybe_timepoint`: Use the switch labeled `exclude option` to leave this `<empty>` for now.
   * `call_hash`: Use the call hash that you generated using the front-end template in the previous step.

   Refer to the documentation for
   [`approve_as_multi`](https://crates.parity.io/pallet_utility/enum.Call.html#variant.approve_as_multi) to gain a
   better understanding of why you're using these parameters.
1. After clicking the Submit Transaction button and Sign and Submit on the modal that appears after you do so, open
   your browser's console and wait for the log entry that confirms the extrinsic's completion; if all went well, you
   should see `{"InBlock":"<block hash>"}` at the end of this entry. In the other Polkadot Apps UI tab, use the form in
   the upper-right hand corner to search for the block hash that was logged to the console. At the top of the results
   page you will find the **block number**. There is also a section for the extrinsics that were included in the block.
   Since [the template node includes the Timestamp pallet](/runtime/src/lib.rs#L250), you should notice that the entry
   for `utility.approveAsMulti` appears second in the list of extrinsics.
1. Back in the other Polkadot Apps UI tab, you will dispatch the `approve_as_multi` function from Bob's account and
   then from Charlie's. Before doing so, unswitch the `exclude option` switch for the `maybe_timepoint` parameter and
   fill in the fields that appear as follows:
   * `height`: Use the block number from the other Polkadot Apps UI tab.
   * `index`: 1 - this is because the extrinsic that initialized the multi-signature dispatch is the second extrinsic
     in the block, or the extrinsic with index 1.

   You will need to adjust the accounts used as `other_signatories` for both Bob and Charlie to include Alice and
   exclude themselves; the list must always stay sorted with respect to account ID (not the name associated with the
   account). For Bob, that means the accounts should appear in this order Ferdie, Dave, Charlie, Alice, Eve; for
   Charlie, the order is Ferdie, Dave, Bob, Alice, Eve.
1. For the fourth and final signature on the multi-signature dispatch, use Dave's account to call `utility > asMulti`
   with the following parameters:
   * `threshold`: 4 - this is the same value as before.
   * `other_signatories`: Use the five accounts listed below, keeping in mind that _order matters_!
      1. Ferdie
      1. Bob
      1. Charlie
      1. Alice
      1. Eve
   * `maybe_timepoint`: Unswitch the `exclude option` switch and fill in the fields that appear as follows:
      * `height`: Use the block number from the other Polkadot Apps UI tab.
      * `index`: 1 - this is because the extrinsic that initialized the multi-signature dispatch is the second
        extrinsic in the block, or the extrinsic with index 1.
   * `call`: `sudo > setKey` - select Alice's account in the dropdown box that appears.
1. Submit the transaction, then use the other Polkadot Apps UI tab to search for the block with extrinsic that you
   completed in the last step. You should see three events associated with this extrinsic, including
   `system.ExtrinsicSuccess`. In the same tab, use
   [the Polkadot Apps UI Chain State app](http://localhost:3000/#/chainstate) to query the `sudo` module's `key` value;
   you should notice that it is Alice's account ID.
1. Now set the Sudo key back to the multi-signature account ID. Go to
   [the Polkadot Apps UI Extrinsics app](http://localhost:3000/#/extrinsics) and fill out the form as folllows:
   * `using the selected account`: Alice
   * `submit the following extrinsic`: `sudo > setKey`
   * `new`: 5CHGqJCwXiTpEpeJcvMbpubV9r3ixafAMDfnfE7ZuR98s3Qe
1. Next you will make changes to your Substrate runtime to prepare for a forkless upgrade. Add the Collective pallet
   to the runtime's Cargo file:
   ```
   [dependencies.pallet-collective]
   default-features = false
   version = '2.0.0-alpha.6'
   ```
   Don't forget to add `'pallet-collective/std'` to [the `features.std`](/runtime/Cargo.toml#L134) array in the Cargo
   file. Implement the Collective trait in your runtime's `lib.rs` file:
   ```rust
   parameter_types! {
      pub const CouncilMotionDuration: BlockNumber = 5 * DAYS;
   }

   impl pallet_collective::Trait<pallet_collective::Instance1> for Runtime {
      type Origin = Origin;
      type Proposal = Call;
      type Event = Event;
      type MotionDuration = CouncilMotionDuration;
   }
   ```
   Add the new Council Collective to your runtime by adding this to `construct_runtime!` macro in `lib.rs`:
   ```
   Council: pallet_collective::<Instance1>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
   ```
   The last thing you need to do in `lib.rs` is update the runtime version; set the `authoring`, `spec` and `impl`
   versions to `2`. You will find these values near the top of `runtime/lib.rs`; they are members of a constant named
   `VERSIONS`, which is a struct of type
   [RuntimeVersion](https://crates.parity.io/sp_version/struct.RuntimeVersion.html).

## Template

The starting point for this project was
[the Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template).
