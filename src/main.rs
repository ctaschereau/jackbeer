#[macro_use]
extern crate rouille;
extern crate lettre;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::io;
use std::fs::File;

use lettre::smtp::authentication::Credentials;
use lettre::{Transport, SmtpClient};
use lettre_email::EmailBuilder;


fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap();
    println!("Now listening on localhost:{}", port);
    rouille::start_server(format!("localhost:{}", port), move |request| {
        rouille::log(&request, io::stdout(), || {
            router!(request,
                (GET) (/) => {
                    let file = File::open("./src/resource/index.html").unwrap();
                    rouille::Response::from_file("text/html", file)
                },

                (POST) (/submit) => {
                    let data = try_or_400!(post_input!(request, {
                        beer_date: String,
                        gauchitude: String,
                        other_guest: String,
                        comments: String,
                    }));
                    //println!("Received data: {:?}", data);

                    send_email(data.beer_date, data.gauchitude, data.other_guest, data.comments);

                    let file = File::open("./src/resource/end.html").unwrap();
                    rouille::Response::from_file("text/html", file)
                },

                _ => rouille::Response::empty_404()
            )
        })
    });
}

fn send_email(beer_date: String, gauchitude: String, other_guest: String, comments: String) {
    let email_content = format!("Jack voudrait aller prendre une bière le : {}
Sa gauchitude devrait être autour de : {}
Convive bonus : {}
Et il voulait faire part de ce commentaire : {}", beer_date, gauchitude, other_guest, comments);

    let smtp_username = env::var("EMAIL_ADDRESS").unwrap();
    let smtp_password = env::var("EMAIL_PASSWORD").unwrap();

    let email = EmailBuilder::new()
        .to(("cedric.taschereau@gmail.com", "Cédric Taschereau"))
        .from(smtp_username.as_ref())
        .sender("jackbeer_mail@ctaschereau.ninja")
        .subject("Réponse de Jack pour la bière")
        .text(email_content)
        .build()
        .unwrap();


    let creds = Credentials::new(
        smtp_username,
        smtp_password
    );

    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .transport();

    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent!!");
    } else {
        println!("Could not send email: {:?}", result);
    }
}

