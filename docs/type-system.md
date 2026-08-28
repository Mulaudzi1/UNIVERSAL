# Type System

UNIVERSAL is designed as a statically checked language. V0.1 implements the first nominal entity/type layer.

Initial type vocabulary: `Text`, `Number`, `Decimal`, `Boolean`, `Date`, `Time`, `DateTime`, `Money`, `List`, `Map`, nominal `Entity`, `Optional`, `Result`, and `Error`.

Only Text/Number/Decimal/Boolean/entity/optional behavior is executable in the bootstrap interpreter. The remaining names are architecture commitments, not claims of complete runtime support.

`Money` will use decimal/integer minor-unit arithmetic plus ISO-style currency identity; floating point will not be used for money. Cross-currency arithmetic will require an explicit conversion operation.

V0.1 avoids general-purpose generics. `List<T>`, `Map<K,V>`, and `Result<T,E>` become surface grammar only after the parser/type-checker design for generic types is agreed.
