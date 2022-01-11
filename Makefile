# install and uninstall, pvc and pv will keep
install:
	helm upgrade -i mysql --set auth.rootPassword=root,auth.database=alfred,primary.persistence.size=30Gi,image.tag=5.7 bitnami/mysql
uninstall:
	helm uninstall mysql
test:
	cargo run -- --pvc data-mysql-0
build-linux:
	cargo build --release --target=x86_64-unknown-linux-gnu
build-macos:
	cargo build --release --target=x86_64-apple-darwin
build-windows:
	cargo build --release --target=x86_64-pc-windows-gnu