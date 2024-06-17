// on importe la librairie actix_web
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_files as afs;
use std::fs;

async fn home() -> impl Responder {
    let html_content = fs::read_to_string("web/home.html").unwrap_or_else(|_| {
        String::from("<h1>Impossible de charger la page!</h1>")
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}


async fn dogs() -> impl Responder {
    let html_content = fs::read_to_string("web/dogs.html").unwrap_or_else(|_| {
        String::from("<h1>Impossible de charger la page!</h1>")
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}

async fn details() -> impl Responder {
    let html_content = fs::read_to_string("web/details.html").unwrap_or_else(|_| {
        String::from("<h1>Impossible de charger la page!</h1>")
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}

// On affiche la page 404 pour toutes les autres routes
async fn error() -> impl Responder {
    // Charger le contenu du fichier HTML
    let html_content = fs::read_to_string("404.html").unwrap_or_else(|_| {
        // Si la lecture du fichier échoue, fournir un contenu de remplacement
        String::from("<h1>404 Not Found !</h1>")
    });
    // Renvoyer le contenu HTML en tant que réponse
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}

// on crée une fonction main qui lance le serveur web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // on fait le lien entre la route / et la fonction index
        App::new()
            .service(web::resource("/").to(home))
            .service(web::resource("/dogs").to(dogs))
            .service(web::resource("/details").to(details))
            .service(afs::Files::new("/public", "./public").show_files_listing()) // on sert les fichiers statiques. Il faut aller modifier les liens dans le fichier html pour y ajouter le /public
            .default_service(web::route().to(error))
        })
    .bind("127.0.0.1:8081")? // on lance le serveur web sur le port 8080
    .run()
    .await
}