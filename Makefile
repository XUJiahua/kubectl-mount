# install and uninstall, pvc and pv will keep
install:
	helm upgrade -i mysql --set auth.rootPassword=root,auth.database=alfred,primary.persistence.size=30Gi,image.tag=5.7 bitnami/mysql
uninstall:
	helm uninstall mysql
test:
	cargo run -- --pvc data-mysql-0