# Solana Account Model

## Account
- ### Address （Public Key）
    - **KeyPair**: on Ed25519 curve, sign with private key
    - **PDA(Program Derived Addresses)**: fall off the Ed25519 curve, sign with "seeds", bump and a specific program ID.
- ### Structure
    - lamports
    - data
    - owner
    - executable
    - rent_epoch
- ### Type
    - #### Programs
      - **Native Programs**
        - **System Program**: New Account Creation, Space Allocation, Assign account ownership
        - **BPF Loader**: Loads and runs/updates BPF programs
        - **Token Program**：all the instruction logic for interacting with tokens
          - Token Program
          - Token Extensions Program 
          - Associated Token Program
      - **Custom Programs**: Created by users, executable
      - **Program Account**: stateless, executable
        - **Program Account**: The main account, points to Program Executable Data Account
        - **Program Executable Data Account**: Contains the executable code for the program
        - **Buffer Account**: A temporary account created during program deployment or upgrades
      - **Data Accounts**: state, non-executable
    - #### **Sysvar Accounts**: Predefined
      - Sysvar Rent
  
## Token Accounts
  - SOL
  - SPL
    - **Mint**: Creates SPL Token
    - **Token Account**: Stores token balances for owner
    - **Owner**: The owner of the token account
    - **Associated Token Account（ATA）**: A special account type linked to the owner and token mint


## **Transactions**：atomic
  - Message
    - **Instructions**：ordered processed
      - 3 pieces of information：
        - The address of the program to invoke
        - The accounts the instruction will read from or write to (is_signer, is_writable)
        - Any additional data required by the instruction (e.g. function arguments)
      - **Cross Program Invocation (CPI)**： invokes the instructions of another program
        - Up to a maximum depth of 4 
        - Signer privileges
        - Programs can "sign" on behalf of PDAs derived from their own program ID
        - Execute with invoke/invoke_signed
    - Recent BlockHash
  - Signers

