# SMS Survey with Airtable

## Prerequisites

Do this section only after you've made the [cat-dog SMS bot]('../module-sms-bot').

## Prepare Dexter

Add this to the bottom of your [cat-dog]('../module-sms-bot/cat-dog-basic.rs') script:

```
> object currentDate javascript
  return new Date(Date.now()).toLocaleDateString()
< object
 
+ post_results
$ POST YOUR_AIRTABLE_URL_GOES_HERE {"headers": {"Authorization": "Bearer YOUR_AIRTABLE_API_KEY_GOES_HERE", "Content-Type": "application/json"}, "body": {"fields": {"Time": "<call>currentDate</call>", "User":"<get _platformId>", "Dogs": "<get dogvar>", "Cats": "<get catvar>" }}}
* ${{__status}} != 200 => ${{error.message}}
- See you soon!
```

Also need to adjust the `+ cat answer` code to set the `catvar` variable. See how we make the change below by adding `<set catvar=yes>` and `<set catvar=no>` below:

```
+ cat answer
* <get openended-answer> == yes => <set catvar=yes> {@ combined <get dogvar> yes}
* <get openended-answer> == no => <set catvar=no> {@ combined <get dogvar> no}
```

Also, after all four possible answers, we need to send folks to the `post_results` trigger. So add `{@ post_results}` to the end of all four possible answers, like this:

```
+ combined yes yes
- Both can be truly great pets. {@ post_results}
```

### Prepare Airtable

- Go to airtable.com and sign up for an account.
    - Use an email address you can verify today
- Skip the marketing questions
- Go to your email and verify your account
- Add a base
- Start from scratch
- Call it "My Events"
- Pick a color and an icon
- Click on the icon once it's set
- Clear all of the boxes that come up
- Edit this so that you have 
    - Time, User, Dogs, Cats
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


