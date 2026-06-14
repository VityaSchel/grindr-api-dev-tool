FROM --platform=linux/amd64 rust:1-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
	libwebkit2gtk-4.1-dev \
	build-essential \
	curl \
	wget \
	file \
	libxdo-dev \
	libssl-dev \
	libayatana-appindicator3-dev \
	librsvg2-dev \
	xdg-utils \
	cmake \
	perl \
	pkg-config \
	libclang-dev \
	git \
	unzip \
	&& rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

RUN mkdir -p /root/.cache/tauri && cd /root/.cache/tauri \
	&& wget -q https://github.com/tauri-apps/binary-releases/releases/download/apprun-old/AppRun-x86_64 \
	&& wget -q https://github.com/tauri-apps/binary-releases/releases/download/linuxdeploy/linuxdeploy-x86_64.AppImage \
	&& wget -q https://raw.githubusercontent.com/tauri-apps/linuxdeploy-plugin-gtk/master/linuxdeploy-plugin-gtk.sh \
	&& wget -q https://raw.githubusercontent.com/tauri-apps/linuxdeploy-plugin-gstreamer/master/linuxdeploy-plugin-gstreamer.sh \
	&& wget -q -O linuxdeploy-plugin-appimage.AppImage https://github.com/linuxdeploy/linuxdeploy-plugin-appimage/releases/download/continuous/linuxdeploy-plugin-appimage-x86_64.AppImage \
	&& chmod +x linuxdeploy* \
	&& for f in linuxdeploy-x86_64.AppImage linuxdeploy-plugin-appimage.AppImage; do \
		dd if=/dev/zero of="$f" bs=1 seek=8 count=3 conv=notrunc status=none; \
	done

# linuxdeploy can't use FUSE inside a container
ENV APPIMAGE_EXTRACT_AND_RUN=1
ENV CARGO_TARGET_DIR=/build/target

WORKDIR /app

CMD bun install --frozen-lockfile && \
	bun --bun tauri build --target x86_64-unknown-linux-gnu && \
	rm -rf dist/linux && mkdir -p dist && \
	cp -r "$CARGO_TARGET_DIR/x86_64-unknown-linux-gnu/release/bundle" dist/linux && \
	rm -rf dist/linux/appimage/*.AppDir
