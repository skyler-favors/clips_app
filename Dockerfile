FROM rustlang/rust:nightly

ARG PORT
ARG DATABASE_URL

ENV ADDRESS=0.0.0.0

WORKDIR /app
COPY . .

RUN mkdir -p clips
RUN cargo build --release

CMD PORT=$PORT DATABASE_URL=$DATABASE_URL ./target/release/clips_app
