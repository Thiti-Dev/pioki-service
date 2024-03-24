-- This file should undo anything in `up.sql`
-- DROP INDEX IF EXISTS idx_unique_pioki_id_with_post_id_and_pass_along_at_is_null;
-- DROP INDEX IF EXISTS idx_unique_pioki_id_with_post_id_and_pass_along_at_is_not_null;

DROP INDEX IF EXISTS idx_unique_pioki_id_with_post_id;
DROP TABLE IF EXISTS post_keepers;