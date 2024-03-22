use std::{i8, io};

//je définie une constante pour definir notre nombre de menu
const NB_MENU: i8 = 9;

//concordance de choix

//fonction permet de comparet 
fn numero_identique_a_la_premiere_chaine(num: i8, a: &String) -> bool {
    let s = a.chars().nth(0).expect("REASON").to_string();
    match s.parse::<i8>() {
        Ok(n) => {
            return n == num;
        }
        Err(_e) => return false,
    }
}

//permet d'aficher le menu
fn afficher_menu(menu: Vec<String>) {
    let mut taille = menu.len();
    let mut compteur: usize = 0;

    while taille > 0 {
        println!("{}", menu[compteur]);
        compteur = compteur + 1;
        taille = taille - 1;
    }
}

fn choix_utilisateur() -> i8 {
    println!("choix du menu:");
    let mut input_line = String::new();
    let _ = io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let valeur_saisie: i8 = input_line.trim().parse().expect("Input not an integer");

    if valeur_saisie <= 0 || valeur_saisie > NB_MENU {
        println!("choix nom valide!");
        return choix_utilisateur();
    } else {
        return valeur_saisie;
    }
}

//fonction pour afficher un film en fontction de son numero
fn afficher_un_film(numero_du_film: i8, tableau_film: Vec<String>) {
    let place_du_caractere = numero_du_film as usize;
    println!("{}", tableau_film[place_du_caractere]);
    println!("hello");
}
//fonction pour affichetr une liste de film contenu dans un tableau
fn afficher_film(tableau_film: Vec<String>) 
    {
        let mut taille=tableau_film.len();
        while taille > 0
        {

        afficher_un_film(taille.try_into().unwrap(),tableau_film.clone());
        taille=taille-1;
        }
    }

fn ajouter_film(chaine:&String,mut tableau_film: Vec<String>)
    {
            tableau_film.push(chaine.to_string());
    }

fn supprimer_film(positon_film:i8,mut tableau_film: Vec<String>)
{


tableau_film.remove(positon_film.try_into().unwrap());
}

fn main() {
    

    
      
    let mut liste_film:Vec<String>=new();
    
    
    ajouter_film(& String::from("Martini"),liste_film.clone());
    afficher_film(liste_film);
    //let _a: i8 = choix_utilisateur();
    println!("{}",liste_film);
}
