# Ruscalimat

🧃 Buy delicious drinks at your company with the ruscalimat a perfect app made by me (and others)

Now with rust

## More docs available in [backend/README.md](./backend/README.md)


## Dev

There's a docker compose in the project directory for the postgres that is required for compiling & running.

### For auth:

Go to http://localhost:8180/admin/master/console/#/ruscalimat/users, add a new user and credentials, then go to
http://localhost:8180/realms/ruscalimat/account/#/ and log in with dev tools open. You can get the bearer jwt
by looking for a call to the token endpoint, it's inside the "access_token" field.
