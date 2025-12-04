
echo "Archivo de configuracion General: /etc/NetHound.toml"
cp ../default-archives/NetHound.toml /etc/


echo "Carpeta de datos: /var/lib/NetHound/"
mkdir /var/lib/NetHound
echo "Base de datos (usuarios): /var/lib/NetHound/auth_accounts.db"
touch /var/lib/NetHound/auth_accounts.db

