import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorGuess } from "../target/types/anchor_guess";
import { expect } from "chai";

// Wrapper function for testing

async function mutateSecretRPC(program, gameKeypair, tester, new_secret) {
  await program.methods
    .mutateSecret(new_secret)
    .accounts({
      game: gameKeypair.publicKey,
      tester: tester.publicKey,
    })
    .signers(tester instanceof anchor.Wallet ? [] : [tester])
    .rpc();
}

async function guessRPC(program, gameKeypair, player, guess_word) {
  await program.methods
    .guess(guess_word)
    .accounts({
      player: player.publicKey,
      game: gameKeypair.publicKey,
    })
    .signers([])
    .rpc();
}

describe("anchor-guess", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorGuess as Program<AnchorGuess>;

  it("initialize!", async () => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const player = (program.provider as anchor.AnchorProvider).wallet;

    await program.methods
      .setupGame()
      .accounts({
        game: gameKeypair.publicKey,
        player: player.publicKey,
      })
      .signers([gameKeypair])
      .rpc();
    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.moveCount).to.equal(0);
    expect(gameState.player).to.eql(player.publicKey);
    expect(gameState.state).to.eql({ active: {} });
    expect(gameState.secret.length).to.equal(5);
  });

  it("player isn't able to mutate the secret", async () => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const player = (program.provider as anchor.AnchorProvider).wallet;
    await program.methods
      .setupGame()
      .accounts({
        game: gameKeypair.publicKey,
        player: player.publicKey,
      })
      .signers([gameKeypair])
      .rpc();

    // make sure an error is thrown like this:  Error Code: ProhibitedAction. Error Number: 6004. Error Message: ProhibitedAction.
    try {
      await mutateSecretRPC(program, gameKeypair, player, "penta");
    } catch (e) {
      expect(e).to.be.instanceOf(Error);
      expect(e.message).to.contain("ProhibitedAction");
    }
  });

  it("guess that a letter contain but wrong position", async () => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const player = (program.provider as anchor.AnchorProvider).wallet;
    const tester = anchor.web3.Keypair.generate();
    await program.methods
      .setupGame()
      .accounts({
        game: gameKeypair.publicKey,
        player: player.publicKey,
      })
      .signers([gameKeypair])
      .rpc();
    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    await mutateSecretRPC(program, gameKeypair, tester, "penta");
    await program.methods
      .guess("ealll")
      .accounts({
        player: player.publicKey,
        game: gameKeypair.publicKey,
      })
      .signers([])
      .rpc();
    gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.moveCount).to.equal(1);
    expect(gameState.guessState).to.eql([
      { inWordButWrongSpot: {} },
      { inWordButWrongSpot: {} },
      { notInWord: {} },
      { notInWord: {} },
      { notInWord: {} },
    ]);
  });

  it("guess that miss all", async () => {
    // Mock the game state that contain secret "penta"
    const gameKeypair = anchor.web3.Keypair.generate();
    const player = (program.provider as anchor.AnchorProvider).wallet;
    const tester = anchor.web3.Keypair.generate();
    await program.methods
      .setupGame()
      .accounts({
        game: gameKeypair.publicKey,
        player: player.publicKey,
      })
      .signers([gameKeypair])
      .rpc();
    await mutateSecretRPC(program, gameKeypair, tester, "penta");
    await guessRPC(program, gameKeypair, player, "hollr");
    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.moveCount).to.equal(1);
    expect(gameState.guessState).to.eql([
      { notInWord: {} },
      { notInWord: {} },
      { notInWord: {} },
      { notInWord: {} },
      { notInWord: {} },
    ]);
  });
  it("guess correct", async () => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const player = (program.provider as anchor.AnchorProvider).wallet;
    const tester = anchor.web3.Keypair.generate();
    await program.methods
      .setupGame()
      .accounts({
        game: gameKeypair.publicKey,
        player: player.publicKey,
      })
      .signers([gameKeypair])
      .rpc();
    await mutateSecretRPC(program, gameKeypair, tester, "penta");
    await guessRPC(program, gameKeypair, player, "penta");
    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.moveCount).to.equal(1);
    expect(gameState.guessState).to.eql([
      { inCorrectPosition: {} },
      { inCorrectPosition: {} },
      { inCorrectPosition: {} },
      { inCorrectPosition: {} },
      { inCorrectPosition: {} },
    ]);
  });
});
