# wrench
Just throw a wrench in it!

### How to run
1. Install rust (Through Visual Studio)
2. Build project ```cargo build```
3. Run project ```cargo run <file> [debug=true]``` example: ```cargo run programs/SimpleMath.wrench``` or if you want verbose information run ```cargo run programs/SimpleMath.wrench debug=true```
4. It's recommended to use the rust-analyzer extension in visual studio code

### Commands
- Build project ```cargo build```
- Run project ```cargo run```
- Test project ```cargo test```
- Format project (Removes unnessesary white space) ```cargo fmt```
- Code quality check (Show warnings where poor code quality) ```cargo clippy```