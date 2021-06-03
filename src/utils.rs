use rraw::me::Me;
use rraw::responses::GenericResponse;
use rraw::responses::subreddit::AboutSubreddit;
use rraw::utils::error::APIError;
use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args,
        buckets::{LimitedFor, RevertBucket},
        CommandGroup,
        CommandOptions, CommandResult, DispatchError, help_commands, HelpOptions, macros::{check, command, group, help, hook}, Reason,
        StandardFramework,
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    prelude::*,
    utils::{content_safe, ContentSafeOptions},
};
use serenity::model::id::ChannelId;

use crate::{Bot, DataHolder};
use num_format::{ToFormattedString, Locale};
use rraw::auth::AnonymousAuthenticator;
use regex::Matches;
use hyper::StatusCode;

pub async fn refresh_server_count(status: &Context) {
    let channel = ChannelId(830636660197687316);
    let i = channel
        .to_channel(&status.http)
        .await
        .unwrap()
        .guild()
        .unwrap()
        .guild_id
        .members(&status.http, None, None)
        .await
        .unwrap()
        .len();
    channel
        .to_channel(&status.http)
        .await
        .unwrap()
        .guild()
        .unwrap()
        .edit(&status.http, |c| c.name(format!("Server Size: {}", i)))
        .await;
}
pub async fn subreddit_info(ctx: Context, matches: Matches<'_, '_>, msg: &Message) {
    for x in matches {
        let text = x.as_str().replace("r/", "");
        let me = Me::login(AnonymousAuthenticator::new(), "Reddit Nobility Bot u/KingTuxWH".to_string()).await.unwrap();
        let subreddit = me.subreddit(text.clone());
        match subreddit.about().await {
            Ok(sub) => {
                let _msg = msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.reference_message(msg);
                        m.embed(|e| {
                            let subreddit1 = sub.data;
                            e.url(format!("https://reddit.com{}", subreddit1.url.unwrap()));
                            e.title(subreddit1.display_name.unwrap());
                            e.field("Members", subreddit1.subscribers.unwrap().to_formatted_string(&Locale::en), true);
                            e.field("Description", subreddit1.public_description.unwrap_or("Missing Description ".to_string()), false);
                            e.footer(|f| {
                                f.text("Robotic Monarch");
                                f
                            });

                            e
                        });
                        m
                    })
                    .await;
            }
            Err(err) => {
                match err {
                    APIError::ExhaustedListing => {}
                    APIError::HTTPError(http) => {
                        if http == StatusCode::FORBIDDEN {
                            let _msg = msg
                                .channel_id
                                .send_message(&ctx.http, |m| {
                                    m.reference_message(msg);
                                    m.embed(|e| {
                                        e.url(format!("https://reddit.com/r/{}", text.clone()));
                                        e.title(text.clone());
                                        e.field("Description", "Hidden Sub", false);
                                        e.footer(|f| {
                                            f.text("Robotic Monarch");
                                            f
                                        });

                                        e
                                    });
                                    m
                                })
                                .await;
                        }
                    }
                    APIError::ReqwestError(_) => {}
                    APIError::JSONError(_) => {}
                    APIError::ExpiredToken => {}
                    APIError::Custom(_) => {}
                }
            }
        };
    }
}

pub async fn user_info(ctx: Context, matches: Matches<'_, '_>, msg: &Message) {
    for x in matches {
        let text = x.as_str().replace("u/", "");
        let me = Me::login(AnonymousAuthenticator::new(), "Reddit Nobility Bot u/KingTuxWH".to_string()).await.unwrap();
        let user = me.user(text.clone());
        match user.about().await {
            Ok(user) => {
                let _msg = msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.reference_message(msg);
                        m.embed(|e| {
                            let user = user.data;
                            e.url(format!("https://reddit.com/u/{}", user.name));
                            e.field("Total Karma", user.total_karma.to_formatted_string(&Locale::en), true);
                            e.field("Comment Karma", user.comment_karma.to_formatted_string(&Locale::en), true);
                            e.field("Link Karma", user.link_karma.to_formatted_string(&Locale::en), true);
                            e.title(user.name);
                            if let Some(img) = user.snoovatar_img {
                                if !img.is_empty() {
                                    e.image(img);
                                } else if let Some(img) = user.icon_img {
                                    e.image(img);
                                }
                            } else {
                                if let Some(img) = user.icon_img {
                                    e.image(img);
                                }
                            }
                            e.footer(|f| {
                                f.text("Robotic Monarch");
                                f
                            });

                            e
                        });
                        m
                    })
                    .await;
            }
            Err(err) => {
                match err {
                    APIError::ExhaustedListing => {}
                    APIError::HTTPError(_) => {}
                    APIError::ReqwestError(_) => {}
                    APIError::JSONError(_) => {}
                    APIError::ExpiredToken => {}
                    APIError::Custom(_) => {}
                }
            }
        };
    }
}
pub async fn refresh_reddit_count(status: Context, me: &Me) {
    let channel = ChannelId(833707456990281818);

    let subreddit = me.subreddit("RedditNobility".to_string());
    let result = subreddit.about().await;
    let count = match result {
        Ok(ok) => {
            ok.data.subscribers.unwrap().to_string()
        }
        Err(er) => {
            match er {
                APIError::ExhaustedListing => {
                    println!("Ex");
                }
                APIError::HTTPError(s) => {
                    println!("Status {}", s);
                }
                APIError::ReqwestError(_) => {
                    println!("Request");

                }
                APIError::JSONError(_) => {
                    println!("JSON");
                }
                APIError::ExpiredToken => {
                    println!("Expired");
                }
                APIError::Custom(s) => {
                    println!("Error: {}", s);

                }
            }
            "Error".to_string()
        }
    };

    channel
        .to_channel(&status.http)
        .await
        .unwrap()
        .guild()
        .unwrap()
        .edit(&status.http, |c| {
            c.name(format!("Reddit Subscribers: {}", count))
        })
        .await;
}
