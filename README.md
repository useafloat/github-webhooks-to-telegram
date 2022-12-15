# GitHub-webhooks

An AWS Lambda function receiving, parsing and sending [Github webhooks](https://docs.github.com/en/developers/webhooks-and-events/webhooks/about-webhooks) to [Telegram](https://telegram.org/).

We use Telegram as our main communication in [Afloat](https://useafloat.com/). Part of this is logging both user and organizational activity in our channels.  
This is a simple AWS Lambda sending organizational events in Telegram through GitHub Webhooks.

## Development
We use [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda) for development and deployment.

### Env variables
The following environment variables are required:
```
TELEGRAM_BOT
TELEGRAM_CHAT_ID
```

Where `TELEGRAM_BOT` is the API token for your telegram bot.
