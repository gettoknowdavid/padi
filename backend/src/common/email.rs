pub struct EmailService {
    client: reqwest::Client,
    api_key: String,
    from: String,
}

impl EmailService {
    pub fn new(client: reqwest::Client, api_key: String) -> Self {
        Self {
            client,
            api_key,
            from: "Padi <noreply@yourpadiapp.com>".to_string(),
        }
    }

    pub async fn send(&self, to: &str, subject: &str, html: &str) {
        let body = serde_json::json!({
            "from": &self.from,
            "to": [to],
            "subject": subject,
            "html": html,
        });

        if let Err(e) = self
            .client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
        {
            tracing::error!("Failed to send email to {}: {:?}", to, e);
        }
    }
}
