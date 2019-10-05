# bsdgames-rust
A light-hearted attempt to learn Rust by porting the bsdgames repository, by a couple of colleagues.

# Developing
To start porting a new game:

1. Check out the repo, create a new branch
2. Choose a game to port from the bsdgames collection
3. From the main repo directory, run `cargo new <gamename>`
4. Add `<gamename>` to the `members` array in Cargo.toml in the base repo directory
5. Add the newly created carego subrepo to git: `git add <gamename>` 
6. Commit and push
7. Continue to develop your game under the game repo.
8. Tell everyone what game you're working on. 

The master repo is locked and requires at least one other reviewer to commit to. 
That's a little over the top, but if the goal is to learn rust, code reviews should
help.
