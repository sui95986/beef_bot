{"metadata":{"message_id":"jfYtFRGrher-4dnd-WeRXeq6vDhGkrWYDF3JkT_iX0k=","message_type":"notification","message_timestamp":"2025-03-31T19:19:53.06543104Z","subscription_type":"channel.ad_break.begin","subscrip tion_version":"1"},"payload":{"subscription":{"id":"aa89e14c-6239-417d-8c98-217555c971ec","status":"enabled","type":"channel.ad_break.begin","version":"1","condition":{"broadcaster_user_id":"87773461"},"transp ort":{"method":"websocket","session_id":"AgoQ_sbgRImJTQO8benYbkWy5BIGY2VsbC1i"},"created_at":"2025-03-31T18:18:15.221528455Z","cost":0},"event":{"broadcaster_user_id":"87773461","broadcaster_user_login":"suici debeef","broadcaster_user_name":"suicidebeef","requester_user_id":"87773461","requester_user_login":"suicidebeef","requester_user_name":"suicidebeef","duration_seconds":90,"is_automatic":true,"started_at":"202 5-03-31T19:19:53.06543104Z"}}}



## Todos
* Send a chat message as the bot
* Look up the other messages that must be handled

IMPORTANT NOTE: WE USE MACROS FOR THESE ENV VARS SO THEY ARE BUILT AT COMPILE TIME
IF YOU WANT THESE TO REFRESH YOU MUST MAKE A CODE CHANGE SO THE BINARY UPDATES FRESH

CLIENT_ID
CLIENT_SECRET
BOT_USER_ID
BROADCASTER_ID
OAUTH_TOKEN

POST https://api.twitch.tv/helix/eventsub/subscriptions
-H 'Authorization: Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx' \
-H 'Client-Id: wbmytr93xzw8zbg0p1izqyzzc5mbiz' \
-H 'Content-Type: application/json' \

{
    "type": "channel.chat.message",
    "version": "1",
    "condition": {
        "broadcaster_user_id": "87773461",
        "user_id": "87773461"
    },
    "transport": {
        "method": "websocket",
        "session_id": "some_session_id"
    }
}

If you use WebSockets to receive events, the request must specify a user access token

Need: User access token;
Not need: app access token;
HAVE: client id and client secret;

does subscription type require user authentication?
yes -- with scopes user:read:chat and channel:bot

https://id.twitch.tv/oauth2/authorize?client_id=htzvetxwhqrl84oafo8rzi25iyq3io&redirect_uri=https%3A%2F%2Flocalhost:8080&response_type=token&scope=user:read:chat%20user:write:chat%20channel:bot%20user:bot%20channel:read:ads

-----------api

What does a message look like?

{
    "metadata": {
        "message_id": "rPG-d8pxdPv4BJpNS4ltzpUpK8Akwu5Qto1r8biLCPY=",
        "message_timestamp": "2025-03-21T08:33:26.90092605Z",
        "message_type": "notification",
        "subscription_type": "channel.chat.message",
        "subscription_version": "1"
    },
    "payload": {
        "event": {
            "badges": [
                {
                    "id": "1",
                    "info": "",
                    "set_id": "bits"
                }
            ],
            "broadcaster_user_id": "87773461",
            "broadcaster_user_login": "suicidebeef",
            "broadcaster_user_name": "suicidebeef",
            "channel_points_animation_id": null,
            "channel_points_custom_reward_id": null,
            "chatter_user_id": "732980080",
            "chatter_user_login": "jellymie",
            "chatter_user_name": "jellymie",
            "cheer": null,
            "color": "",
            "message": {
                "fragments": [
                    {
                        "cheermote": null,
                        "emote": null,
                        "mention": null,
                        "text": "Hi",
                        "type": "text"
                    }
                ],
                "text": "Hi"
            },
            "message_id": "8579a1b3-fc8a-4bb8-b95f-1e3f6ae19663",
            "message_type": "text",
            "reply": null,
            "source_badges": null,
            "source_broadcaster_user_id": null,
            "source_broadcaster_user_login": null,
            "source_broadcaster_user_name": null,
            "source_message_id": null
        },
        "subscription": {
            "condition": {
                "broadcaster_user_id": "87773461",
                "user_id": "87773461"
            },
            "cost": 0,
            "created_at": "2025-03-21T08:33:17.519692311Z",
            "id": "6cd4b70d-755d-4b3a-871f-31fca4061972",
            "status": "enabled",
            "transport": {
                "method": "websocket",
                "session_id": "AgoQPTSEO7AkQ2qvQK9NgaNwlxIGY2VsbC1h"
            },
            "type": "channel.chat.message",
            "version": "1"
        }
    }
}
