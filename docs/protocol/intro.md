# Why Ark?

The Ark protocol is a second layer on the bitcoin network. The protocol is fully
self-custodial and users can withdraw their bitcoin unilaterally at any time.
There is a central coordinator which we call the ark-service-provider or ASP.

## UTXO-sharing using transaction trees

Let's look at a mass-payout where the ASP pays various amounts to a large number of clients.

An ASP could create single transaction which has one taproot-output for each client.
This means that each output adds `43 vbytes` to the transaction size or a cost
of `215 sat` assuming `5 sat/vbyte`.

Instead of paying each user separately, the ASP a tree transactions that pays each user.
This tree can be constructed in such a way that any payment is final even if only
the root of the tree is confirmed on-chain. The ASP only has to pay the fee for a a single
transaction with a single taproot output.

![Illustration of a Transaction tree](./img/transaction_tree.png)

The ASP is saving fees at the expense of their clients. A client that wants
to spent the bitcoin has to pay onchain fees for each transaction in their
branch of the tree. I'll promise we'll solve that problem in the next section. Let's ignore that
problem for now and look into how the tree is constructed.

The core idea is that
all funds are secured by a N of N multi-sig that must be signed by all receivers.
Having N of N is great, if you are one of the receivers you have an absolute veto power.

The N of N multisig is signed before the ASP sends any funds to the tree.
Each client has a fully signed transasction and can claim the funds at any time by
broadcasting their transactions and paying onchain fees. We call this a unilateral exit
because their is no need to ask permission from anyone.

We apply this idea recursively. At the root of the tree is a transaction
which has an output that is signed by all receivers. We have a presigned
transaction that splits the funds over two outputs. Each output is locked by
a multisig of all receivers for that output. We repeat this until we reach a leave
where the output is owned by a singel receiver. We call the outputs at the leave a VTXO 
or a virtual transaction output.

To construct such a tree the ASP can construct the transaction, request
signatures from all clients. Once all transactions are signed the ASP can
safely fund the transaction and publish the root of the tree.

This transaction tree is a core building block of Ark. It provides each user
with a mechanism to withdraw their funds unilaterally. This is what makes
Ark self-custodial. 

Generally, we want to reserve the unilateral exit only for rare occasions.
In the next section we'll explain how clients can avoid paying on-chain fees
by swapping their VTXO's.


## Swap VTXO's attomically

The ark-protocol is based on rounds that are coordinated by the ASP. In each
round a new transaction-tree is created. A client can spend VTXO's by swapping 
it against a VTXO in a new round or against an output in an on-chain transaction.

Swapping a VTXO doesn't have an onchain footprint and is much cheaper than a
unilateral exit. The ASP provides the swaps in exchange for a small fee. 

You can use a swap to make a payment. To make a payment you can swap a couple
of VTXO's that are owned by you against one VTXO that is owned by the payee and a second VTXO
containing the change.

The basic building blocks atomic swaps are forfeits and connectors. The forfeit-construct
allows a client to give up or the VTXO. The connector makes the forfeit conditional and ensures
the client will only give up their VTXO if something is offered in return.

### Forfeit transactions

If you are already familiar with the lightning network you might know how these forfeits
work. We use the same mechanism as penalty transaction in the lightning
network.

To support forfeits each VTXO must follow a specific pattern that consists out of 2 transactions.
The first is a taproot transaction wich is locked by a 2 of 2 between the client and the ASP.
This output pays into a pre-signed transaction called the exit transaction. The
client can only spend the exit-transaction after {{ ark.vtxo_exit_delta }} blocks which is
roughly {{ ark.vtxo_exit_delta_description }}. The time lock is relative, this means we only
start counting the blocks once the exit-transaction is confirmed on chain.

At any time, the client can perform a unilateral exit. The only difference is that the client has
to wait roughly {{ ark.vtxo_exit_delta_description }} before the funds can be used.

The client can sign a forfeit transaction and send it to the ASP. This is a transaction
that takes all the funds in the VTXO and sends it to the ASP. A client would attempt
to a unilateral exit on a forfeited VTXO would just waste fees. The asp has {{ ark.vtxo_exit_delta_description }}
to claim the funds before the client can access it.

### Connectors make swap atomic

A connector ensures that if a client forfeits a VTXO they get something in return.
This could either be a UTXO or VTXO in a next round.

The idea is pretty simple. The forfeit transaction can only be confirmed if all inputs
are confirmed. A connector is an output of the round-transaction and an input into the 
forfeit transaction.

If the round would fail it can never make it onchain. By consequence, the forfeit transaction
will never make it onchain as well.

Thanks to the connector, it is safe for the client to sign the forfeiture transaction before 
the round is confirmed.

### Stages of a round

The round has the following phases
1. ASP triggers the round
2. all clients tell what VTXO's they want to swap
3. the ASP constructs the transaction tree and forfeit transactions
4. The clients sign the transaction tree and all forfeit transactions
5. The ASP funds the round and ensures it is confirmed on-chain

## Boarding the ark

A client can spend a UTXO and get a VTXO in return. We call this process onboarding.

The idea is pretty simple. At any point in time the client can ask the server
to cosign a VTXO and fund it.

This process will result in a single on-chain transaction. Typically, the server
will require 6 confirmations before it will allow the VTXO to participate in a round.
This protetects the server from double-spends.

## Expiry dates allow the ASP to reclaim funds

The protocol is great from the perspective of a client. They can just on-board their UTXO.


The protocol isn't econmically viable yet for the ASP. 
Every time a swap occurs, the ASP receives a `vtxo` that is hidden deep inside a
transaction-tree.

The trick is to make `vtox`'s expire. At every level of the three there is a
absolute timeout which is roughly 28 days by default. Once the period expires
the ASP can claim all funds and use it to fund new rounds.

For clients this means the `vtxo`-expires. However, they can cheaply swap a
`vtxo` that is about to expire against a fresh `vtxo` with a longer lifetime.

