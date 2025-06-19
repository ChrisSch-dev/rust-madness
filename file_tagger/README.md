```markdown
# File Tagger CLI

A simple tool to add, remove, and list tags for files using a local `.tags.json` in each directory.

## Usage

```sh
# Add a tag
cargo run -- add myfile.txt important

# Remove a tag
cargo run -- remove myfile.txt important

# List tags
cargo run -- list myfile.txt
```
```