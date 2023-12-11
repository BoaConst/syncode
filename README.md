**Group 2 - SynCode:** Demin, Suumil, Helia, Pranay, Shunzhi

# <p align="center">DVCS User Guide & Acceptance Tests</p>

# <p align="center">Part A: DVCS Installation & User Guide</p>

## Installation
1. Git clone the repository and then navigate to the root of the project.
```rust
git clone https://github.com/BoaConst/syncode.git

cd syncode
```
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
- Initialize a repository in the current working directory:
```rust
syncode init
```
- Or initialize a repository in a different directory:
```rust
syncode init <"path/to/another/dir">
```
2. **Clone a Repository (clone):**
- Clone a repository to the current working directory:
```rust
syncode clone <"path/to/source/repo">
```
- Or clone a repository to a different directory:
```rust
syncode clone <"path/to/source/repo"> <"path/to/destination/repo">
```
3. **Add Files to Tracking (add):**
- Add files to be tracked:
```rust
syncode add file1.txt file2.txt
```
4. **Remove Files from Tracking (remove):**
- Remove a file from being tracked:
```rust
syncode add file1.txt file2.txt
```
5. **Commit Changes (commit):**
- Commit the current state of the repository:
```rust
syncode commit "Your commit message"
```
6. **View Repository Status (status):**
- View the status of tracked and untracked files:
```rust
syncode status
```
7. **View Commit Log (log):**
- View the commit history:
```rust
syncode log
```