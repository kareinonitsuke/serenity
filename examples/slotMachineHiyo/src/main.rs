use std::env;
use rand::Rng;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let momo = ":baby_chick:";
        let momogang = ":hatching_chick:";
        let picture_num:i16 = 2;
        let slot_num:i16 = 3;

       if msg.content == "!ひよこスロット" {
            println!("Shard {}", ctx.shard_id);
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);
                    return;
                },
            };

            let mut picture = ["a","b","c"];
            for _n in 1..slot_num+1{
                let rand_num :i16 = rand::thread_rng().gen_range(0, picture_num);
                let i:i16 = _n;
                match i{//配列の指定のところ1だとうまくいかないので暫定　そもそもどう書くべきだろうか
                    1=> match rand_num{
                        0=> picture[0] = momo,
                        1=> picture[0] = momogang,
                        _=> picture[0] = "error"
                        },
                    2=> match rand_num{
                        0=> picture[1] = momo,
                        1=> picture[1] = momogang,
                        _=> picture[1] = "error"
                        },
                    3=> match rand_num{
                        0=> picture[2] = momo,
                        1=> picture[2] = momogang,
                        _=> picture[2] = "error"
                        },
                    _ => println!("Error"),
                }
                if _n == 3{//forの外で結果とるのどうしたらいいかわからんかったので暫定
                    let response = MessageBuilder::new()
                    //.mention(&msg.author)
                    //.push(" ")
                    .push(picture[0].to_string()+" "+picture[1]+" "+picture[2])
                    .build();
                    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    // The total number of shards to use. The "current shard number" of a
    // shard - that is, the shard it is assigned to - is indexed at 0,
    // while the total shard count is indexed at 1.
    //
    // This means if you have 5 shards, your total shard count will be 5, while
    // each shard will be assigned numbers 0 through 4.
    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {:?}", why);
    }
}
