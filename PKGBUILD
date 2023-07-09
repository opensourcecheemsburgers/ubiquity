# Maintainer: sukanka<su975853527 AT gmail dot com>
pkgname=ubiquity
pkgver=0.2.0
pkgrel=2
pkgdesc="An open-source and cross-platform markdown editor written in Rust."
arch=('x86_64')
url="https://github.com/opensourcecheemsburgers/ubiquity"
license=('GPL3')
depends=('webkit2gtk' 'libayatana-appindicator')
makedepends=('npm' 'rustup')
source=("${pkgname}-${pkgver}.tar.gz::${url}/archive/refs/tags/v${pkgver}.tar.gz"
"${pkgname}.desktop"
)

prepare(){
	cd $srcdir/${pkgname}-${pkgver}
    rustup default nightly
    cargo install trunk
    cargo install tauri-cli
    cargo target add wasm32-unknown-unknown

    cd frontend
    npx tailwindcss -i ./css/input.css -o ./css/output.css --watch
}

build(){
	cd $srcdir/${pkgname}-${pkgver}
	# export HOME=$srcdir
	export RUSTFLAGS="-L /usr/lib/quickjs"
	yarn install
	yarn run check
	cargo-tauri build
}
package(){
	cd $srcdir/${pkgname}-${pkgver}
	install -Dm755 src-tauri/target/release/${pkgname} -t ${pkgdir}/usr/bin

	install -d ${pkgdir}/usr/lib/${pkgname}/resources
	ln -sf /etc/clash/Country.mmdb -t ${pkgdir}/usr/lib/${pkgname}/resources

	install -Dm644 src/assets/image/logo.svg ${pkgdir}/usr/share/icons/hicolor/scalable/apps/${pkgname}.svg

	install -Dm644 ${srcdir}/${pkgname}.desktop -t ${pkgdir}/usr/share/applications

}
q