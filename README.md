# ghostd

temporary shared clipboard server and UI, intended for (W)LAN use

## Goals
- [ ] allow user to copy/paste text (and files?) between devices
- [ ] store minimal data and only temporarily
- [ ] make lightweight and available as service (systemd)
- [ ] self-host and make available only on local network (not implemented or enforced in this repo)

## Usage
`cargo run`: starts a web server at localhost:4321

## systemd
```
[Unit]
Description=ghostd daemon

[Service]
Type=simple
ExecStart=/path/to/ghostd

[Install]
WantedBy=multi-user.target
```
