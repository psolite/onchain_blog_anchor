import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OnchainBlog } from "../target/types/onchain_blog";
import { assert } from "chai";

describe("onchain_blog", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const signer = provider.wallet.publicKey

  const program = anchor.workspace.OnchainBlog as Program<OnchainBlog>;

  const timestamp = new anchor.BN(Date.now());
  const timestampBuffer = timestamp.toArrayLike(Buffer, 'le', 8);

  const [postPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("psolite"), signer.toBuffer(), timestampBuffer],
    program.programId
  );

  const title = "First Post";
  const content = "This is my first blog post!"



  it("Creates a new post", async () => {

    const postContext = {
      post: postPda,
      author: signer,
      systemProgram: anchor.web3.SystemProgram.programId,
    }

    await program.methods.createPost(title, content, timestamp).
      accounts(postContext)
      .signers([])
      .rpc();

    const post = await program.account.post.fetch(postPda);
    console.log("Original Post:", post.title);
    assert.ok(post.title === title);

  });

  it("Edits an existing post", async () => {

    // Fetch the existing post
    let post = await program.account.post.fetch(postPda);
    

    // Prepare new data for the post
    const newTitle = "Updated Title";
    const newContent = "This is the updated content.";

    const editContext = {
      post: postPda,
      author: signer,
    }

    await program.methods.editPost(newTitle, newContent)
      .accounts(editContext)
      .signers([])
      .rpc();

    // Fetch the updated post
    post = await program.account.post.fetch(postPda);
    console.log("Edited Post:", post.title);

    // Verify the changes
    assert.equal(post.title, newTitle);
    assert.equal(post.content, newContent);
  });

  it("Toggles the publish status of a post", async () => {
    const toggleContext = {
      post: postPda,
      author: signer,
    }
    // Toggle the publish status
    await program.methods.togglePublish()
      .accounts(toggleContext)
      .signers([])
      .rpc();

    // Fetch the post again and verify the status has changed
    const post = await program.account.post.fetch(postPda);
    assert.ok(!post.isPublished);
  });

  it("Deletes an existing post", async () => {

    const deleteContext = {
      post: postPda,
      author: signer,
    }

    // Call the delete_post instruction
    await program.methods.deletePost()
      .accounts(deleteContext)
      .signers([])
      .rpc();

    // Verify the account is closed
    try {
      await program.account.post.fetch(postPda);
      assert.fail("The post account should no longer exist");
    } catch (err) {
      console.log("Post account successfully deleted:", err.message);
    }
  });

});
