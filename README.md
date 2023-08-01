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

# Run the bot
1. `docker pull angelsflyinhell/raeys:latest`
2. `docker run -d -e DISCORD_TOKEN=<token> --name raeys angelsflyinhell/raeys:latest`