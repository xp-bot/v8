# raeys - XP v8
`raeys` is the project name of the version 8 rewrite of the XP Discord Bot.
Our main priorities for this rewrite are stability, performance and sustainability, since we really don't want to do this another, 9th time.

# Conventions
You should (obviously) follow Rust's language conventions, but please also mind the following:
- Commit names should be in this structure: 
```
<type>(<scope>): <subject>

f.e.:
feat(commands): add ping command
fix(commands): fix ping command
```
> Detailed examples, branch guide and more: [Convention Guide](https://dev.to/varbsan/a-simplified-convention-for-naming-branches-and-commits-in-git-il4)
- Always create a new branch for your feature. When you're done, create a pull request to merge your branch into `dev`. `dev` will be merged into `master` when we're ready to release a new version.

# Run the bot
0. Create a `.env` file with the following content:
```env
# Bot
DISCORD_TOKEN=
WEBSITE=https://xp-bot.net

# API
API_AUTH=
API_URL=http://namespace.media:3000

# Config
RUST_LOG=raeys

# Colors
BLUE=0x5a62ed
RED=0xe54344
GREEN=0x7DC95E
GRAY=0x37474f
```

1. `docker pull angelsflyinhell/raeys:latest`
2. `docker run -d -e DISCORD_TOKEN=<token> --name raeys angelsflyinhell/raeys:latest`
