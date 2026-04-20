# Stellar Movie Watchlist DApp

**Stellar Movie Watchlist DApp** - Blockchain-Based Decentralized Movie Tracking System

## Project Description

Stellar Movie Watchlist DApp adalah solusi smart contract terdesentralisasi yang dibangun di atas blockchain Stellar menggunakan Soroban SDK. Proyek ini menyediakan platform yang aman, transparan, dan *immutable* (tidak dapat diubah secara sepihak) untuk mengelola daftar tontonan film pribadi langsung di blockchain. 

Kontrak ini memastikan bahwa data koleksi film Anda disimpan dengan aman dan hanya dapat dikelola melalui fungsi smart contract yang telah ditentukan, menghilangkan ketergantungan pada penyedia basis data terpusat. Dengan sistem ini, riwayat tontonan dan rating film Anda menjadi aset digital yang sepenuhnya berada di bawah kendali Anda.

## Project Vision

Visi kami adalah merevolusi cara pecinta film mengelola data hobi mereka di era digital dengan:

- **Decentralizing Entertainment Data**: Memindahkan manajemen watchlist dari server terpusat ke jaringan blockchain global yang terdistribusi.
- **Ensuring Ownership**: Memberdayakan pengguna untuk memiliki kontrol penuh atas preferensi dan riwayat tontonan mereka.
- **Guaranteeing Immutability**: Menyediakan catatan permanen dan tahan manipulasi untuk setiap rating dan ulasan film.
- **Enhancing Privacy**: Memanfaatkan keamanan blockchain untuk melindungi informasi personal dari akses yang tidak sah.
- **Building Trustless Systems**: Menciptakan platform di mana integritas data dijamin oleh kode, bukan oleh kebijakan perusahaan.

## Key Features

### 1. **Simple Movie Management**
- Menambahkan film baru dengan detail Judul, Genre, dan Rating.
- Pembuatan ID unik secara otomatis untuk setiap entri film.
- Penyimpanan persisten yang aman di jaringan Stellar.

### 2. **Efficient Data Retrieval**
- Mengambil seluruh daftar film (watchlist) dalam satu panggilan fungsi.
- Representasi data terstruktur untuk integrasi frontend yang mudah.
- Sinkronisasi real-time dengan status blockchain terbaru.

### 3. **Dynamic Rating Updates**
- Memperbarui rating film yang sudah ada setelah Anda selesai menonton.
- Memungkinkan perubahan data secara transparan di dalam blok.

### 4. **Secure Deletion**
- Menghapus film tertentu dari daftar menggunakan ID unik mereka.
- Penghapusan permanen dari storage kontrak untuk menjaga efisiensi data.
- Manajemen storage yang bersih untuk mengoptimalkan biaya transaksi (fee).

### 5. **Stellar Network Integration**
- Memanfaatkan kecepatan tinggi dan biaya sangat rendah dari jaringan Stellar.
- Dibangun menggunakan Soroban Smart Contract SDK yang modern.
- Arsitektur yang skalabel untuk koleksi film yang terus bertambah.

## Contract Details

- **Contract Address**: `CB3GB7LGCGMC2KXJK7DX5SLVBTYORPQTSLKWEMU3YUE5G5RGHJ4XWGJC`
- **Language**: Rust
- **Platform**: Soroban Smart Contract (Stellar)

## Future Scope

### Short-Term Enhancements
1. **Watch Status**: Menambahkan status "Sudah Ditonton" atau "Rencana Tonton".
2. **Category Management**: Menambahkan tag untuk mengelompokkan film berdasarkan suasana hati.
3. **Advanced Search**: Implementasi filter pencarian berdasarkan genre atau rating minimum.

### Medium-Term Development
4. **Collaborative Watchlist**: Fitur daftar tontonan bersama untuk keluarga atau komunitas.
5. **Notification System**: Pengingat otomatis untuk jadwal rilis film melalui bridge off-chain.
6. **Metadata Integration**: Integrasi dengan Oracle untuk mengambil poster film dan sinopsis secara otomatis.

### Long-Term Vision
7. **DAO Governance**: Tata kelola komunitas untuk fitur-fitur baru protokol.
8. **Privacy Layers**: Implementasi Zero-Knowledge Proofs untuk menyembunyikan konten watchlist secara privat.
9. **AI-Powered Recommendations**: Integrasi AI untuk menyarankan film berdasarkan riwayat tontonan di blockchain.

---

## Technical Requirements

- Soroban SDK
- Rust Programming Language
- Stellar Blockchain Network

## Getting Started

Deploy smart contract ini ke jaringan Soroban Stellar dan berinteraksi menggunakan empat fungsi utama:

- `add_movie()` - Menambahkan film baru (Judul, Genre, Rating).
- `get_movies()` - Mengambil semua daftar film dari kontrak.
- `update_rating()` - Memperbarui skor rating film berdasarkan ID.
- `remove_movie()` - Menghapus film dari daftar berdasarkan ID.

---

**Stellar Movie Watchlist DApp** - *Securing Your Cinematic Journey on the Blockchain*

ID Smart contract: CB3GB7LGCGMC2KXJK7DX5SLVBTYORPQTSLKWEMU3YUE5G5RGHJ4XWGJC