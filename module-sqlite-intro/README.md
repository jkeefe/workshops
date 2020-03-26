# Sqlite intro

A little workshop to learn and tinker with SQL.

## Let's get some dog data

- Let's look at a [CSV of 81,000 NYC dogs](https://docs.google.com/spreadsheets/d/1dvL1vq4YTG4Y72XHlWvwTg27f1k3UVnXDZfJm5MHef8/edit?usp=sharing)

## Glitch

- Go to https://glitch.com
- Login (or create a new account)
- Click "New Project" ... (we're actually not using most of this, but it installs SQLite)
- Find the "Tools" button at the bottom of the screen. Pick "Terminal"
- At the top pick "Full Screen Terminal"

## Databases!

Note about the `.data` directory: This is a great place to store data. It doesn't copy to other people when they remix your project. BUT for this experiment, it can be a pain because if you lose connectivity, you bail out of that directory ... and have to go back into it.  Change into the `.data` directory by typing `cd .data`

- To get the CSV, type this `wget "http://media.johnkeefe.net/class-modules/nyc_dogs_2012.csv"`
- To start the database, type `sqlite3 dogs.db`
- To switch into CSV mode type `.mode csv`
- To import the CSV into our database, type `.import nyc_dogs_2012.csv doginfo`
- Note that ^ that line created a "doginfo" table within our database
- Check out if everything worked by typing: `.schema doginfo`
- Jump back out of CSV mode: `.mode columns`
- Turn on headers: `.headers on`

### A little bit of SQL

- Try `SELECT * FROM doginfo WHERE dog_name='Spot';` (note the `;` at the end of a SQL command)
- Try `SELECT * FROM doginfo WHERE dog_name='spot';`
- Try `SELECT * FROM doginfo WHERE dog_name LIKE 'spot';`
- Try `SELECT * FROM doginfo LIMIT 10;` 

- Try `SELECT dog_name FROM doginfo;`
- Try `SELECT dog_name, zip_code FROM doginfo;`
- Try `SELECT gender, count(gender) FROM doginfo GROUP BY gender;`
- Try `SELECT dog_name, count(dog_name) FROM doginfo`

- Try `SELECT dog_name, count(dog_name) FROM doginfo GROUP BY dog_name;`
- Try `SELECT dog_name, count(dog_name) FROM doginfo GROUP BY dog_name COLLATE NOCASE;`
- Try `SELECT dog_name, count(dog_name) FROM doginfo GROUP BY dog_name COLLATE NOCASE ORDER BY count(dog_name);`
- Try `SELECT dog_name, count(dog_name) FROM doginfo GROUP BY dog_name COLLATE NOCASE ORDER BY count(dog_name) DESC LIMIT 10;`
- Try `SELECT dog_name, count(dog_name) FROM doginfo WHERE dog_name LIKE "bella" GROUP BY dog_name COLLATE NOCASE;`

### Output to CSV

```sql
sqlite> .mode csv
sqlite> .headers on
sqlite> .output topdogs.csv
sqlite> SELECT dog_name, count(dog_name) FROM doginfo GROUP BY dog_name COLLATE NOCASE ORDER BY count(dog_name) DESC;
sqlite> .stdout
sqlite> .mode columns
```


