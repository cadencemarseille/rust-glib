.PHONY: install clean doc

install:
	rustpkg install glib

clean:
	$(RM) -r .rust bin build lib

doc:
	test -d doc || mkdir doc
	rustdoc --output doc html src/glib/mod.rs
