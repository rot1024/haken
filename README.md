# haken

haken ("覇権") - Anime selector and recording rule registerer

<p align="center"><img src="https://raw.githubusercontent.com/rot1024/haken/master/doc/screenshot.png" width="600" /></p>

## Supported Application

### Provider

 - [あにめも！](http://animemo.jp/) (`animemo`)

### Recorder

 - [EPGStation](https://github.com/l3tnun/EPGStation) (`epgstation`)
   - `epgstation_endpoint`: EPGStation API endpoint URL
 - ~~[Chiachu](https://chinachu.moe)~~ (Not supported yet. Your pull request welcome)

## Getting Started

Install haken:

```
curl -L https://github.com/rot1024/haken/releases/download/v0.1.1/haken_0.1.1_`uname -s`_`uname -m` > /usr/local/bin/haken && chmod +x /usr/local/bin/haken
```

Put a config file in `$HOME/.config/haken`, `$HOME/.haken`, `/etc/haken`, or working directory. YAML(`config.yml`), JSON(`config.json`), and TOML(`config.toml`) are supported.

```yml
provider: animemo
recorder: epgstation
epgstation_endpoint: "http://hogehoge:888/api"
```

Then run:

```sh
haken
```
