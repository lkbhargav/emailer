use anyhow::Result;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub enum Relay {
    Gmail,
}

pub struct Email {
    from: String,
    reply_to: String,
    mailer: SmtpTransport,
}

impl Email {
    pub fn new(
        from: String,
        reply_to: String,
        username: &str,
        app_password: &str,
        relay: Relay,
    ) -> Result<Email> {
        let creds = Credentials::new(username.to_string(), app_password.to_string());

        let relay = match relay {
            Relay::Gmail => "smtp.gmail.com",
        };

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(relay)?.credentials(creds).build();

        Ok(Email {
            from,
            reply_to,
            mailer,
        })
    }

    pub fn send(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        let header = ContentType::TEXT_PLAIN;

        let email = Message::builder()
            .from(self.from.parse()?)
            .reply_to(self.reply_to.parse()?)
            .header(header)
            .to(to.parse()?)
            .subject(subject)
            .body(String::from(body))?;

        let _ = &self.mailer.send(&email)?;

        Ok(())
    }
}
