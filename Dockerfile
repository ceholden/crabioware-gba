# Multi-stage build - copy devkitarm install into a container that gives us rust
FROM devkitpro/devkitarm:20200528
FROM rustlang/rust:nightly-slim

COPY --from=0 /opt/devkitpro /opt/devkitpro

ENV DEVKITARM=/opt/devkitpro/devkitARM
ENV PATH=${PATH}:${DEVKITARM}/bin:/opt/devkitpro/tools/bin
ENV DISTDIR=/opt/crabioware/target
ENV APPDIR=/opt/crabioware

# FIXME - install & cleanup apt stuff/etc
RUN DEBIAN_FRONTEND=noninteractive apt-get update \
    && apt-get install --no-install-recommends --yes \
      build-essential \
      curl \
      libssl-dev \
      pkg-config \
    && echo "todo - remove lists"
    # && rm -rf /var/lib/apt/lists/*

# Install rust tools
RUN cargo install cargo-xbuild just \
  && rustup component add rust-src

COPY crabioware ${APPDIR}

WORKDIR ${APPDIR}
# VOLUME ${DISTDIR}
