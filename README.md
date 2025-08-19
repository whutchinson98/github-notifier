# Github Notifier

# Setup
```bash
cargo install github-notifier
```

# Configuration
Create a config.toml file in your home directory under `~/.config/github-notifier/config.toml`

```toml
github_username = "your-github-username"
github_token = "your-github-token"
```

# Usage

If the exit code is **2**, then you have notifications.
If the exit code is **0**, then you do not have notifications.
If the exit code is **1**, then there was an error.

## Example

Here is an example shell script I use as part of my waybar:

```sh
#!/bin/sh

github-notifier
exit_code=$?

if [ $exit_code -eq 2 ]; then
    echo '{"text": "", "class": "notification", "tooltip": "You have GitHub notifications"}'
else
    echo '{"text": "", "class": "none", "tooltip": "No notifications"}'
fi
```

## Debugging

To run with tracing, run `RUST_LOG=trace github-notifier`
