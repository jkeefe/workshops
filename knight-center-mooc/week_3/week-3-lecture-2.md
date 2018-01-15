# SMS to AirTable


### Prepare Airtable

- Go to airtable.com and sign up for an account.
    - Use an email address you can verify today
- Skip the marketing questions
- Go to your email and verify your account
- Add a base
- Start from scratch
- Call it "Pet Survey"
- Pick a color and an icon
- Click on the icon once it's set
- Clear all of the boxes that come up
- Edit this so that you have 
    - User, Dogs, Cats
    - Make them all "Single line of Text"
- Open a new tab
- Go to https://airtable.com/api
- Find your new "base" and click it
- OK Look carefully at this. There's a URL in the darker part of this page. You need that URL. It'll look something like `https://api.airtable.com/v0/appn5FegAgnXtr0RG/Table%201`
- Paste that into your dexter code where it says `YOUR_AIRTABLE_URL_GOES_HERE`
- Go back to Airtable
- You need an Airtable API key. Get it by clicking the "account" link in the "Authentication section."
- Click "Generate an API Key"
- Copy the key
- Go back to Dexter and paste it where it says `YOUR_AIRTABLE_API_KEY_GOES_HERE`
- Try your script in the mock phone.
- Check your Airtable!


```
+ post results
$ POST YOUR_AIRTABLE_URL_GOES_HERE {"headers": {"Authorization": "Bearer YOUR_AIRTABLE_API_KEY_GOES_HERE", "Content-Type": "application/json"}, "body": {"fields": {"User":"<get _platformId>", "Dogs": "<get dogvar>", "Cats": "<get catvar>" }}}
* ${{__status}} != 200 => ${{error.message}}
- See you soon!
```

Also, after all four possible answers, we need to send folks to the `post_results` trigger. So add `{@ post_results}` to the end of all four possible answers, like this: