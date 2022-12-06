# How to Run CarleChain

__IMPORTANT:__ Ensure your environment has Rust & Cargo installed!

1. Clone our repository: 

    `git clone https://github.com/fresleven/CarleChain.git`

2. Navigate to the CarleChain directory:

    `cd CarleChain`

3. Run code based on your choice:
    - Logistic regression example:

        `cargo run --release`

    - Single threading:

        `cargo run --bin single --release`
    
    - Multi-threading:

        `cargo run --bin multi --release`

4. Wait for code the project to run (__Note: building may take a while__).