# trixie, not bookworm: the MSVC STL headers xwin downloads require clang 19+
# (bookworm ships 14).
FROM rust:1-trixie

RUN apt-get update && apt-get install -y --no-install-recommends \
	clang \
	llvm \
	lld \
	nsis \
	cmake \
	ninja-build \
	nasm \
	perl \
	pkg-config \
	libclang-dev \
	git \
	curl \
	wget \
	file \
	unzip \
	&& rm -rf /var/lib/apt/lists/*

RUN ln -s "$(command -v clang)" /usr/local/bin/clang-cl

RUN rustup target add x86_64-pc-windows-msvc
RUN cargo install --locked cargo-xwin

RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

ENV CARGO_TARGET_DIR=/build/target
ENV XWIN_CACHE_DIR=/build/xwin

WORKDIR /app

CMD bun install --frozen-lockfile && \
	bun --bun tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc --bundles nsis && \
	rm -rf dist/windows && mkdir -p dist && \
	cp -r "$CARGO_TARGET_DIR/x86_64-pc-windows-msvc/release/bundle" dist/windows
