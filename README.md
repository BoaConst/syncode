# Rust Distributed Version Control System (DVCS)

This project is a simple implementation of a distributed version control system (DVCS) in Rust. 

## Installation
1. Git clone the repository and then navigate to the root of the project.
2. Run the following command to build and install the binary.
```rust
cargo install --path .
```
For Unix-like systems (Linux, MacOS)

- Bash(Linux): Add to ~/.bashrc or ~/.bash_profile

```rust
export PATH="$HOME/.cargo/bin:$PATH"
```

- Zsh (macOs): Add to ~/.zshrc:

```rust
export PATH="$HOME/.cargo/bin:$PATH"
```

After adding the line, apply the changes:

```rust
source ~/.bashrc  # or source ~/.zshrc
```

- Windows: Add C:\Users\<YourUserName>\.cargo\bin to your system's PATH environment variable.

## Usage

1. **Initialize a Repository (init):**
- Initialize a repository in the current directory:
```rust
syncode init
```
- Or in a specified directory:
```rust
syncode init /path/to/directory
```
2. **Add Files to Tracking (add):**
- Add files to be tracked:
```rust
syncode add file1.txt file2.txt
```
3. **Remove Files from Tracking (remove):**
- Remove a file from being tracked:
```rust
syncode add file1.txt file2.txt
```
4. **Commit Changes (commit):**
- Commit the current state of the repository:
```rust
syncode commit "Your commit message"
```
5. **View Repository Status (status):**
- View the status of tracked and untracked files:
```rust
syncode status
```
6. **View Commit Log (log):**
- View the commit history:
```rust
syncode log
```