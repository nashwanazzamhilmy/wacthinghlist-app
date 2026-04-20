# Stellar Movie Watchlist DApp

**Stellar Movie Watchlist DApp** – A Blockchain-Based Decentralized Movie Tracking System.

## Project Description

Stellar Movie Watchlist DApp is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. This project provides a secure, transparent, and immutable platform for managing personal movie watchlists directly on the blockchain. 

By utilizing Soroban's smart contract capabilities, this DApp ensures that your movie collection and ratings are stored permanently, eliminating reliance on centralized database providers. Your cinematic history becomes a sovereign digital asset, managed entirely through verifiable code.

## Project Vision

Our vision is to redefine personal media management in the digital age by:

* **Decentralizing Entertainment Data**: Shifting watchlist management from centralized silos to a global, distributed ledger.
* **Ensuring Data Ownership**: Empowering users with total control over their viewing history and personal preferences.
* **Guaranteeing Immutability**: Providing a permanent, tamper-proof record for every movie rating and entry.
* **Enhancing Privacy**: Leveraging blockchain security to protect personal data from unauthorized third-party access.
* **Building Trustless Systems**: Creating a platform where data integrity is guaranteed by immutable code, not corporate promises.

## Key Features

### 1. **Seamless Movie Management**
* Add new movies with details: Title, Genre, and Rating.
* Automated unique ID generation for each entry using Soroban's PRNG.
* Persistent and secure storage on the Stellar blockchain.

### 2. **Efficient Data Retrieval**
* Fetch the entire watchlist in a single function call.
* Structured data representation for easy frontend integration.
* Real-time synchronization with the latest blockchain state.

### 3. **Dynamic Rating Updates**
* Update movie ratings on-chain after watching.
* Reflect changing opinions transparently within the ledger.

### 4. **Secure Entry Removal**
* Delete specific movies from the list using their unique IDs.
* Permanent removal from contract storage to maintain data efficiency.
* Clean storage management to optimize transaction fees.

### 5. **Stellar Network Performance**
* Leverages the high speed and ultra-low costs of the Stellar network.
* Built using the modern and robust Soroban Smart Contract framework.

## Contract Details

* **Contract ID**: `CB3GB7LGCGMC2KXJK7DX5SLVBTYORPQTSLKWEMU3YUE5G5RGHJ4XWGJC`
* **Language**: Rust
* **Platform**: Soroban Smart Contract (Stellar)
* **Network**: Stellar Testnet

## Future Scope

### Short-Term Enhancements
1.  **Watch Status**: Add "Plan to Watch" or "Completed" toggles for better organization.
2.  **Category Tags**: Allow users to group movies by custom moods or themes.
3.  **Advanced Filtering**: On-chain filtering by genre or minimum rating.

### Medium-Term Development
4.  **Metadata Integration**: Integration with Oracles to automatically fetch posters and synopses.
5.  **Collaborative Lists**: Shared watchlists for families or friend groups.
6.  **Social Sharing**: Public-facing profiles to share your cinematic journey with others.

### Long-Term Vision
7.  **DAO Governance**: Community-driven protocol improvements and feature prioritization.
8.  **Privacy Layers**: Implementation of Zero-Knowledge Proofs for private watchlist content.
9.  **AI Recommendations**: On-chain suggestions based on verified viewing history.

---

## Technical Requirements

* Soroban SDK
* Rust Programming Language
* Stellar CLI

## Getting Started

Interact with the smart contract on Stellar's Soroban network using the four primary functions:

* `add_movie(title, genre, rating)` – Create a new movie entry.
* `get_movies()` – Retrieve the full list from the contract.
* `update_rating(id, new_rating)` – Modify an existing movie's rating.
* `remove_movie(id)` – Permanently delete an entry by its ID.

---

**Stellar Movie Watchlist DApp** – *Securing Your Cinematic Journey on the Blockchain*