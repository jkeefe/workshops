# Build a question-and-answer bot: The dog-cat bot

```
+ hi
- Do you like dogs?

+ *
- You said: <star>
```

```
+ hi
- Do you like dogs?

+ *
* <star> == y => Dogs are great!
* <star> == n => I don't either.
```

```
+ hi
- Do you like dogs?

+ *
% do you like dogs
* <star> == y => Do you like cats?
* <star> == n => Do you like cats?

+ *
% do you like cats
- Thanks for playing!
```

```
+ hi
- Do you like dogs?

+ *
% do you like dogs
* <star> == y => <set dogvar = yes> Do you like cats?
* <star> == n => <set dogvar = no> Do you like cats?

+ *
% do you like cats
* <star> == y => <set catvar = yes> Thanks for playing!
* <star> == n => <set catvar = no> Thanks for playing!
```

```
+ hi
- Do you like dogs?

+ *
% do you like dogs
* <star> == y => <set dogvar = yes> Do you like cats?
* <star> == n => <set dogvar = no> Do you like cats?

+ *
% do you like cats
* <star> == y => <set catvar = yes> {@ answered <get dogvar> <get catvar>}
* <star> == n => <set catvar = no>  {@ answered <get dogvar> <get catvar>}

+ answered yes yes
- Both can be truly great pets.

+ answered yes no
- Woof! Dogs are great.

+ answered no yes
- Meow! Just remember, dogs have owners. Cats have servants.

+ answered no no
- Maybe you're more of a bird person.
```

