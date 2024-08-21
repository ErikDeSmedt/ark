# A regtest demo

Our software is experimental and it is reckless
to run it with real bitcoin.

Bitcoin Core provides a few other networks
that can be used to test new functionality. In this scenario
we are going to use `regtest`.

## Setting up a regtest environment

Ensure you have installed all optional dependencies described in [compile from source](../02_compile_from_source).
Go to root of the [repository](https://github.com/ark-bitcoin/ark) and
source the `ark_demo.sh` file.

```
source ark_demo.sh
```

This creates a good environment to play with `bark` and `aspd`.
We start with setting up a `regtest` bitcoin-node.
If you already have a node running you can use that one.
You just have to ensure that `txindex` is enabled.

For the purpose of this tutorial it is easier to spin up a new
`regtest` node. The `ark_demo.sh` script has created a handy
alias `bd`

```bash
bd --daemon
```

You can always use `type -a bd` to see what the alias does.
In this case it will tell you that `bd` is an alias for
`bitcoind -regtest -datadir=/ark/test/bitcoindatadir -server -txdindex`.

You can use the `bitcoin-cli` which is aliased to `bcli` to interact
with the node.

```
bcli getnetworkinfo
```

Then we create and configure an ark-server using the `aspd`-command.
Our ark-server will run on `regtest` and use the `bitcoin`-node
we've started a few lines before.

```bash
aspd create \
    --network regtest \
    --datadir ./test/arkdatadir \
    --bitcoind-url $BITCOIND_URL \
    --bitcoind-cookie $BITCOIND_COOKIE
```

The server can be started using

```bash
aspd start --datadir ./test/arkdatadir
```

The server will start working immediately but requires some funds.
You can find the onchain address in the logs and
send some funds to it.

```bash
bcli generatetoaddress 1 <asp-addr>
```

The funds are newly mined and aren't useable for the first 100
blocks. We can generate 100 extra blocks.

```bash
bcli generatetoaddress 100 mtDDKi5mDjZqGzmbnfUnVQ8ZhCPMPVsApj
```

Next, you can start some clients.
To create a client, use the following command:

```bash
bark --datadir ./test/bark1 create \
    --regtest \
    --asp http://localhost:3535 \
    --bitcoind $BITCOIND_URL \
    --bitcoind-cookie $BITCOIND_COOKIE

bark --datadir ./test/bark2 create \
    --regtest \
    --asp http://localhost:3535 \
    --bitcoind $BITCOIND_URL \
    --bitcoind-cookie $BITCOIND_COOKIE
```

Note that clients can receive off-chain Ark transactions without
having any on-chain balance, but a little bit of on-chain money
is needed to perform unilateral exits.

You can fund the wallet using
```bash
BARK1_ADDR=$(bark --datadir ./test/bark1 get-address)
bcli generatetoaddress 1 $BARK1_ADDR
bcli generatetoaddress 100 mtDDKi5mDjZqGzmbnfUnVQ8ZhCPMPVsApj
```

To use the onchain wallets, there are a few commands available:

```bash
BARK2_ADDR=$(bark --datadir ./test/bark2 get-address)
bark --datadir ./test/bark1 send-onchain $BARK2_ADDR "0.1 btc"
bark --datadir ./test/bark2 balance
```

Once we have money, we can onboard into the Ark,
afterwards the balance will also show an off-chain element.

```bash
bark --datadir ./test/bark1 onboard "1 btc"
bark --datadir ./test/bark1 balance
```

Remember that all txs will just be in the mempool if you don't generate blocks
once a while...

```bash
bcli generatetoaddress 1 mtDDKi5mDjZqGzmbnfUnVQ8ZhCPMPVsApj
```

Then, let's send some money off-chain:

```bash
## Should be empty..
BARK2_PK=$(bark --datadir ./test/bark2 get-vtxo-pubkey)
# For now every client has just a single pubkey.
echo "${BARK2_PK}"
bark --datadir ./test/bark1 send-round ${BARK2_PK} "0.1 btc"
bark --datadir ./test/bark2 balance
```

You will notice that there is a slight delay when sending,
this is because the client needs to wait for the start of the
next round.

However, you can also pay out-of-round. These payments
work instantly.

```bash
## Should be empty..
BARK2_PK2=$(bark --datadir ./test/bark2 get-vtxo-pubkey)
# For now every client has just a single pubkey.
echo "${BARK2_PK2}"
bark --datadir ./test/bark1 send-round ${BARK2_PK2} "0.1 btc"
bark --datadir ./test/bark2 balance
```
