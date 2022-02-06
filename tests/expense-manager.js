const anchor = require("@project-serum/anchor");

const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 Starting test...");

  const provider = anchor.Provider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.ExpenseManager;

  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("💰 Your base account", account.totalAmount.toString());

  await program.rpc.spendAmt(10,{
    accounts:{
      baseAccount: baseAccount.publicKey,
      user:provider.wallet.publicKey
    }
  })

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);

  console.log('💰 Your base account', account.totalAmount.toString());

  console.log('🚀 Test completed',account.spendList);

};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
