# Why Ark?

The ark-protocol is a second layer on the bitcoin network. The protocol is fully
self-custodial and users can withdraw their bitcoin unilaterally at any time.
There is a central coordinator which we call the ark-service-provider or `asp`.

## UTXO-sharing using transaction trees

Let's look at a mass-payout where the `asp` pays various amounts to a large number of clients.

A naive `asp` would create single transaction which has one taproot-output for each client.
This means that each output adds `43 vbytes` to the transaction size or a cost
of `215 sat` assuming `5 sat/vbyte`.

The `asp` can save on fees by creating a tree of transactions. Each node in this 
tree is a normal bitcoin transaction. However, only the root of the tree is
confirmed on chain.

The `asp` can construct the tree using simple multi-signatures. The root of the
tree contains all the funds and is signed by all clients. Each child has 2
outputs, and each output is only signed by the clients that have money in
that output. We call the outputs at the leave of the trees `vtxo's` or virtual
transaction outputs.

To construct such a tree the `asp` can construct the transaction, request
signatures from all clients. Once all transactions are signed the `asp` can
safely fund the transaction and publish the root of the tree.

// Include image here

Even if only the root of the tree is confirmed on the blockchain the full tree
is finalized. Every client can spend `vtxo` at any time by revealing their branch of
the tree. We call this a `unilateral exit`.

// Include table of costs

You could argue that the transaction-tree didn't solve a real problem yet. The
`asp` saved some funds when constructing the tree. But the `unilateral exit` is
much more expensive for the client.


## Swap a `vtxo` atomically against a new `vtxo`

The ark-protocol is based on rounds that are coordinated by the `asp`. In each
`round` a new transaction-tree is created. A client can spend `vtxo` by swapping 
it against a `vtxo` in a new round or against an output in an on-chain transaction.

Swapping a `vtxo` doesn't have an onchain footprint and is much cheaper than a
`unilateral exit`. The `asp` provides the swaps in exchange for a small fee. 

To make a payment a client can swap a couple of `vtxo's` against a new `vtxo`
that is owned by the `payee` and another `vtxo` containing the change.

The basic building blocks the swap's are forfeits and connectors. If you are
already familiar with the lightning network you might know how these forfeits
work. The mechanism matches that of penalty transaction in the lightning
network.

To ensure clients can forfeit a `vtxo` we must create the following
smart-contract. ther the client can spend the money after a relative time lock or 
a multisig between the `asp` and client can spend the money immediately.

The client constructs the `forfeit` transaction which has two inputs and an
output that gives all funds to the `aspd`.

The first input is the `vtxo` and has been signed by both the client and the `asp`. 
The client could attempt to spend the `vtxo` but it would be pointless as the `asp` has
plenty of time to publish the `forfeit`-transaction.

The second input is the `connector`. The connector is an output from the round
which will contain the new `vtxo`. This connector ensures that the
`forfeit`-transaction is only valid if the new round exists.

Note, that his mechanism can be used to swap a `vtxo` against a new `vtxo` or to
The round has the following phases
1. `asp` triggers the round
2. all clients tell what `vtxo`'s they want to swap
3. the `asp` constructs the transaction tree
4. The clients sign the transaction tree
5. The `asp` shares all signatures with the client
6. The client constructs and signs all forfeit transactions
7. The `asp` funds the round and ensures it is confirmed on-chain

[^1]: If you are familiar with the lightning network you might have seen forfeit
    transactions before. It is the same thing.

## Boarding the ark

At the moment every round is fully funded by the `asp`. How can clients get
their first `vtxo`? The process of converting a `utxo` into a `vtxo` is called
boarding.

A client can do this at any time and doesn't even need to wait for a round. The
client can construct a transaction that looks exactly like a `vtxo` and request
the `asp` to cosign the transaction. 

The client can fund the transaction and ensure it will be confirmed on-chain.

## Expiry dates allow the `asp` to reclaim funds

The protocol isn't econmically viable yet for the `asp`. 
Every time a swap occurs, the `asp` receives a `vtxo` that is hidden deep inside a
transaction-tree.

The trick is to make `vtox`'s expire. At every level of the three there is a
absolute timeout which is roughly 28 days by default. Once the period expires
the `asp` can claim all funds and use it to fund new rounds.

For clients this means the `vtxo`-expires. However, they can cheaply swap a
`vtxo` that is about to expire against a fresh `vtxo` with a longer lifetime.

