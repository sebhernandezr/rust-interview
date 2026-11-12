# Rust interview: code review exercise

## The situation

A teammate opened a pull request that adds a user registration endpoint to our Axum service. The code is in `src/main.rs`. It doesn't compile.

## Your task

1. **Make it compile.**
2. **Review it as if you had to approve this PR.** What else would you change before it goes to production, and why? Fix what you can in the time; for the rest, explaining what you would do is enough.

The endpoint should:

- accept `POST /register` with a JSON body `{ "email": "...", "password": "..." }`
- create the user, or reject the request if the email is already registered
- send a welcome email to the new user

## Rules

- **You may use any AI tool you like.** Please share your full screen, including the AI chat.
- **Think out loud.** Tell us what you ask the AI and how you decide whether its answer is good.
- There is no single correct solution. We care more about your reasoning than about finishing everything.

## Getting started

```bash
cargo build                # see the compiler error
cargo run                  # starts the server on http://localhost:3000
```

Once it compiles, you can try it from a second terminal:

```bash
curl -i -X POST localhost:3000/register \
  -H 'content-type: application/json' \
  -d '{"email":"ana@example.com","password":"secret123"}'
```

`EmailClient` stands in for a real HTTP client such as `reqwest::Client`. Treat it the same way.

You can add any crate you need with `cargo add <crate>`.
