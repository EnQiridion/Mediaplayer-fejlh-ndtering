use std::collections::HashMap;
use std::io::{self, Write};

// ── Fejltyper ────────────────────────────────────────────────────────────────

#[derive(Debug)]
enum MusicError {
    PlaylistAlreadyExists(String),
    PlaylistNotFound(String),
    SongAlreadyInPlaylist(String),
    SongNotFound(String),
    EmptyPlaylist(String),
    Offline,
    InvalidUser,
}

impl std::fmt::Display for MusicError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MusicError::PlaylistAlreadyExists(n) => write!(f, "Playlist '{}' eksisterer allerede.", n),
            MusicError::PlaylistNotFound(n)      => write!(f, "Playlist '{}' blev ikke fundet.", n),
            MusicError::SongAlreadyInPlaylist(s) => write!(f, "Sangen '{}' er allerede på listen.", s),
            MusicError::SongNotFound(s)          => write!(f, "Sangen '{}' findes ikke.", s),
            MusicError::EmptyPlaylist(n)         => write!(f, "Playlist '{}' er tom.", n),
            MusicError::Offline                  => write!(f, "Ingen internetforbindelse – prøv igen."),
            MusicError::InvalidUser              => write!(f, "Ugyldigt brugernavn."),
        }
    }
}

type Playlists = HashMap<String, Vec<String>>;

// ── Backendlogik ─────────────────────────────────────────────────────────────

fn create_playlist(playlists: &mut Playlists, name: &str) -> Result<(), MusicError> {
    if playlists.contains_key(name) {
        return Err(MusicError::PlaylistAlreadyExists(name.to_string()));
    }
    playlists.insert(name.to_string(), vec![]);
    Ok(())
}

fn add_song(playlists: &mut Playlists, playlist: &str, song: &str) -> Result<(), MusicError> {
    let songs = playlists
        .get_mut(playlist)
        .ok_or_else(|| MusicError::PlaylistNotFound(playlist.to_string()))?;

    if songs.contains(&song.to_string()) {
        return Err(MusicError::SongAlreadyInPlaylist(song.to_string()));
    }
    songs.push(song.to_string());
    Ok(())
}

fn play_song(playlists: &Playlists, playlist: &str, song: &str, online: bool) -> Result<String, MusicError> {
    let songs = playlists
        .get(playlist)
        .ok_or_else(|| MusicError::PlaylistNotFound(playlist.to_string()))?;

    if songs.is_empty() {
        return Err(MusicError::EmptyPlaylist(playlist.to_string()));
    }

    songs
        .iter()
        .find(|s| s.as_str() == song)
        .ok_or_else(|| MusicError::SongNotFound(song.to_string()))?;

    if !online {
        return Err(MusicError::Offline);
    }

    Ok(format!("♪  Afspiller nu: '{}'  ♪", song))
}

// ── TUI-hjælpere ─────────────────────────────────────────────────────────────

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

fn print_header() {
    println!("╔══════════════════════════════════════╗");
    println!("║        🎵  Musik Manager TUI  🎵      ║");
    println!("╚══════════════════════════════════════╝");
    println!();
}

fn print_menu() {
    println!("┌──────────────────────────────────────┐");
    println!("│  [1]  Opret afspilningsliste          │");
    println!("│  [2]  Tilføj sang til liste           │");
    println!("│  [3]  Afspil sang                     │");
    println!("│  [4]  Vis alle lister og sange        │");
    println!("│  [0]  Afslut                          │");
    println!("└──────────────────────────────────────┘");
    print!("  Vælg: ");
    io::stdout().flush().unwrap();
}

fn prompt(label: &str) -> String {
    print!("  {} ", label);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Kunne ikke læse input");
    input.trim().to_string()
}

fn print_ok(msg: &str) {
    println!("\n  ✅  {}", msg);
}

fn print_err(err: &MusicError) {
    println!("\n  ❌  {}", err);
}

fn print_playlists(playlists: &Playlists) {
    println!();
    if playlists.is_empty() {
        println!("  (ingen afspilningslister endnu)");
        return;
    }

    for (name, songs) in playlists {
        println!("  📁  {}", name);
        if songs.is_empty() {
            println!("       (ingen sange)");
        } else {
            for (i, song) in songs.iter().enumerate() {
                println!("       {}. {}", i + 1, song);
            }
        }
    }
}

fn pause() {
    println!();
    prompt("Tryk Enter for at fortsætte...");
}

// ── Menuhandlere ──────────────────────────────────────────────────────────────

fn handle_create(playlists: &mut Playlists) {
    clear_screen();
    print_header();
    println!("  ── Opret afspilningsliste ──\n");

    let name = prompt("Navn på liste:");
    if name.is_empty() {
        println!("\n  ⚠️   Navn må ikke være tomt.");
    } else {
        match create_playlist(playlists, &name) {
            Ok(_)  => print_ok(&format!("Playlist '{}' oprettet!", name)),
            Err(e) => print_err(&e),
        }
    }
    pause();
}

fn handle_add_song(playlists: &mut Playlists) {
    clear_screen();
    print_header();
    println!("  ── Tilføj sang ──\n");

    print_playlists(playlists);
    println!();

    let playlist = prompt("Navn på afspilningsliste:");
    let song     = prompt("Sangnavn:");

    if playlist.is_empty() || song.is_empty() {
        println!("\n  ⚠️   Ingen felter må være tomme.");
    } else {
        match add_song(playlists, &playlist, &song) {
            Ok(_)  => print_ok(&format!("'{}' tilføjet til '{}'!", song, playlist)),
            Err(e) => print_err(&e),
        }
    }
    pause();
}

fn handle_play(playlists: &Playlists) {
    clear_screen();
    print_header();
    println!("  ── Afspil sang ──\n");

    print_playlists(playlists);
    println!();

    let playlist = prompt("Navn på afspilningsliste:");
    let song     = prompt("Sangnavn:");
    let online_s = prompt("Er du online? (j/n):");
    let online   = online_s.to_lowercase() == "j";

    if playlist.is_empty() || song.is_empty() {
        println!("\n  ⚠️   Ingen felter må være tomme.");
    } else {
        match play_song(playlists, &playlist, &song, online) {
            Ok(msg) => print_ok(&msg),
            Err(e)  => {
                print_err(&e);
                // Giver brugeren mulighed for at prøve igen ved offline-fejl
                if let MusicError::Offline = e {
                    let retry = prompt("Prøv igen? (j/n):");
                    if retry.to_lowercase() == "j" {
                        match play_song(playlists, &playlist, &song, true) {
                            Ok(msg) => print_ok(&msg),
                            Err(e2) => print_err(&e2),
                        }
                    }
                }
            }
        }
    }
    pause();
}

fn handle_list(playlists: &Playlists) {
    clear_screen();
    print_header();
    println!("  ── Alle afspilningslister ──");
    print_playlists(playlists);
    pause();
}

// ── Main loop ────────────────────────────────────────────────────────────────

fn main() {
    let mut playlists: Playlists = HashMap::new();

    loop {
        clear_screen();
        print_header();
        print_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Kunne ikke læse input");

        match choice.trim() {
            "1" => handle_create(&mut playlists),
            "2" => handle_add_song(&mut playlists),
            "3" => handle_play(&playlists),
            "4" => handle_list(&playlists),
            "0" => {
                clear_screen();
                println!("  Farvel! 👋");
                break;
            }
            _  => {
                println!("\n  ⚠️   Ugyldigt valg.");
                pause();
            }
        }
    }
}