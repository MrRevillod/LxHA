use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lxha_lib::app::constants::{LXHA_MAIL_ADRESS, LXHA_MAIL_PASSWORD, LXHA_MAIL_HOST};
use axum_responses::{AxumResponse, HttpResponse};

// Function to validate email addresses using a regular expression
pub fn mail_validator(mail: &str) -> bool {
    let regex = regex::Regex::new(r"^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$").unwrap();
    regex.is_match(mail)
}
// Function to send an email with the given template, subject, and the destination user email address
pub fn sender(template: String, subject: &str, user_email: &str) -> AxumResponse {
    // Validate the user email address
    if !mail_validator(user_email) {
        return Err(HttpResponse::CUSTOM(400, "Invalid email"))
    }

    let email_from = format!("LXHA <{}>", *LXHA_MAIL_ADRESS);

    // Create the email message
    let email = Message::builder()
        .from(email_from.parse().unwrap())
        .to(user_email.parse().unwrap())
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .body(template)
        .unwrap();
    
    // Create credentials for the SMTP server
    let creds = Credentials::new(LXHA_MAIL_ADRESS.to_owned(),LXHA_MAIL_PASSWORD.to_owned());
    
    // Build the SMTP transporter
    let transporter = SmtpTransport::relay(&LXHA_MAIL_HOST).unwrap().credentials(creds).build();

    // Send the email and handle the result
    match transporter.send(&email) {
        Ok(_) => return Ok(HttpResponse::CUSTOM(200, "Email sent successfully!")),
        Err(e) => {
            dbg!(&e);
            return Err(HttpResponse::CUSTOM(400, "Email was not sent"))},
    }
}
