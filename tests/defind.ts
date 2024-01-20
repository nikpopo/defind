import * as anchor from "@coral-xyz/anchor";
import {Program, setProvider} from "@coral-xyz/anchor";
import { Defind } from "../target/types/defind";
import {expect} from "chai";

describe("defind", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.Defind as Program<Defind>;

  //const counter = anchor.web3.Keypair.generate().publicKey;

  const fund = {
      name: "test",
  }

  const [fundPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(fund.name), provider.wallet.publicKey.toBuffer()],
      program.programId
  );

  const [dataPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("datapda"), provider.wallet.publicKey.toBuffer()],
      program.programId
  );

  it("Bank created!", async  () => {
    const tx = await program.methods
        .create(fund.name)
        .accounts({
          fund: fundPda,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc()

    const account = await program.account.fund.fetch(fundPda);
    console.log(fundPda)
    expect(fund.name == account.name)
  });

  it("Deposit successfully", async () => {
      //const accountDataBefore = await program.account.depositData.fetch(dataPda)
      const depAmount = new anchor.BN(1000000000)
      // const depU64 = 1000000000

      const tx = await program.methods
          .deposit(depAmount)
          .accounts({
              fund: fundPda,
              user: provider.wallet.publicKey,
              data: dataPda,
              //counter: counter,
              systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc()

      console.log("deposit tx: " + dataPda.toBase58())
      const accountData = await program.account.depositData.fetch(dataPda)
      const accountFund = await program.account.fund.fetch(fundPda)
      //expect(accountData.deposits == depAmount.add(accountDataBefore.deposits))
  });

  // it("Withdrawal successfully", async () => {
  //     //const accountDataBefore = await program.account.depositData.fetch(dataPda)
  //     const withdrawalAmount = new anchor.BN(10000)
  //     // const depU64 = 1000000000
  //
  //     const tx = await program.methods
  //         .withdraw(withdrawalAmount)
  //         .accounts({
  //             fund: fundPda,
  //             user: provider.wallet.publicKey,
  //             data: dataPda,
  //             //counter: counter,
  //             systemProgram: anchor.web3.SystemProgram.programId,
  //         })
  //         .rpc()
  //
  //     console.log("withdrawal tx: " + dataPda.toBase58())
  //     const accountData = await program.account.depositData.fetch(dataPda)
  //     expect(accountData.deposits == withdrawalAmount)
  // })

  it("Is initialized!", async () => {})
  //
  // it("Incremented the count", async () => {})
  //
  // it("Deposit successfully", async () => {
  // })
});
