# webdiff

Generate JSON events based on changes in web targets.

- Define watch targets in a config file.
- Provide CSS selector to check specific content.

_Note: mostly an excuse to write a little rust._

## Example

```
{
  "targets": [
    {
      "uri": "https://news.ycombinator.com",
      "selector": "a.storylink",
      "interval": 3
    },
    {
      "uri": "https://reddit.com",
      "selector": "h3._eYtD2XCVieq6emjKBH3m",
      "interval": 2
    }
  ]
}
```

## Later

- Notify targets to send JSON events to.
- Multiple selectors?
