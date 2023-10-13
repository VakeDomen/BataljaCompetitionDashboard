diesel migration revert
diesel migration run

mv src/database/models.rs src/database/models_old.rs
mv src/database/schema.rs src/database/schema_old.rs

diesel print-schema > schema.rs
mv schema.rs src/database/schema.rs
diesel_ext --model > src/database/models.rs

rm src/database/schema_old.rs
rm src/database/models_old.rs