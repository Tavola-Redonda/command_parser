# command_parser
generic command parser lib
## Features
- Parse any string based on a list of separator characters
## To-do
- [ ] Add uninclusive separation support
- [ ] Add suport for separators with more then one character
- [ ] Add regex support
## Installation

```bash
cargo add --git https://github.com/Tavola-Redonda/command_parser.git
```
## Usage
The only function implemented right now is `generic_parser`. Import the lib and use the function
### Example
```rust
use commnadparser::generic_parser

fn main() {
   let command = "my|command-with.separator"
   let parsed = generic_pareser(command, vec!['|','-','.']);
   println!("{:?}", parsed); // => ["my", "|", "command", "-", "with", ".", "separator"]
}

```
### generic_parser
```rust
fn generic_parser(cmd:&str, seps:Vec<char>) -> Vec<&str>
```
- `cdm`: raw command to be parsed
- `seps`: A `Vec<char>` containing all possible separators
- Returns a `Vec<&str>`
