# Contributing to v8
If you're looking to contribute, you can [fork](https://github.com/xp-bot/v8/fork) this repository and create a pull request to merge your changes into `dev`. `dev` will be merged into `master` when we're ready to release a new version.

Instructions on how to run the bot locally can be found in the [README](https://github.com/xp-bot/v8) of the main repository.\
We also ask you to follow our convention guide below, so that all commits and branches are consistent and can be traced back to their origin, if necessary.
> Also make sure, that each pull request has a detailed description of what you changed and why you changed it. Pull requests should always be as small as possible and only contain one feature or bugfix.

# Conventions
First of all, you should always follow Rust's language conventions for variables, functions, etc., but please also mind the following:
- Commit names should be in this structure: 
```
<type>(<scope>): <subject>

f.e.:
feat(commands): add ping command
fix(commands): fix ping command
```
> Detailed examples, branch guide and more: [Convention Guide](https://dev.to/varbsan/a-simplified-convention-for-naming-branches-and-commits-in-git-il4)

Pull requests that do not follow these rules will be rejected.