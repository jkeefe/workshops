+ post results
$ POST YOUR_AIRTABLE_URL_GOES_HERE {"headers": {"Authorization": "Bearer YOUR_AIRTABLE_API_KEY_GOES_HERE", "Content-Type": "application/json"}, "body": {"fields": {"User":"<get _platformId>", "Dogs": "<get dogvar>", "Cats": "<get catvar>" }}}
* ${{__status}} != 200 => ${{error.message}}
- See you soon!