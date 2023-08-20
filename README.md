[![Check Build](https://github.com/xp-bot/raeys/actions/workflows/check.yml/badge.svg)](https://github.com/xp-bot/raeys/actions/workflows/check.yml)
[![Docker Image](https://github.com/xp-bot/raeys/actions/workflows/deploy.yml/badge.svg)](https://github.com/xp-bot/raeys/actions/workflows/deploy.yml)

# XP v8
The official XP v8 rewrite built with Rust and serenity-rs.

# Contributions
We're always looking for contributors! If you want to contribute, please read the [contribution guide](CONTRIBUTING.md) first.

# Run the bot
1. Create a `.env` file with the following content:
```env
# Bot
DISCORD_TOKEN= <YOUR BOT TOKEN HERE>
WEBSITE=https://xp-bot.net

# API
API_AUTH=
API_URL=http://namespace.media:3000

# Top.gg
TOPGG_TOKEN=  <YOUR TOP.GG TOKEN HERE>

# Config
RUST_LOG=error,xp_bot=info

# Colors
BLUE=0x5a62ed
RED=0xe54344
GREEN=0x7DC95E
GRAY=0x37474f
```

2. `docker pull ghcr.io/xp-bot/raeys-v8:latest`
3. `docker run -d --env .env --name raeys ghcr.io/xp-bot/raeys-v8:latest`