# Entities

Entities are nominal structured types. A property may be optional with `?`.

```universal
ENTITY Employee
    name: Text
    scorecard: Scorecard?
END
```

V0.1 treats relationships as typed properties. Future explicit declarations such as `Employee BELONGS TO Department` should lower to relationship metadata plus typed properties, rather than creating a second incompatible model.
