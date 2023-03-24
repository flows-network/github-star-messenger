use github_flows::{listen_to_event, EventPayload};
use sendgrid_flows::{send_email, Email};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let login: &str = "jaykchen";
    let owner: &str = "jaykchen";
    let repo: &str = "a-test";

    listen_to_event(login, owner, repo, vec!["star"], |payload| {
        handler(repo, payload)
    })
    .await;

    Ok(())
}

async fn handler(repo: &str, payload: EventPayload) {
    let sender_email_address: &str = "jaykchen@gmail.com";
    let receiver_email_address: &str = "achenics@gmail.com";

    if let EventPayload::UnknownEvent(e) = payload {
        let stargazers_count = e["repository"]["stargazers_count"].as_i64().unwrap_or(-1);

        let text =
            format!("Congratulations on your repository {repo} with {stargazers_count} stars.");

        if stargazers_count % 10 == 0 {
            let email = Email {
                to: vec![String::from(receiver_email_address)],
                subject: String::from("Hi"),
                content: text,
            };
            send_email(sender_email_address, &email).expect("failed to send email");
        }
    }
}
