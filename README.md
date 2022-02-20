# posts api

classical API for posts CRUD written on rust (with rocket and diesel)

features:
* auth (hardcoded)
* aggregating stats
* processing stats in time interval

For launching this copy `Rocket.toml.tmpl` to `Rocket.toml` and insert there your database URI

Then apply migrations `diesel migration run`
And start app `cargo run`