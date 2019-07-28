#!/bin/sh

rustdoc slides.md -o . --html-in-header=resources/header.inc.html --markdown-no-toc
