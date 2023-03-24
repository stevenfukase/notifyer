From cynic:

It's in a separate crate because the GitHub schema is quite large and defining
it in it's own crate means we don't have to recompile it as often.

See: https://github.com/obmarg/cynic/tree/eae3220afc5935aa982927f57a7b93c2f18a9c3f/schemas/github