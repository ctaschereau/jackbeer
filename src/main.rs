#[macro_use]
extern crate rouille;
extern crate lettre;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::io;
use std::fs::File;

use lettre::smtp::authentication::Credentials;
use lettre::{SendableEmail, EmailAddress, Transport, Envelope, SmtpClient};

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap();
    println!("Now listening on localhost:{}", port);
    rouille::start_server(format!("localhost:{}", port), move |request| {
        rouille::log(&request, io::stdout(), || {
            router!(request,
                (GET) (/) => {
                    let file = File::open("./src/resource/index.html").unwrap();
                    let response = rouille::Response::from_file("text/html", file);
                    response
                },

                (POST) (/submit) => {
                    let data = try_or_400!(post_input!(request, {
                        beer_date: String,
                        gauchitude: String,
                        other_guest: String,
                        comments: String,
                    }));

                    println!("Received data: {:?}", data);

                    send_email(data.beer_date, data.gauchitude, data.other_guest, data.comments);

                    rouille::Response::html("Success! <a href=\"/\">Go back</a>.")
                },

                _ => rouille::Response::empty_404()
            )
        })
    });
}

fn send_email(beer_date: String, gauchitude: String, other_guest: String, comments: String) {
    let mut email_content = "".to_string();
    email_content.push_str("Jack voudrait aller prendre une bière le : ");
    email_content.push_str(beer_date.as_ref());
    email_content.push_str("\nSa gauchitude devrait être autour de : ");
    email_content.push_str(gauchitude.as_ref());
    email_content.push_str("\nConvive bonus : ");
    email_content.push_str(other_guest.as_ref());
    email_content.push_str("\nEt il voulait faire part de ce commentaire : ");
    email_content.push_str(comments.as_ref());

    let email = SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new("cegepcedtest@gmail.com".to_string()).unwrap()),
            vec![EmailAddress::new("cedric.taschereau@gmail.com".to_string()).unwrap()],
        ).unwrap(),
        "Réponse de Jack pour la bière".to_string(),
        email_content.into_bytes(),
    );

    let creds = Credentials::new(
        env::var("EMAIL_ADDRESS").unwrap(),
        env::var("EMAIL_PASSWORD").unwrap()
    );


    // Open a remote connection to gmail
    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        //.smtp_utf8(true)
        .transport();

    // Send the email
    let result = mailer.send(email);

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }
}

