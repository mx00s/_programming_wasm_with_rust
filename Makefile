.PHONY: find-questions

find-questions:
	@grep -rn "Q[0-9]\+:" .
