# Discord bot for [Tesseract OCR](https://github.com/tesseract-ocr)

This bot was made with [serenity-rs](https://github.com/serenity-rs/serenity)

For the bot to work, create a file called ".env" and enter the following input:

```
DISCORD_TOKEN = \\bot token goes here

PREFIX = \\whatever you want your prefix to be
```

Available commands (so far):

- `read`: reads the **attached** image file. Tries to find text in the image and gives them back to the asker

