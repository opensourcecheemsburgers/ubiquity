# Maintainer: Stephen Power <simpilldev@gmail.com>

pkgname=ubiquity
pkgver=0.4.0
pkgrel=4
pkgdesc="An open-source, cross-platform markdown editor written in Rust using Tauri, Yew, Tailwind and DaisyUI."
arch=('x86_64')
url="https://github.com/opensourcecheemsburgers/ubiquity"
license=('GPL3')
depends=('webkit2gtk' 'libayatana-appindicator')
makedepends=('npm' 'rustup')
source=("${pkgname}-${pkgver}.tar.gz::${url}/archive/refs/tags/v${pkgver}.tar.gz"
"${pkgname}.desktop::${url}/${pkgname}.desktop"
)
sha256sums=(
	'f9dc9b4c514bc9346409ed4bf3bf8c234fd06f84f29c5e9459a7625fa1d427ff'
	'80294b66c744e6da117d431a80f6332fde89b7bdc7b33442c6e76e8b76c5b94d'
)

build(){
	cd $srcdir/${pkgname}-${pkgver}
	rustup component add --toolchain nightly rust-src
    cargo install trunk
    cargo install tauri-cli
    cargo target add wasm32-unknown-unknown
    cd frontend
    npx tailwindcss -i ./css/input.css -o ./css/output.css --watch
	cd ..
	cargo tauri build --target x86_64-unknown-linux-gnu -- -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
}
package(){
	cd $srcdir/${pkgname}-${pkgver}
	install -Dm755 src-tauri/target/release/x86_64-unknown-linux-gnu/${pkgname} -t ${pkgdir}/usr/bin
	install -Dm644 src-tauri/icons/${pkgname}.svg ${pkgdir}/usr/share/icons/hicolor/scalable/apps/${pkgname}.svg
	install -Dm644 ${srcdir}/${pkgname}.desktop -t ${pkgdir}/usr/share/applications
}