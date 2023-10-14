diesel migration revert
diesel migration run

mv src/db/models.rs src/db/models_old.rs
mv src/db/schema.rs src/db/schema_old.rs

diesel print-schema > schema.rs
mv schema.rs src/db/schema.rs
diesel_ext --model > src/db/models.rs

rm src/db/schema_old.rs
rm src/db/models_old.rs