<div align="center">
  <p>
    <a href="https://hub.docker.com/r/bricksoft/tikkitokki-dc" target="_blank">
      <img alt="Docker Hub Status" src="https://img.shields.io/docker/pulls/bricksoft/tikkitokki-dc?logo=docker&style=flat-square"></a>
    <a href="https://github.com/peanutbother/tikkitokki/releases/latest" target="_blank">
      <img alt="Github Release" src="https://img.shields.io/github/v/release/peanutbother/tikkitokki?logo=github&style=flat-square"></a>
    <a href="https://github.com/peanutbother/tikkitokki/stargazers" target="_blank"><img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/peanutbother/tikkitokki?logo=github&logoColor=white&style=flat-square"></a>
    <br />
    <a href="https://github.com/peanutbother/tikkitokki/actions" target="_blank">
      <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/peanutbother/tikkitokki/test.yml?branch=main&label=Tests&logo=github&style=flat-square"></a>
    <a href="https://app.codecov.io/gh/peanutbother/tikkitokki" target="_blank">
      <img alt="Codecov branch" src="https://img.shields.io/codecov/c/github/peanutbother/tikkitokki/main?logo=codecov&logoColor=white&style=flat-square"></a>
    <a href="https://rust-lang.org/" target="_blank">
      <img alt="rust-edition" src="https://img.shields.io/badge/rust%20edition-2018-blue?logo=rust&style=flat-square"></a>
    <a href="https://docs.rs/poise/latest/poise/" target="_blank">
      <img alt="rust discord library (poise)" src="https://img.shields.io/crates/v/poise?label=poise&logo=discord&logoColor=white&style=flat-square"></a>
    <a href="https://discord.gg/HeaQ7wxDyj" target="_blank">
      <img alt="Discord" src="https://img.shields.io/discord/995301719711957072?logo=discord&logoColor=white&style=flat-square"></a>
  </p>
</div>

# TikkiTokki Discord Bot

## About

This discord bot resolves urls to tiktok videos to an embeddable link to the direct video.

By default the bot will try to match as many urls as possible while returning a unified list of resolved urls without duplicates.

## Setup

To get started invite the bot and run the setup with `/watch` to watch a voice channel and select the matching role to manage.
You can `/unwatch` this channel / role later.

To make this bot active you also need to `/activate` it in order for it to actually watch the channels you set up.
You can `/deactivate` the bot later if you pause it, or kick it if you no longer wish to use it.

## Development / Deployment

This bot is running inside docker and assumes certain environment variables to be set to function properly.

- TZ: defaults to `Etc/Utc`, a timezone to create proper timestamps in logs and some discord api
- RUST_LOG: optinal - overrided to `error,tikkitokki=info` in production, allows you to set more verbose logging if needed
- DISCORD_TOKEN: required, your discord api token

To deploy the container either pull `bricksoft/tikkitokki-dc` or build it locally by `docker build .` and then run it:
> docker run -d -v PATH_TO_YOUR_DATA:/data -e DISCORD_TOKEN=YOUR_TOKEN_HERE bricksoft/tikkitokki-dc:latest
