# yong [![Build Status](https://travis-ci.org/shaolang/yong.svg?branch=master)](https://travis-ci.org/shaolang/yong)

yong is a command line tool that eases the switching of development tools
between versions by manipulating the path and environment variables.
The goal is to make this a generic enough tool to replace the plethora
of version managers for different languages.

Non-goal: alias commands to do automatic switching,

## Usage

To add application for yong to manage:

```
$ yong add --app java --home-name JAVA_HOME --bin bin
```

Then to register an installation of the application:

```
$ yong add --app java --version 11 --home-path path/to/java/home
```

To use the application, specify the application followed by the version after
the `use` subcommand:

```
$ yong use java 11
```

To see all the registered applications:

```
$ yong list
```

Copyright © 2020 Shaolang Ai

Distributed under the Apache Software License 2.0
