

// incoming web hook
//  https://api.slack.com/incoming-webhooks
// rust webhook:
//  https://github.com/frostly/rust-slack

extern crate slack_hook;

// realtime messaging
// bot user integration:  https://api.slack.com/bot-users
// Real Time Messaging API: https://api.slack.com/rtm
// rust: https://github.com/slack-rs/slack-rs
extern crate slack;

use slack_hook::{Slack, Payload, PayloadTemplate};

use std::error::Error;


struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {

    // event:
    //   Message(Standard {
    //               ts: "1459604462.000173",
    //               channel: Some("C04D3USMM"),
    //               user: Some("U039CNAPF"),
    //               text: Some("@botuser boom"),
    //               is_starred: None,
    //               pinned_to: None,
    //               reactions: None,
    //               edited: None,
    //               attachments: None
    //           }
    //       ),
    fn on_event(&mut self, cli: &mut slack::RtmClient, event: Result<&slack::Event, slack::Error>, raw_json: &str) {
        match event {
            Ok(message) => {

                // UserTyping
                // Message(Message)
                // ReconnectUrl
                // Standard
                // ...

                match message {

                    // Message
                    &slack::Event::Message(ref ev) => {

                        match ev {

                            // Message(Standard)
                            &slack::Message::Standard {user: Some(ref user_id), text: Some(ref text), channel: Some(ref channel), ..} => {

                                //
                                let botid = cli.get_id().unwrap_or("".to_string());
                                if *user_id == botid {
                                    println!("message is post from myself.");
                                    return
                                }

                                let incomming = format!("userid: {}, message: {:?},)", &user_id, &text);
                                cli.send_message("#dev", &incomming);

                            },
                            _=>{
                                return;
                            }
                        }

                    },
                    _ => {
                        return ;
                    }
                }

            },
            Err(e) => {

                // TODO: why this error is prompted so often?
                //   invalid message failure: decoder error
                println!("invalid message failure: {}", e.description());
            }
        }

    }

    fn on_ping(&mut self, cli: &mut slack::RtmClient) {
        println!("on_ping");
    }

    fn on_close(&mut self, cli: &mut slack::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &mut slack::RtmClient) {

        println!("on_connect");

        // Do a few things using the api:
        // send a message over the real time api websocket
        let _ = cli.send_message("#dev", "wake up! (rtm)");

        // post a message as a user to the web api
        let _ = cli.post_message("#dev", "wake up! (postMessage)", None);
    }
}


/*
export SLACK_BOT_KEY=
export SLACK_INCOMMING_WEBHOOK_URL=
*/

fn try_realtime_message() {

    let slack_bot_key = ::std::env::var("SLACK_BOT_KEY").unwrap().to_string();

    let mut handler = MyHandler;
    let mut cli = slack::RtmClient::new(&slack_bot_key);
    let (client, rx) = cli.login().unwrap();

    println!("name: {}", cli.get_name().unwrap());
    println!("team: {}", cli.get_team().unwrap().name);

    let r = cli.run::<MyHandler>(&mut handler, client, rx);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}


fn try_message_webhook() {
    let webhookurl = ::std::env::var("SLACK_INCOMMING_WEBHOOK_URL").unwrap();
    let inwebhook = Slack::new(&webhookurl);
    let p = Payload::new(PayloadTemplate::Complete {
        text: Some("money mouth face"),
        channel: Some("#dev"),
        username: Some("familybot2"),
        icon_url: None,
        icon_emoji: Some(":money_mouth_face:"),
        attachments: None,
        unfurl_links: Some(true),
        link_names: Some(false)
    });

    let res = inwebhook.send(&p);
    match res {
        Ok(()) => println!("ok"),
        Err(x) => println!("ERR: {:?}",x)
    }
}


fn main() {
    try_message_webhook();
    try_realtime_message();
}



