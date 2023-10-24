diesel migration revert
diesel migration run

mv src/db/schema.rs src/db/schema_old.rs

diesel print-schema > schema.rs
mv schema.rs src/db/schema.rs

rm src/db/schema_old.rs