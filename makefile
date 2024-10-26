.PHONY: release
release:
	@if [ "$(tag)" = "" ]; then \
		echo "Error: Please provide a tag argument using 'make release tag=v1.0.0'"; \
		exit 1; \
	fi
	git push
	git tag $(tag)
	git push origin $(tag)
