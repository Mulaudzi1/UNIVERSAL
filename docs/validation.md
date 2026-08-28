# Validation

`VALIDATE expression` emits a validation result as structured runtime output. The CLI currently renders it as `validation: <message>`.

Future structured declarations will attach code, message, field path, metadata, and localization keys. Validation is not modeled as an exception by default.
