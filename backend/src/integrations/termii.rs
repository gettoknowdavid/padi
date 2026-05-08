use anyhow::Result;

pub async fn send_otp(
    client: &reqwest::Client,
    api_key: &str,
    phone: &str,
    otp: &str,
) -> Result<()> {
    let body = serde_json::json!({
        "api_key": api_key,
        "message_type": "NUMERIC",
        "to": phone,
        "from": "Padi",
        "channel": "dnd",
        "pin_attempts": 3,
        "pin_time_to_live": 10,
        "pin_length": 6,
        "pin_placeholder": "< 1234 >",
        "message_text": "Your Padi verification code is: < 1234 >. Valid for 10 minutes. Do not share.",
        "pin_type": "NUMERIC"
    });

    client.post("https://api.ng.termii.com/api/sms/otp/send")
        .json(&body)
        .send()
        .await?;

    Ok(())
}
