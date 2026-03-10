docs-serve:
	mkdocs serve

docs-build:
	mkdocs build

docs-build-strict:
	mkdocs build --strict

docs-docker-build:
	docker build -f Dockerfile.docs -t palmscript-docs .

docs-docker-run:
	docker run --rm -p 8080:8080 palmscript-docs

ide-docker-build:
	docker build -f Dockerfile.ide -t palmscript-ide .

ide-docker-run:
	docker run --rm -p 8080:8080 palmscript-ide
