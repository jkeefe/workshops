+ hi
- Do you like dogs?

+ *
% do you like dogs
* <star> == y => <set dogvar=yes> Do you like cats?
* <star> == n => <set dogvar=no> Do you like cats?

+ *
% do you like cats
* <star> == y => <set catvar=yes> {@ answered <get dogvar> <get catvar>}
* <star> == n => <set catvar=no> {@ answered <get dogvar> <get catvar>}

+ answered yes yes
- Both can be great pets!

+ answered yes no
- Woof! Dogs are the best.

+ answered no yes
- Meow! Just remember, dogs have owners. Cats have servants.

+ answered no no
- Maybe you're more of a bird person.