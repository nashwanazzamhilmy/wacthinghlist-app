#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// 1. Struktur data untuk menyimpan informasi film
#[contracttype]
#[derive(Clone, Debug)]
pub struct Movie {
    pub id: u64,
    pub title: String,
    pub genre: String,
    pub rating: u32, // Skala 1-100 atau 1-5
}

// 2. Storage key unik untuk data watchlist
const MOVIE_DATA: Symbol = symbol_short!("MOV_DATA");

#[contract]
pub struct MovieWatchlistContract;

#[contractimpl]
impl MovieWatchlistContract {

    // Fungsi untuk mengambil semua film di watchlist
    pub fn get_movies(env: Env) -> Vec<Movie> {
        env.storage().instance().get(&MOVIE_DATA).unwrap_or(Vec::new(&env))
    }

    // Fungsi untuk menambahkan film baru ke watchlist
    pub fn add_movie(env: Env, title: String, genre: String, rating: u32) -> String {
        let mut movies: Vec<Movie> = env.storage().instance().get(&MOVIE_DATA).unwrap_or(Vec::new(&env));
        
        let movie = Movie {
            id: env.prng().gen::<u64>(), // Generate ID unik secara acak
            title,
            genre,
            rating,
        };
        
        movies.push_back(movie);
        
        env.storage().instance().set(&MOVIE_DATA, &movies);
        
        String::from_str(&env, "Film berhasil ditambahkan ke watchlist!")
    }

    // Fungsi untuk menghapus film berdasarkan ID
    pub fn remove_movie(env: Env, id: u64) -> String {
        let mut movies: Vec<Movie> = env.storage().instance().get(&MOVIE_DATA).unwrap_or(Vec::new(&env));

        for i in 0..movies.len() {
            if movies.get(i).unwrap().id == id {
                movies.remove(i);
                env.storage().instance().set(&MOVIE_DATA, &movies);
                return String::from_str(&env, "Film berhasil dihapus dari watchlist");
            }
        }

        String::from_str(&env, "Film tidak ditemukan")
    }
    
    // Fungsi tambahan: Update rating film
    pub fn update_rating(env: Env, id: u64, new_rating: u32) -> String {
        let mut movies: Vec<Movie> = env.storage().instance().get(&MOVIE_DATA).unwrap_or(Vec::new(&env));
        
        for i in 0..movies.len() {
            let mut movie = movies.get(i).unwrap();
            if movie.id == id {
                movie.rating = new_rating;
                movies.set(i, movie);
                env.storage().instance().set(&MOVIE_DATA, &movies);
                return String::from_str(&env, "Rating film berhasil diperbarui");
            }
        }
        
        String::from_str(&env, "Film tidak ditemukan")
    }
}

mod test;