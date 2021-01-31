# Architectural Decisions

## CI Automation

The service will be built using Docker.

TODO

## Framework

As of December of 2020, the most stable, fast and popular Rust web framework is Actix Web. It seems to be the best suitable option.

## Logging

Tracing is a framework created by the Tokio (Async Runtime) team to deal with logs. It looks excellent for async apps.

## Monitoring

TODO

## Configuration

Initially, the configuration may be obtained just from env vars. We may use `dotenv` (to load `.env` files in development environment) and `envy` (to deserialize the env vars into a struct).

## Authentication

TODO

## Authorization

TODO

## Security

TODO

## Workers

TODO

## Notifications (e.g. SMS, email...)

TODO
