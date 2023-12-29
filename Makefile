generate-entities:
	sea-orm-cli generate entity \
    -u sqlite:///home/etim/.local/share/org.etim.querry/querry.db \
    -o src/entities
